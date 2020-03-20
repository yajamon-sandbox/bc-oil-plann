use std::fmt;

#[derive(Debug, Clone)]
pub struct Buckets(pub i64);
impl fmt::Display for Buckets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mb: MilliBuckets = self.clone().into();
        mb.fmt(f)
    }
}

#[derive(Debug)]
pub struct MilliBuckets(pub i64);
pub type MB = MilliBuckets;
impl fmt::Display for MilliBuckets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let MilliBuckets(value) = self;
        write!(f, "{} mB", value)
    }
}

impl From<Buckets> for MilliBuckets {
    fn from(buckets: Buckets) -> Self {
        let Buckets(value) = buckets;
        MilliBuckets(value * 1000)
    }
}
