
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_point_map() {
        let dp = DataPoint(0, Some(2)).map(|x| x / 2);
        assert_eq!(dp.0, 0);
        assert_eq!(dp.1, Some(1));
    }
}
