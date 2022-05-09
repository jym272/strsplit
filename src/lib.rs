//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
// str and String comparison
// String -> &str (cheap --> AsRef<str>)
// str -> String (expensive --> From<str>, memcpy)



#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,

}

trait Delimiter {
    fn find_next(&self, haystack: &str) -> Option<(usize, usize)>;
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}
impl<'haystack, D> Iterator for StrSplit<'haystack, D>
    where D: Delimiter
{
    type Item = &'haystack str; //return type of the iterator with 'haystack lifetime
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;
        match self.delimiter.find_next(remainder) {
            Some((start, end)) => {
                self.remainder = Some(&remainder[end..]);
                Some(&remainder[..start])
            }
            None => {
                self.remainder.take()
            }

        }
    }
}

impl Delimiter for &'_ str {
    fn find_next(&self, haystack: &str) -> Option<(usize, usize)> {
        haystack.find(self).map(|start| (start, start + self.len()))

    }
}
impl Delimiter for char {
    fn find_next(&self, haystack: &str) -> Option<(usize, usize)> {
        haystack.char_indices().find(|(_, c)| *c == *self).map(|(start, _)| (start, start + self.len_utf8()))
    }
}


fn until_char(haystack: &str, delimiter: char) -> &str {
    StrSplit::new(haystack, delimiter).next().expect("Always at least one result")
}




#[test]
fn test_until_char() {
    assert_eq!(until_char("Hello, world!", ','), "Hello");
    assert_eq!(until_char("Hello, world!", '!'), "Hello, world");
    assert_eq!(until_char("Hello, world!", ' '), "Hello,");
    assert_eq!(until_char("Hello, world!", 'x'), "Hello, world!");
}


#[test]
fn it_works() {
    let haystack = "a2 __ b23 __ c__ d  __ 123e__ fg";
    let letters = StrSplit::new(haystack, "__");
    assert_eq!(letters.collect::<Vec<_>>(), vec!["a2 ", " b23 ", " c", " d  ", " 123e", " fg"]);
}

#[test]
fn it_works2() {
    let haystack = "a b c d e f g";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e", "f", "g"]));
}

#[test]
fn bug_at_the_end() {
    let haystack = "a b c d e f g ";
    let letters = StrSplit::new(haystack, " ");
    assert_eq!(letters.collect::<Vec<_>>(), vec!["a", "b", "c", "d", "e", "f", "g", ""]);
}
#[test]
fn empty_string() {
    let haystack = "";
    let letters = StrSplit::new(haystack, "&");
    assert_eq!(letters.collect::<Vec<_>>(), vec![""]);
}
#[test]
fn unknown_delimiter() {
    let haystack = "a b c d e f g";
    let letters = StrSplit::new(haystack, "&");
    assert_eq!(letters.collect::<Vec<_>>(), vec!["a b c d e f g"]);
}


