use rusty_leveldb::{DBIterator, LdbIterator, DB};

pub struct Iter(DBIterator);

impl Iterator for Iter {
    type Item = (Vec<u8>, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl Iter {
    pub fn from_db(db: &mut DB) -> Self {
        Self(db.new_iter().unwrap())
    }
}
