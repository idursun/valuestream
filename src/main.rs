use std::ops::Div;

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

impl<T> Div<T> for DataPoint<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = DataPoint<T>;

    fn div(self, rhs: T) -> Self::Output {
        let n = self.1 / rhs;
        DataPoint(self.0, n)
    }
}

impl<T> Div<T> for ValueStream<T>
where
    T: Div<Output = T> + Copy,
{
    type Output = ValueStream<T>;

    fn div(self, rhs: T) -> Self::Output {
        let divided = self.0.into_iter().map(|dp| dp / rhs).collect::<Vec<_>>();
        ValueStream(divided)
    }
}

fn main() {
    let vs = ValueStream::<i32>::new();
    let vs2 = vs.add(0, 10).add(1, 20);
    let divided = vs2 / 2;
    println!("Hello, world! {:?}", divided.len());
    println!("Hello, world! {:?}", divided);
}
