mod file_interaction;
mod json_convert;
////DATABASE////
pub struct DB{
    name: String,
    collection: Vec<(String,Vec<u8>)>
}
impl DB {
    ////DATABASE METHODS////
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
        if self.get(k.clone()).0 == None {
            self.collection.push((k,v));
            return true
        }
        false
    }

    pub fn upd(&mut self, k: String, v: Vec<u8>) -> bool {
        let (o, i) = self.get(k);
        if o == None {
            return false
        }
        self.collection[i].1 = v;
        true
    }

    pub fn rem(&mut self, k: String) -> bool {
        let (o, i) = self.get(k);
        if o == None {      
            return false
        }
        self.collection.remove(i);
        true
    }
}