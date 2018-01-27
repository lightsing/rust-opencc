extern crate qptrie;

use std::str;
use std::ops::{Deref, DerefMut};
use self::qptrie::Trie;

pub struct Dict(Trie<Vec<u8>, Vec<u8>>);

const RAW_DICT_ST: &'static str = include_str!("st.txt");
const RAW_DICT_TS: &'static str = include_str!("ts.txt");

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

#[test]
fn test_dict_ts() {
    let mut dict = Dict::new();
    dict.load(RAW_DICT_TS);
    let tc = "曾經有一份真誠的愛情放在我面前，我沒有珍惜，等我失去的時候我才\
    後悔莫及。人事間最痛苦的事莫過於此。如果上天能夠給我一個再來一次得機會，\
    我會對那個女孩子說三個字，我愛你。如果非要在這份愛上加個期限，我希望是，\
    一萬年。";
    let sc = "曾经有一份真诚的爱情放在我面前，我没有珍惜，等我失去的时候我才\
    后悔莫及。人事间最痛苦的事莫过于此。如果上天能够给我一个再来一次得机会，\
    我会对那个女孩子说三个字，我爱你。如果非要在这份爱上加个期限，我希望是，\
    一万年。";
    assert_eq!(sc, dict.replace_all(tc));
}

#[test]
fn test_dict_st() {
    let mut dict = Dict::new();
    dict.load(RAW_DICT_ST);
    let sc = "夸夸其谈 夸父逐日
    我干什么不干你事。
    太后的头发很干燥。
    燕燕于飞，差池其羽。之子于归，远送于野。
    请成相，世之殃，愚暗愚暗堕贤良。人主无贤，如瞽无相何伥伥！请布基，慎圣人\
    ，愚而自专事不治。主忌苟胜，群臣莫谏必逢灾。
    曾经有一份真诚的爱情放在我面前，我没有珍惜，等我失去的时候我才后悔莫及。\
    人事间最痛苦的事莫过于此。如果上天能够给我一个再来一次得机会，我会对那个\
    女孩子说三个字，我爱你。如果非要在这份爱上加个期限，我希望是，一万年。
    新的理论被发现了。
    鲶鱼和鲇鱼是一种生物。
    金胄不是金色的甲胄。";
    let tc = "誇誇其談 夸父逐日
    我幹什麼不干你事。
    太后的頭髮很乾燥。
    燕燕于飛，差池其羽。之子于歸，遠送於野。
    請成相，世之殃，愚闇愚闇墮賢良。人主無賢，如瞽無相何倀倀！請布基，慎聖人\
    ，愚而自專事不治。主忌苟勝，羣臣莫諫必逢災。
    曾經有一份真誠的愛情放在我面前，我沒有珍惜，等我失去的時候我才後悔莫及。\
    人事間最痛苦的事莫過於此。如果上天能夠給我一個再來一次得機會，我會對那個\
    女孩子說三個字，我愛你。如果非要在這份愛上加個期限，我希望是，一萬年。
    新的理論被發現了。
    鮎魚和鮎魚是一種生物。
    金胄不是金色的甲冑。";
    assert_eq!(tc, dict.replace_all(sc));
}