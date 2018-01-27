extern crate qptrie;

use std::str;
use std::ops::{Deref, DerefMut};
use self::qptrie::Trie;

pub struct Dict(Trie<Vec<u8>, Vec<u8>>);

impl Deref for Dict {
    type Target = Trie<Vec<u8>, Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Dict {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Dict {
    fn prefix_match<'a, 'b>(&'a self, query: &'b [u8]) -> Option<(&'b [u8], &'a [u8])> {
        let mut text = query;
        while !text.is_empty() {
            let text_vec = text.to_vec();
            match self.get(&text_vec) {
                Some(target) => return Some((text, target)),
                None => text = &text[..text.len()-1]
            }
        }
        return None
    }
}

impl Dict {
    pub fn new() -> Self {
        Dict(Trie::new())
    }

    pub fn load(&mut self, raw: &str) -> &mut Self{
        for (key, value) in raw.lines()
        .filter_map(|line| {
            let mut cols = line.splitn(2, ' ');
            Some((cols.next()?, cols.next()?))
        }) {
            self.insert(key.to_string().into_bytes(), value.to_string().into_bytes());
        }
        self
    }

    pub fn replace_all(&self, text: &str) -> String {
        let mut output = String::with_capacity(text.len());
        let mut text: &[u8] = text.as_ref();
        while !text.is_empty() {
            match self.prefix_match(&text) {
                Some((prefix, value)) => {
                    output.push_str(str::from_utf8(value).unwrap());
                    text = &text[prefix.len()..];
                },
                None => {
                    let mut chars = str::from_utf8(text).unwrap().chars();
                    output.push(chars.next().unwrap());
                    text = chars.as_str().as_ref();
                }
            }
        }
        output
    }
}


#[test]
fn test_prefix_match() {
    let mut dict = Dict::new();
    dict.load("
A a'
B b'
C c'
ABC abc'
ABCD abcd'
DDD ddd'
BB bb'");
    assert_eq!(Some((b"A".as_ref(), b"a'".as_ref())), dict.prefix_match(b"A"));
    assert_eq!(Some((b"B".as_ref(), b"b'".as_ref())), dict.prefix_match(b"BXX"));
    assert_eq!(Some((b"ABC".as_ref(), b"abc'".as_ref())), dict.prefix_match(b"ABCX"));
    assert_eq!(Some((b"ABCD".as_ref(), b"abcd'".as_ref())), dict.prefix_match(b"ABCDEFG"));
    assert_eq!(None, dict.prefix_match(b"X"));
    assert_eq!(None, dict.prefix_match(b"DD"));
}

#[test]
fn test_dict_simple() {
    let mut dict = Dict::new();
    dict.load("
A a
B b
ABC xxx
'");
    assert_eq!("a", dict.replace_all("A"));
    assert_eq!("ab", dict.replace_all("AB"));
    assert_eq!("xxx", dict.replace_all("ABC"));
    assert_eq!("abxxxa", dict.replace_all("ABABCA"));
    assert_eq!("aXbXab", dict.replace_all("AXBXAB"));
}