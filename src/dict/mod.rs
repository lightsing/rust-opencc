extern crate qptrie;

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
        while text.len() > 2 {
            let text_vec = text.to_vec();
            match self.get(&text_vec) {
                Some(target) => return Some((text, target)),
                None => text = &text[..text.len()-2]
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

}