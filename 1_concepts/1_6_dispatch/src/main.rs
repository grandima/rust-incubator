use crate::storage_traits::*;
use std::collections::HashMap;
mod storage_traits;

fn main() {
    let hm = HashMap::new();
    let storage = StorageImpl(hm);
    let repo = UserRepositoryDyn::new(Box::new(storage));
    repo.get(&1);
    // let repo: UserRepository<u32> = UserRepository{};
    println!("Imple2ment me!");
}
