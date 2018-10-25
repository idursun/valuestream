use std::ops::{Div, Mul};

#[derive(Debug)]
struct DataPoint<T>(u64, T);

#[derive(Debug)]
struct ValueStream<T>(Vec<DataPoint<T>>);

impl<T> ValueStream<T> {
    fn new() -> ValueStream<T> {
        ValueStream(Vec::new())
    }

    fn add(self, date: u64, value: T) -> Self {
        let mut ds = self.0;
        ds.push(DataPoint(date, value));
        ValueStream(ds)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T> Div<T> for ValueStream<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = ValueStream<T>;

    fn div(self, rhs: T) -> Self::Output {
        ValueStream(
            self.0
                .into_iter()
                .map(|dp| DataPoint(dp.0, dp.1 / rhs))
                .collect::<Vec<_>>(),
        )
    }
}

impl<T> Mul<T> for ValueStream<T>
where
    T: Mul<Output = T> + Copy,
{
    type Output = ValueStream<T>;

    fn mul(self, rhs: T) -> Self::Output {
        ValueStream(
            self.0
                .into_iter()
                .map(|dp| DataPoint(dp.0, dp.1 * rhs))
                .collect::<Vec<_>>(),
        )
    }
}

fn main() {
    println!(
        "divided   : {:?}",
        ValueStream::new().add(0, 10).add(1, 20) / 2
    );
    println!(
        "multiplied: {:?}",
        ValueStream::new().add(0, 10).add(1, 20) * 2
    );
}
