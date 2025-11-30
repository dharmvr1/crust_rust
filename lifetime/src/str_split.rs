// use std::ops::Deref;

pub struct StrSplit<'a> {
    haystack: &'a str,
    delimiter: &'a str,
}

pub struct MyPeekable<I>
where
    I: Iterator,
{
    iter: I,
    cached: Option<I::Item>,
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        StrSplit {
            haystack,
            delimiter,
        }
    }
    pub fn peek(&self) -> Option<&'a str> {
        if let Some(next_delim) = self.haystack.find(self.delimiter) {
            let result = &self.haystack[..next_delim];
            Some(result)
        } else if !self.haystack.is_empty() {
            return Some(self.haystack);
        } else {
            None
        }
    }
}
impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delimiter) = self.haystack.find(self.delimiter) {
            let remainder = &self.haystack[..next_delimiter];
            self.haystack = &self.haystack[(next_delimiter + self.delimiter.len())..];
            let result = Some(remainder);

            return result;
        } else if !self.haystack.is_empty() {
            let result = self.haystack;
            self.haystack = "";
            return Some(result);
        } else {
            None
        }
    }
}

impl<I: Iterator> MyPeekable<I> {
    pub fn new(iter: I) -> Self {
        MyPeekable { iter, cached: None }
    }

    pub fn peek(&mut self) -> Option<&I::Item> {
        if self.cached.is_none() {
            self.cached = self.iter.next()
        }
        let result = self.cached.as_ref();
        result
    }
}

impl<I: Iterator> Iterator for MyPeekable<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cached.is_some() {
            return self.cached.take();
        }
        self.iter.next()
    }
}
impl<'a> DoubleEndedIterator for StrSplit<'a> {
    
    fn next_back(&mut self) -> Option<Self::Item> {
            if let Some(next_delim)= self.haystack.rfind(self.delimiter) {
                let result = &self.haystack[next_delim..];
                self.haystack = self.haystack[..]
                return  Some(result);
            }
            else {
                None
            }
    }
}

#[test]
fn next() {
    let mut list: Vec<_> = StrSplit::new("a b c  f d", " ").collect();

    assert_eq!(["a", "b", "c", "", "f", "d"], *list);
    let mut list2 = StrSplit::new("a b c  f d", " ");

    assert_eq!(Some("a"), list2.peek());
    assert_eq!(Some("a"), list2.peek());
}

#[test]
fn test_peek() {
    let list2 = StrSplit::new("a b c  f d", " ");

    let mut mypeek = MyPeekable::new(list2);

    assert_eq!(Some(&"a"), mypeek.peek());
    assert_eq!(Some("a"), mypeek.next());
    assert_eq!(Some(&"b"), mypeek.peek());
}

// pub struct StrSplit<'haystack,'delimiter> {
//     remainder: Option<&'haystack str>,
//     delimiter: &'delimiter str,
// }

// impl<'haystack,'delimiter> StrSplit<'haystack,'delimiter> {
//     pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
//         Self {
//             remainder: Some(haystack),
//             delimiter: delimiter,
//         }
//     }
// }

// impl<'haystack,'delimiter> Iterator for StrSplit<'haystack,'delimiter> {
//     type Item = &'haystack str;
//     fn next(&mut self) -> Option<Self::Item> {
//         if let Some(ref mut remainder) = self.remainder {
//             if let Some(next_delim) = remainder.find(self.delimiter) {
//                 println!("{}", next_delim);
//                 let until_delim = &remainder[..next_delim];
//                 *remainder = &remainder[(next_delim + self.delimiter.len())..];
//                 Some(until_delim)
//             } else {
//                 self.remainder.take()
//             }
//         } else {
//             None
//         }
//     }
// }

// fn until_char(s: &str, c: char) -> &'_ str {
//     let delim = format!("{}", c);
//     StrSplit::new(s, &delim)
//         .next()
//         .expect("StrSplit always gives atleast one result ")
// }

// #[test]
// fn it_works() {
//     let haystack = "a b c   d ";
//     let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
//     assert_eq!(letters, vec!["a", "b", "c", "d"])
// }
