use std::str::from_utf8;

use super::DB;

impl DB {
    pub fn to_json(self) -> String {
        let c = self.get_all();
        let mut ret = String::new();
        ret += "{\n    \"";
        ret += self.name.as_str();
        ret += "\":{\n";
        for p in c {
            ret += "        \"";
            ret += p.0.as_str();
            ret += "\": \"";
            ret += from_utf8(&p.1).expect("Failed to get data");
            ret += "\",\n"
        }
        ret += "    }\n";
        ret += "}\n";
        return ret
    }
}