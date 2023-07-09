use std::collections::BTreeMap;
macro_rules! my_map {
    ( $( $x:expr => $y:expr ),* ) => {
        {
            let mut map = BTreeMap::new();
            $(
                 map.insert($x, $y);
            )*
            map
        }
    };
}

fn main() {
    let v = my_map!("a" => "b", "c"=> "d");
    println!("{:?}", v);
}
