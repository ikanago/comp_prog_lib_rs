pub trait Monoid: Clone {
    fn mempty() -> Self;
    fn mappend(&self, other: &Self) -> Self;
}

#[derive(Clone)]
pub struct Max<T>(pub T)
where
    T: Copy + Ord + num::Bounded;

impl<T> Monoid for Max<T>
where
    T: Copy + Ord + num::Bounded,
{
    fn mempty() -> Self {
        Max(T::min_value())
    }

    fn mappend(&self, other: &Self) -> Self {
        Max(self.0.max(other.0))
    }
}

#[derive(Clone)]
pub struct Min<T>(pub T)
where
    T: Copy + Ord + num::Bounded;

impl<T> Monoid for Min<T>
where
    T: Copy + Ord + num::Bounded,
{
    fn mempty() -> Self {
        Min(T::max_value())
    }

    fn mappend(&self, other: &Self) -> Self {
        Min(self.0.min(other.0))
    }
}
