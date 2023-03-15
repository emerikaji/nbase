use super::DB;
use std::str::from_utf8;

impl DB {
    //output:
    //    "name": {
    //        "key": "value",
    //        "key": "value"
    //    }
    pub fn to_json_object(&self) -> String {
        let c = self.get_all();
        let mut ret = String::new();
        ret += "    \"";
        ret += self.name.as_str();
        ret += "\": {\n";
        for (i, p) in c.iter().enumerate() {
            ret += "        \"";
            ret += &p.0;
            ret += "\": \"";
            ret += from_utf8(&p.1).expect("Failed to get data");
            if i == c.len()-1 {
                ret += "\"\n";
            } else {
                ret += "\",\n";
            }
        }
        ret += "    }";
        return ret
    }
}