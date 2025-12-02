use std::result;

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    fornt_inner: Option<<O::Item as IntoIterator>::IntoIter>,
    back_inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            fornt_inner: None,
            back_inner: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.fornt_inner {
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.fornt_inner = None;
            }
            if let Some(nex_inner) = self.outer.next() {
                self.fornt_inner = Some(nex_inner.into_iter());
            } else {
                return self.back_inner.as_mut()?.next();
            }
        }
    }
}

pub fn flatten<I: Iterator>(iter: I) -> Flatten<I>
where
    I: Iterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter)
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut inner_iter) = self.back_inner {
                if let Some(i) = inner_iter.next_back() {
                    return Some(i);
                }
                self.back_inner = None;
            }
            if let Some(next_inner_iter) = self.outer.next_back() {
                self.back_inner = Some(next_inner_iter.into_iter());
            } else {
                return self.fornt_inner.as_mut()?.next_back();
            }
        }
    }
}

#[test]
fn check_flatten() {
    let itera = std::iter::once(vec!["1"]);

    let list = vec![vec![1, 2, 3], vec![4, 5, 6]].into_iter();
    let mut flatten: Vec<_> = Flatten::new(list).collect();
    // let result = flatten.count();
    // println!("{:?}", result);
    // assert_eq!(6, result);
    assert_eq!(flatten, [1, 2, 3, 4, 5, 6]);

    // println!("{:?}", flatten.flatten());
}

#[test]
fn test_double_iter() {
    let list = vec![vec![1, 2, 3], vec![4, 5, 6]].into_iter();
    let mut flatten = Flatten::new(list);
    // assert_eq!(flatten.next(), Some(1));
    assert_eq!(flatten.next_back(), Some(6));
    assert_eq!(flatten.next(), Some(1));
    assert_eq!(flatten.next(), Some(2));
    assert_eq!(flatten.next(), Some(3));
    assert_eq!(flatten.next(), Some(4));
    assert_eq!(flatten.next_back(), Some(5));

    // assert_eq!(flatten.next_back(),Some(6));

    // assert_eq!(flatten,[6,5,4,3,2,1]);
}
