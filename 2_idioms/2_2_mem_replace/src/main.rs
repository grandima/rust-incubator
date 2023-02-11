use std::{mem::{replace, swap}, fmt::{Display, Debug}};

fn main() {
    let mut s = Solver {
        expected: Trinity { a: 1, b: 2, c: 3 },
        unsolved: vec![
            Trinity { a: 1, b: 2, c: 3 },
            Trinity { a: 2, b: 1, c: 3 },
            Trinity { a: 2, b: 3, c: 1 },
            Trinity { a: 3, b: 1, c: 2 },
        ],
    };
    s.resolve();
    println!("{:?}", s);
}

#[derive(Debug, PartialEq)]
struct Trinity<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Trinity<T> {
    fn rotate(&mut self) {
        swap(&mut self.a, &mut self.b);
        swap(&mut self.b, &mut self.c);
    }
}

#[derive(Debug)]
struct Solver<T> {
    expected: Trinity<T>,
    unsolved: Vec<Trinity<T>>,
}

impl<T: PartialEq> Solver<T> {
    fn resolve(&mut self) {
        'l: for t in self.unsolved.iter_mut() {
            for _ in 0..3 {
                if *t == self.expected {
                    continue 'l;
                }
                t.rotate();
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use std::any::Any;

    use super::*;
    #[test]
    fn test_trinity() {
        let mut s = Solver {
            expected: Trinity { a: 1, b: 2, c: 3 },
            unsolved: vec![
                Trinity { a: 1, b: 2, c: 3 },
                Trinity { a: 2, b: 1, c: 3 },
                Trinity { a: 2, b: 3, c: 1 },
                Trinity { a: 3, b: 1, c: 2 },
            ],
        };
        s.resolve();
        let expected = vec![
            Trinity { a: 1, b: 2, c: 3 },
            Trinity { a: 2, b: 1, c: 3 },
            Trinity { a: 1, b: 2, c: 3 },
            Trinity { a: 1, b: 2, c: 3 },
        ];
        assert_eq!(s.unsolved, expected);
    }
}