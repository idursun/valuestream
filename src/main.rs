extern crate valuestreams;

use valuestreams::{DataPoint, ValueStream};

fn main() {
    let dp = DataPoint(0, Some(1));
    let dp2 = dp.map(|x| x * 2);
    println!("{:?}{:?}", dp, dp2);

    let mut vs = ValueStream::new();
    for i in 1..10 {
        vs = vs.add(i, i);
    }

    println!("divided   : {:?}", vs.clone() / 2);
    println!("multiplied: {:?}", vs.clone() * 2);
    println!("window    : {:?}", vs.apply_window(1, 3));
    println!(
        "last_value: {:?}",
        vs.clone().last_value().expect("should have returned")
    );
}
