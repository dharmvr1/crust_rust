pub enum Cow<'a, B>
where
    B: 'a + ToOwned,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}

impl<'a, B> Cow<'a, B> {
    pub fn to_mut(&mut self) -> &mut <B as ToOwned>::Owned {
        match *self {
            Cow::Owned(ref mut x) => x,
            Cow::Borrowed(x) => {
                *self = Owned(x.to_owned());

                match *self {
                    Self::Owned(x) => x,
                    Self::Borrowed(_) => unreachable!(),
                }
            }
        }
    }
}


use std::ops::Deref;

#[derive(Debug, Clone)]
pub enum MyCow<'a, T>
where
    T: Clone,
{
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T> MyCow<'a, T>
where
    T: Clone,
{
    /// Creates a new borrowed Cow
    pub fn borrowed(b: &'a T) -> Self {
        MyCow::Borrowed(b)
    }

    /// Creates a new owned Cow
    pub fn owned(o: T) -> Self {
        MyCow::Owned(o)
    }

    /// Clone-on-write: ensures the data is owned and returns a mutable reference
    pub fn to_mut(&mut self) -> &mut T {
        match self {
            MyCow::Borrowed(b) => {
                // Convert borrowed → owned by cloning
                let owned = (*b).clone();
                *self = MyCow::Owned(owned);
                
                // Now guaranteed to be Owned
                match self {
                    MyCow::Owned(ref mut o) => o,
                    _ => unreachable!(),
                }
            }
            MyCow::Owned(ref mut o) => o,
        }
    }
}
