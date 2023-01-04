use std::collections::HashSet;

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
#[derive(Clone)]
struct Polyline {
    my_set: HashSet<Point>
}
impl Polyline {
    pub fn new(point: Point) -> Self {
        let mut set = HashSet::new();
        set.insert(point);
        Self { my_set: set }
    }
    pub fn add(&mut self, point: Point) -> bool {
        self.my_set.insert(point)
    }
}
#[test]
fn test_same() {
    let a = Point::default();
    let a_same = a;
    let mut line = Polyline::new(a_same);
    assert!(!line.add(a))
}
#[test]
fn test_different() {
    let a = Point::default();
    let b = Point{x: 2, y :4};
    let mut line = Polyline::new(a);
    assert!(line.add(b))
}
fn main() {
    println!("Implement weeme!");
}
