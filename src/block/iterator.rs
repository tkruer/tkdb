use std::{sync::Arc, usize};
use bytes::Buf;

use crate::key::KeyVec;

pub struct BlockIterator {
    block: Arc<Block>
    key: KeyVec,
    value_range: (usize, usize),
    idx: usize,
    first_key: KeyVec
}
