#[derive(Debug)]
pub struct Buckets(pub i64);

#[derive(Debug)]
pub struct MilliBuckets(pub i64);
pub type MB = MilliBuckets;

impl From<Buckets> for MilliBuckets {
    fn from(buckets: Buckets) -> Self {
        let Buckets(value) = buckets;
        MilliBuckets(value * 1000)
    }
}
