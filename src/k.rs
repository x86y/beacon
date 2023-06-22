use ngnk::{K, K0};

pub fn keval(s: &'static str, args: Vec<K>) -> K {
    K0(s, args)
}
