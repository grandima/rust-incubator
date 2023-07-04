use crate::store::coin::Coin;
use crate::store::VendingMachine;

mod store {
    use std::borrow::Cow;
    use std::collections::btree_map::{Entry, OccupiedEntry};
    use std::collections::{BTreeMap, HashMap};
    use std::collections::hash_map::Entry as HMEntry;
    use crate::store::coin::Coin;

    pub mod coin {
        use std::convert::TryFrom;

        #[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
        pub enum Coin {
            One = 1,
            Two = 2,
            Five = 5,
            Ten = 10,
            Twenty = 20,
            Fifty = 50,
        }

        impl TryFrom<u8> for Coin {
            type Error = CoinError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    1 => Ok(Coin::One),
                    2 => Ok(Coin::Two),
                    5 => Ok(Coin::Five),
                    10 => Ok(Coin::Ten),
                    20 => Ok(Coin::Twenty),
                    50 => Ok(Coin::Fifty),
                    _ => Err(CoinError::NoSuchCoin),
                }
            }
        }

        pub enum CoinError {
            NoSuchCoin,
        }
    }

    #[derive(Clone, Debug, Hash, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Product<'a> {
        price: usize,
        name: Cow<'a, str>,
    }

    #[derive(Clone, Copy, Debug)]
    struct PriceAndAmount {
        price: usize,
        amount: usize,
    }

    #[derive(Clone, Debug)]
    pub struct VendingMachine<'a, State> {
        products: HashMap<Cow<'a, str>, PriceAndAmount>,
        capacity: usize,
        coins: BTreeMap<Coin, usize>,
        state: State,
    }

    impl<'a> VendingMachine<'a, Ready> {
        pub fn new(capacity: usize) -> Self {
            VendingMachine {
                products: HashMap::new(),
                capacity: capacity,
                coins: BTreeMap::new(),
                state: Ready,
            }
        }

        pub fn add_product(
            machine: &mut Self,
            name: impl Into<Cow<'a, str>>,
            price: usize,
        ) -> Result<(), VendingMachineError> {
            if machine.capacity == 0 {
                return Err(VendingMachineError::OutOfFreeSpace);
            }

            let name = name.into();
            let entry = machine
                .products
                .iter_mut()
                .find(|(product_name, _)| **product_name == name);

            if let Some((_, data)) = entry {
                if data.price == price {
                    data.amount += 1;
                    machine.capacity -= 1;
                } else {
                    return Err(VendingMachineError::SameProductNameDifferentPrice);
                }
            } else {
                machine
                    .products
                    .insert(name, PriceAndAmount { price, amount: 1 });
                machine.capacity -= 1;
            }

            Ok(())
        }

        pub fn add_products<C, I>(
            machine: &mut Self,
            products: I,
        ) -> Result<(), VendingMachineError>
        where
            C: Into<Cow<'a, str>>,
            I: IntoIterator<Item = (C, usize)>,
        {
            for (name, price) in products.into_iter() {
                Self::add_product(machine, name, price)?;
            }

            Ok(())
        }

        pub fn add_coin(machine: &mut Self, coin: Coin) {
            machine
                .coins
                .entry(coin)
                .and_modify(|amount| *amount += 1)
                .or_insert(1);
        }

        pub fn add_coins<I: IntoIterator<Item = Coin>>(machine: &mut Self, coins: I) {
            for coin in coins.into_iter() {
                VendingMachine::add_coin(machine, coin);
            }
        }

        pub fn choose(
            mut self,
            product: impl Into<Cow<'a, str>>,
        ) -> Result<VendingMachine<'a, Paying<'a>>, VendingMachineError> {
            let product_name = product.into();
            let product = self.products.entry(product_name);
            let product = match product {
                HMEntry::Occupied(x) => Product {
                    price: x.get().price,
                    name: x.key().clone(),
                },
                HMEntry::Vacant(_) => return Err(VendingMachineError::NoProduct),
            };
            Ok(VendingMachine {
                products: self.products,
                capacity: self.capacity,
                coins: self.coins,
                state: Paying {
                    product,
                    payed: Vec::new(),
                },
            })
        }
    }

    impl<'a> VendingMachine<'a, Paying<'a>> {
        pub fn insert_coin(&mut self, coin: Coin) {
            self.state.payed.push(coin);
        }

        pub fn inset_coins<I: IntoIterator<Item = Coin>>(&mut self, coins: I) {
            for coin in coins.into_iter() {
                Self::insert_coin(self, coin);
            }
        }

        pub fn get_product(
            mut self,
        ) -> Result<(VendingMachine<'a, Ready>, Product<'a>, Vec<Coin>), VendingMachineError>
        {
            let payed = self
                .state
                .payed
                .iter()
                .fold(0, |acc, coin| acc + *coin as usize);

            let rest = payed
                .checked_sub(self.state.product.price)
                .ok_or(VendingMachineError::NotEnoughMoney)?;

            let rest_coins = Self::calc_rest(rest, Cow::Borrowed(&self.coins));

            if let Some(rest_coins) = rest_coins {
                let product_name = self.state.product.name.clone();
                let (_, data) = self
                    .products
                    .iter_mut()
                    .find(|(name, _)| **name == product_name)
                    .unwrap();

                data.amount -= 1;

                if data.amount == 0 {
                    self.products.remove(&*product_name);
                }

                for coin in &rest_coins {
                    Self::decrement_amount(coin, &mut self.coins).unwrap();
                }

                return Ok((
                    VendingMachine {
                        products: self.products,
                        capacity: self.capacity,
                        coins: self.coins,
                        state: Ready,
                    },
                    Product {
                        price: self.state.product.price,
                        name: self.state.product.name,
                    },
                    rest_coins,
                ));
            }

            Err(VendingMachineError::CantGiveRest(self.state.payed))
        }

        fn calc_rest(rest: usize, coins: Cow<BTreeMap<Coin, usize>>) -> Option<Vec<Coin>> {
            Self::calc_rest_internal(rest, coins, 0, Cow::Owned(Vec::new()))
        }

        fn calc_rest_internal(
            rest: usize,
            mut coins: Cow<BTreeMap<Coin, usize>>,
            cur_sum: usize,
            rest_coins: Cow<Vec<Coin>>,
        ) -> Option<Vec<Coin>> {
            if rest == cur_sum {
                return Some(Vec::clone(&rest_coins));
            }

            let mut coins_copy = coins.clone();
            if cur_sum < rest {
                if let Some((coin, amount)) = coins.to_mut().iter_mut().rev().next() {
                    return if *amount == 0 {
                        coins_copy.to_mut().remove(coin);
                        Self::calc_rest_internal(rest, coins_copy, cur_sum, rest_coins)
                    } else {
                        *amount -= 1;

                        let new_sum = cur_sum + *coin as usize;
                        let mut new_rest_coins = rest_coins.clone();
                        new_rest_coins.to_mut().push(*coin);

                        Self::calc_rest_internal(rest, coins.clone(), new_sum, new_rest_coins)
                            .or_else(|| Self::calc_rest_internal(rest, coins, cur_sum, rest_coins))
                    };
                }
            }

            None
        }

        fn decrement_amount<K: Eq + Ord>(
            key: &K,
            map: &mut BTreeMap<K, usize>,
        ) -> Result<(), VendingMachineError> {
            let (_, amount) = map
                .iter_mut()
                .find(|(map_key, _)| **map_key == *key)
                .ok_or(VendingMachineError::NoProduct)?;

            *amount -= 1;

            if *amount == 0 {
                map.remove(key);
            }

            Ok(())
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum VendingMachineError {
        SameProductNameDifferentPrice,
        OutOfFreeSpace,
        NoProduct,
        NotEnoughMoney,
        CantGiveRest(Vec<Coin>),
    }



    #[derive(Debug)]
    pub struct Ready;


    #[derive(Debug, Clone)]
    pub struct Paying<'a> {
        product: Product<'a>,
        payed: Vec<Coin>,
    }



    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn calc_rest() {
            let mut machine_coins = BTreeMap::new();

            [(Coin::Fifty, 2), (Coin::Ten, 1), (Coin::Two, 7)]
                .iter()
                .for_each(|(coin, amount)| {
                    machine_coins.insert(*coin, *amount);
                });

            assert!(VendingMachine::calc_rest(0, Cow::Borrowed(&machine_coins))
                .unwrap()
                .is_empty());

            let rest = VendingMachine::calc_rest(14, Cow::Borrowed(&machine_coins)).unwrap();
            assert_eq!(&rest[..], &[Coin::Ten, Coin::Two, Coin::Two]);

            let rest = VendingMachine::calc_rest(13, Cow::Borrowed(&machine_coins));
            assert_eq!(rest, None);

            VendingMachine::calc_rest(124, Cow::Borrowed(&machine_coins)).unwrap();

            assert!(VendingMachine::calc_rest(125, Cow::Borrowed(&machine_coins)).is_none());
        }

        #[test]
        fn vending_machine() {
            let mut machine = VendingMachine::new(5);

            VendingMachine::add_products(
                &mut machine,
                vec![("Snickers", 5), ("Snickers", 5), ("Mars", 7), ("Bounty", 3)],
            )
            .unwrap();

            assert_eq!(
                VendingMachine::add_product(&mut machine, "Mars", 5),
                Err(VendingMachineError::SameProductNameDifferentPrice)
            );

            VendingMachine::add_product(&mut machine, "Mars", 7).unwrap();

            assert_eq!(
                VendingMachine::add_product(&mut machine, "Snickers", 5),
                Err(VendingMachineError::OutOfFreeSpace)
            );

            VendingMachine::add_coins(&mut machine, vec![Coin::Two, Coin::One, Coin::Two]);

            let mut machine = machine.choose("Snickers").unwrap();

            machine.insert_coin(Coin::Ten);

            let (machine, product, rest) = machine.get_product().unwrap();

            assert_eq!(&rest[..], &[Coin::Two, Coin::Two, Coin::One]);
            assert_eq!(product.name, "Snickers");
            assert_eq!(product.price, 5);
            assert!(machine.coins.is_empty());
        }
    }
}

fn main() {
    let mut machine = VendingMachine::new(5);
    VendingMachine::add_products(
        &mut machine,
        vec![("Snickers", 5), ("Snickers", 5), ("Mars", 7), ("Bounty", 3)],
    )
    .unwrap();
    VendingMachine::add_product(&mut machine, "Mars", 7).unwrap();
    VendingMachine::add_coins(&mut machine, vec![Coin::Two, Coin::One, Coin::Two]);

    let mut machine = machine.choose("Snickers").unwrap();

    machine.insert_coin(Coin::Ten);

    let (machine, product, rest) = machine.get_product().unwrap();

    dbg!(product);
    dbg!(rest);
}
