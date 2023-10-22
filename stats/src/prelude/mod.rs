pub trait Mean {
    type Output;

    fn mean(&self) -> Self::Output;
}

pub trait Median {
    type Output;

    fn median(&self) -> Self::Output;
}

pub trait Mode {
    type Output;

    fn mode(&self) -> Self::Output;
}

pub trait Variance: Mean {
    fn var(&self) -> Self::Output;
}

pub trait Deviation: Variance {
    fn std(&self) -> Self::Output;
}

pub trait Range {
    type Output;

    fn range(&self) -> Self::Output;
    fn percentile(&self, percent: u32) -> Self::Output;
    fn interquartile_range(&self) -> Self::Output;
}
