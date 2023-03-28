use std::cmp::Ordering::{Equal, Greater, Less};

fn bsearch_range_table(ch: char, r: &[(char, char)]) -> bool {
    r.binary_search_by(|&(lo, hi)| {
        if lo > ch {
            Greater
        } else if hi < ch {
            Less
        } else {
            Equal
        }
    })
    .is_ok()
}

include!(concat!(env!("ENV_UNICODE_DIR"), "/ID_Start.rs"));

pub fn is_ident_start(ch: char) -> bool {
    bsearch_range_table(ch, ID_START)
}

include!(concat!(env!("ENV_UNICODE_DIR"), "/ID_Continue.rs"));

pub fn is_ident_continue(ch: char) -> bool {
    bsearch_range_table(ch, ID_CONTINUE)
}
