use rusty_leveldb::{DBIterator, LdbIterator, DB};

pub struct DBIter(DBIterator);

impl Iterator for DBIter {
    type Item = (Vec<u8>, Vec<u8>);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl DBIter {
    pub fn from_db(db: &mut DB) -> Self {
        Self(db.new_iter().unwrap())
    }
}
