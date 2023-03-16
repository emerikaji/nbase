#[cfg(feature = "file")]
mod file_interaction;
#[cfg(feature = "json")]
mod json_convert;
const SEP: u8 = 0x3b;
#[cfg(feature = "altsep")]
const SEP: u8 = 0x7c;

/// Represents a key/value store
pub struct DB{
    /// The name of the store will be used as the name of the file
    pub name: String,
    /// Key/value pairs are stored in a vec. Keys are always strings, values can be anything
    pub collection: Vec<(String,Vec<u8>)>
}
impl DB {
    /// Creates a new DB
    pub fn new(s: String) -> DB {
        let new_db = DB {
            name: s,
            collection: Vec::new()
        };
        new_db
    }

    pub fn open(s: String, c: Vec<(String,Vec<u8>)>) -> DB {
        let open_db = DB {
            name: s,
            collection: c
        };
        open_db
    }

    pub fn get_all(&self) -> Vec<(String,Vec<u8>)> {
        return self.collection.clone()
    }

    pub fn get(&self, k: String) -> (Option<Vec<u8>>,usize) {
        for (i,pair) in self.collection.iter().enumerate() {
            if pair.0 == k {
                return (Some(pair.1.clone()),i)
            }
        }
        return (None,0)
    }

    pub fn add(&mut self, k: String, v: Vec<u8>) -> bool {
        if k.contains(SEP as char) || v.contains(&SEP) {
            return false
        }
        if self.get(k.clone()).0 == None {
            self.collection.push((k,v));
            return true
        }
        false
    }

    pub fn upd(&mut self, k: String, v: Vec<u8>) -> bool {
        if v.contains(&SEP) {
            return false
        }
        let (o, i) = self.get(k);
        if o == None {
            return false
        }
        self.collection[i].1 = v;
        true
    }

    pub fn remm(&mut self, k: String) -> bool {
        let (o, i) = self.get(k);
        if o == None {      
            return false
        }
        self.collection.remove(i);
        true
    }
}