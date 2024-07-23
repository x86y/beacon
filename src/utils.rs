use std::{collections::HashMap, time::Duration};

use once_cell::sync::Lazy;

pub fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

pub mod macros {
    macro_rules! bqn386 {
        ($q:expr) => {
            iced::widget::text($q)
                .font(Font::with_name("BQN386 Unicode"))
                .size(14)
                .line_height(iced::widget::text::LineHeight::Absolute(12.into()))
        };
    }
    pub(crate) use bqn386;
}

#[cfg(feature = "k")]
use ngnk::{K, K0};
#[cfg(feature = "k")]
pub fn keval(s: &'static str, args: Vec<K>) -> K {
    K0(s.to_string(), args)
}

use cbqn::{eval, BQNValue};
use serde::{Deserialize, Serialize};
pub static REPL: Lazy<BQNValue> = Lazy::new(|| {
    eval("(•ReBQN{repl⇐\"loose\"})⎊{𝕊: \"Error: \"∾•CurrentError@}")
        .expect("Err on repl construction")
});

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Ty {
    Array,
    Number,
    Character,
    Function,
    Mod1,
    Mod2,
    Namespace,
    Err,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvalCell {
    pub src: String,
    pub res: String,
    pub ty: Ty,
    pub time: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HistoryMap(pub HashMap<usize, HashMap<usize, Vec<EvalCell>>>);
impl HistoryMap {
    pub fn min_tab(&self) -> &usize {
        self.0.keys().min().unwrap_or(&0)
    }
    pub fn max_tab(&self) -> &usize {
        self.0.keys().max().unwrap_or(&0)
    }
    pub fn new() -> HistoryMap {
        let mut h = HashMap::new();
        let mut hi = HashMap::new();
        hi.insert(0, vec![]);
        h.insert(0, hi);
        HistoryMap(h)
    }
}
