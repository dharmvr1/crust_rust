use std::result;

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    inner: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            inner: None,
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
            if let Some(ref mut inner_iter) = self.inner {
                if let Some(i  ) = inner_iter.next() {
                    return Some(i);
                }
                self.inner = None;
            }
            let next_inner_inter = self.outer.next()?.into_iter();
            self.inner = Some(next_inner_inter);
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

#[test]
fn check_flatten() {
    let itera = std::iter::once(vec!["1"]);

    let list = vec![vec![1, 2, 3], vec![4, 5, 6]].into_iter();
    let mut flatten:Vec<_> = Flatten::new(list).collect();
    // let result = flatten.count();
    // println!("{:?}", result);    
    // assert_eq!(6, result);
assert_eq!(flatten, [1,2,3,4,5,6]);

    // println!("{:?}", flatten.flatten());
}
