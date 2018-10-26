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

    fn last_value(&self) -> Option<&DataPoint<T>> {
        let len = self.0.len();
        if len > 0 {
            let v = &self.0[len - 1];
            return Some(v);
        }
        None
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T, S> Div<S> for ValueStream<T>
where
    S: Copy,
    T: Div<S, Output = T>,
{
    type Output = ValueStream<T>;

    fn div(self, rhs: S) -> Self::Output {
        ValueStream(
            self.0
                .into_iter()
                .map(|dp| DataPoint(dp.0, dp.1 / rhs))
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
        "last_value: {:?}",
        ValueStream::new().add(0, 10).add(1, 20).last_value()
    );
    println!(
        "multiplied: {:?}",
        ValueStream::new().add(0, 10).add(1, 20) * 2
    );
}
