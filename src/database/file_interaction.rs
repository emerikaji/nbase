use super::DB;
use std::fs;
const SEP: u8 = 0x3b;
#[cfg(feature = "altsep")]
const SEP: u8 = 0x7c;

impl DB {
    pub fn read_from_file(s: String) -> DB {
        let name = s.clone() + ".ndb";
        let out = fs::read_to_string(name).expect("Failed to read").as_bytes().to_vec();
        let contentsize = usize::from_be_bytes(out[0..8].try_into().expect("Error reading header"));
        let lzo = minilzo_rs::LZO::init().unwrap();
        let contents = lzo.decompress_safe(&out[8..], contentsize).unwrap();
        let mut collection: Vec<(String,Vec<u8>)> = Vec::new();
        let mut mode = true;
        let mut key = String::new();
        let mut value = Vec::<u8>::new();
        for i in contents {
            if mode {
                if i == SEP {
                    mode = false;
                    continue;
                }
                key.push(i as char)
            } else {
                if i == SEP {
                    mode = true;
                    collection.push((key.clone(),value.clone()));
                    key = String::new();
                    value = Vec::<u8>::new();
                    continue;
                }
                value.push(i)
            }
        }
        return DB::open(s, collection);
    }

    pub fn write_to_file(self) {
        let mut contents = Vec::<u8>::new();
        let name = self.name.clone() + ".ndb";
        for p in &self.collection {
            contents.append(&mut p.0.as_bytes().to_vec());
            contents.push(SEP);
            contents.append(&mut p.1.to_vec());
            contents.push(SEP);
        };
        let mut lzo = minilzo_rs::LZO::init().unwrap();
        let mut out: Vec<u8> = contents.len().to_be_bytes().to_vec();
        out.append(&mut lzo.compress(&contents).unwrap());
        fs::write(name, out).expect("Failed to write");
    }

    pub fn to_json_file(self) {
        let name = self.name.clone() + ".json";
        let mut out = String::new();
        out += "{\n";
        out += self.to_json_object().as_str();
        out += "\n}\n";
        fs::write(name, out).expect("Failed to write");
    }
}