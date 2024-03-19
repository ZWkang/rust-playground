
pub enum EntryKind {
  PUT = 1,
  DEL = 2,
}

pub struct Entry {
  key_len: usize,
  value_len: usize,
  key: String,
  value: String,
  kind: EntryKind,
}