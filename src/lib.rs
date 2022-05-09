//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
// str and String comparison
// String -> &str (cheap --> AsRef<str>)
// str -> String (expensive --> From<str>, memcpy)



#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,

}

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}
                                                 //don't care about the delimiter lifetime
impl<'haystack> Iterator for StrSplit<'haystack, '_>
    // where 'delimiter: 'haystack //lifetime bound, delimiter must be in haystack and lives at least as long as haystack
{
    type Item = &'haystack str; //return type of the iterator with 'haystack lifetime
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder?;
        match remainder.find(self.delimiter) {
            Some(index) => {
                let (left, right) = remainder.split_at(index);
                self.remainder = Some(right.trim_start_matches(self.delimiter));
                Some(left)
            }
            None => {
                // self.remainder = None;
                // Some(remainder)
                self.remainder.take()
            }
        }
    }
}



// fn until_char(haystack: &str, delimiter: char) -> &str {
//     match haystack.find(delimiter) {
//         Some(index) => &haystack[..index],
//         None => haystack,
//     }
// }
fn until_char(haystack: &str, delimiter: char) -> &str {
    let delim = format!("{}", delimiter); //lifetime is in the scope of this function

    //The compiler in "new" choose the shorter lifetime between the two, that's delim lifetime.
    //so, haystack hast 'delim lifetime.
    //Cannot return the value, delim is gonna be deallocated after the function ends.
    //We need to define two separate lifetime, one for the haystack and one for the delimiter, that way,
    //haystack has his own lifetime, and delim has his own lifetime. (haystack/and his lifetime/ is returned)

    StrSplit::new(haystack, &delim).next().expect("Always at least one result")
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


