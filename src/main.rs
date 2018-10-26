use std::ops::{Div, Mul};

#[derive(Debug, Clone)]
struct DataPoint<T>(u64, Option<T>) where T: Clone;

impl<T: Clone> DataPoint<T> {

    fn map<S: Clone, F: Fn(T) -> S>(&self, f: F) -> DataPoint<S> {
        let v = self.1.clone();
        DataPoint(self.0, v.map(f))
    }

}

#[derive(Debug, Clone)]
struct ValueStream<T>(Vec<DataPoint<T>>) where T: Clone;

impl<T: Clone> ValueStream<T> {
    fn new() -> ValueStream<T> {
        ValueStream(Vec::new())
    }

    fn add(self, date: u64, value: T) -> Self {
        let mut ds = self.0;
        ds.push(DataPoint(date, Some(value)));
        ValueStream(ds)
    }

    fn last_value(&self) -> Option<&DataPoint<T>> {
        let len = self.0.len();
        if len > 0 {
            let v = &self.0[len - 1];
            return Some(v);
        }
        None
    }

    fn apply_window(&self, min: u64, max: u64) -> ValueStream<T> {
        ValueStream(
            self.0
                .iter()
                .filter(|&DataPoint(ref key, _)| *key <= max && *key >= min)
                .cloned()
                .collect::<Vec<_>>(),
        )
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T: Clone> From<Vec<(u64, T)>> for ValueStream<T> {
    fn from(data: Vec<(u64, T)>) -> ValueStream<T> {
        let mut vs = ValueStream::new();
        for item in data {
            vs = vs.add(item.0, item.1);
        }
        vs
    }
}

impl<T, S> Div<S> for ValueStream<T>
where
    S: Copy,
    T: Div<S, Output = T> + Clone,
{
    type Output = ValueStream<T>;

    fn div(self, rhs: S) -> Self::Output {
        ValueStream(
            self.0
                .into_iter()
                .map(|DataPoint(key, value)| DataPoint(key, value.map(|x| x / rhs)))
                .collect::<Vec<DataPoint<T>>>(),
        )
    }
}

impl<T, S> Mul<S> for ValueStream<T>
where
    S: Copy,
    T: Mul<S, Output = T> + Copy,
{
    type Output = ValueStream<T>;

    fn mul(self, rhs: S) -> Self::Output {
        ValueStream(
            self.0
                .into_iter()
                .map(|dp| DataPoint(dp.0, dp.1.map(|x| x * rhs)))
                .collect::<Vec<_>>(),
        )
    }
}

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
    println!("window    : {:?}", vs.apply_window(1, 3)) ;
    println!(
        "last_value: {:?}",
        vs.clone().last_value().expect("should have returned")
    );
}
