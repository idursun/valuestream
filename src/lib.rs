use std::ops::{Div, Mul};

#[derive(Debug, Clone)]
pub struct DataPoint<T>(pub u64, pub Option<T>)
where
    T: Clone;

impl<T: Clone> DataPoint<T> {
    pub fn map<S: Clone>(&self, f: impl Fn(T) -> S) -> DataPoint<S> {
        let v = self.1.clone();
        DataPoint(self.0, v.map(f))
    }
}

#[derive(Debug, Clone)]
pub struct ValueStream<T>(Vec<DataPoint<T>>)
where
    T: Clone;

impl<T: Clone> ValueStream<T> {
    pub fn new() -> ValueStream<T> {
        ValueStream(Vec::new())
    }

    pub fn add(self, date: u64, value: T) -> Self {
        let mut ds = self.0;
        ds.push(DataPoint(date, Some(value)));
        ValueStream(ds)
    }

    pub fn last_value(&self) -> Option<&DataPoint<T>> {
        let len = self.0.len();
        if len > 0 {
            let v = &self.0[len - 1];
            return Some(v);
        }
        None
    }

    pub fn apply_window(&self, min: u64, max: u64) -> ValueStream<T> {
        ValueStream(
            self.0
                .iter()
                .filter(|&DataPoint(ref key, _)| *key <= max && *key >= min)
                .cloned()
                .collect::<Vec<_>>(),
        )
    }

    pub fn len(&self) -> usize {
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
                .map(|dp| dp.map(|x| x / rhs))
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
                .map(|dp| dp.map(|x| x * rhs))
                .collect::<Vec<_>>(),
        )
    }
}
