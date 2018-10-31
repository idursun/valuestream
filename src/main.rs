extern crate valuestreams;

//use valuestreams::valuestream::ValueStream;
use valuestreams::ValueStream;

fn main() {
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
