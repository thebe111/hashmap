#[derive(Debug, PartialEq)]
struct Item<'a> {
    key: &'a str,
    value: &'a str,
}

#[allow(dead_code)]
#[derive(Debug)]
struct HashTable<'a> {
    size: u32,
    items: Vec<Item<'a>>,
}

#[allow(dead_code)]
impl<'a> Item<'a> {
    fn new(key: &'a str, value: &'a str) -> Self {
        Self { key, value }
    }
}

#[allow(dead_code)]
impl<'a> HashTable<'a> {
    fn new() -> Self {
        Self {
            size: 0,
            items: vec![],
        }
    }

    // NOTE: prevent collisions - open addressing with double hashing
    fn gethash(&self, key: &str, buckets: u32, attempts: u16) -> u32 {
        let ahash = self.hash(key, 151, buckets);
        let bhash = self.hash(key, 163, buckets);

        let res = (ahash + (attempts as u32 * (bhash + 1))) % buckets;

        res
    }

    fn hash(&self, key: &str, a: u32, buckets: u32) -> u32 {
        let mut hash = 0;
        let keylen = key.len();

        for i in 0..keylen {
            hash += a.pow((keylen - (i + 1)).try_into().unwrap()) * key.as_bytes()[i] as u32;
            hash %= buckets;
        }

        hash
    }

    fn search(self, key: &'a str) {}

    // @@@
    fn add(&mut self, item: Item<'a>) {
        let index = self.gethash(item.key, self.size, 0);

        for (i, item) in self.items.iter().enumerate() {
            loop {
                if let Some(record) = self.items.get(&item) {
                    break;
                }

                index = self.gethash(item.key, self.size, i as u16);
            }
        }

        self.items[index] = item;
        self.size += 1;
    }

    fn rm(&mut self, key: &str) {
        self.size -= 1;
        self.items.retain(|i| i.key != key);
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_hashtable_add() {
        let mut hashtable = HashTable::new();

        let i = Item::new("k", "v");

        let expect = [Item { key: "k", value: "v" }];

        hashtable.add(i);

        assert!(hashtable.items.iter().eq(expect.iter()));
    }

    #[test]
    fn main_hashtable_rm() {
        let mut hashtable = HashTable::new();

        let i1 = Item::new("k1", "v1");
        let i2 = Item::new("k2", "v2");

        hashtable.add(i1);
        hashtable.add(i2);

        let expect = [Item { key: "k2", value: "v2" }];

        hashtable.rm("k1");

        assert!(hashtable.items.iter().eq(expect.iter()));
    }

    #[test]
    fn main_hashtable_hash() {
        let hashtable = HashTable::new();

        assert_eq!(hashtable.hash("cat", 151, 53), 5);
        assert_eq!(hashtable.hash("cat", 163, 53), 21);
    }
}

