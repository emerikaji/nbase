use std::fs;
use crate::database::DB;

const SEP: u8 = 0x3b;

pub struct DBs {
    name: String,
    pub db_list: Vec<DB>
}
impl DBs {
    pub fn new(s: String) -> DBs {
        let new_dbs = DBs {
            name: s,
            db_list: Vec::new()
        };
        new_dbs
    }

    pub fn open(s: String, l: Vec<DB>) -> DBs {
        let open_dbs = DBs {
            name: s,
            db_list: l
        };
        open_dbs
    }

    pub fn read_from_file(s: String) -> DBs { // dbname;;key;value;key;value;;
        let name = s.clone() + ".ndbs";
        let out = fs::read_to_string(name).expect("Failed to read").as_bytes().to_vec();
        let contentsize = usize::from_be_bytes(out[0..8].try_into().expect("Error reading header"));
        let lzo = minilzo_rs::LZO::init().unwrap();
        let contents = lzo.decompress_safe(&out[8..], contentsize).unwrap();
        let mut db_list: Vec<DB> = Vec::new();
        let mut collection: Vec<(String,Vec<u8>)> = Vec::new();
        let mut dbmode: bool = false; // false: out of dd; true: in db
        let mut kvmode = true; // true: key; false: value
        let mut sep_enc = false; // true when sep was just encountered
        let mut dbname = String::new();
        let mut key = String::new();
        let mut value = Vec::<u8>::new();
        for i in contents {
            if dbmode {
                if i == SEP {
                    if sep_enc {
                        db_list.push(DB::open(dbname, collection));
                        dbname = String::new();
                        collection = Vec::new();
                        dbmode = false;
                        continue;
                    }
                    kvmode = !kvmode;
                    if kvmode {
                        collection.push((key.clone(), value.clone()));
                        key = String::new();
                        value = Vec::<u8>::new();
                    }
                    sep_enc = true;
                    continue;
                }
                if kvmode {
                    key.push(i as char);
                } else {
                    value.push(i)
                }
                sep_enc = false;
            } else {
                if i == SEP {
                    if sep_enc {
                        dbmode = true;
                        kvmode = true;
                        continue;
                    }
                    sep_enc = true;
                    continue;
                }
                dbname.push(i as char);
                sep_enc = false;
            }
        }
        return DBs::open(s, db_list);
    }

    pub fn write_to_file(self) {
        let mut contents = Vec::<u8>::new();
        let name = self.name.clone() + ".ndbs";
        for d in &self.db_list {
            contents.append(&mut d.name.as_bytes().to_vec());
            contents.push(SEP);
            contents.push(SEP);
            for p in &d.collection {
                contents.append(&mut p.0.as_bytes().to_vec());
                contents.push(SEP);
                contents.append(&mut p.1.to_vec());
                contents.push(SEP);
            }
            contents.push(SEP)
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
        for (i, d) in self.db_list.iter().enumerate() {
            out += d.to_json_object().as_str();
            if i == self.db_list.len()-1 {
                out += "\n"
            } else {
                out += ",\n"
            }
        }
        out += "}\n";
        fs::write(name, out).expect("Failed to write");
    }
}