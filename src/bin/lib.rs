use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Contatto {
    pub name: String,
    pub surname: String,
    pub age: i32
}
impl PartialEq for Contatto {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.surname == other.surname && self.age == other.age
    }
}

pub fn byte_to_str(bytes: &[u8]) -> String{
    let mut new_str: String = String::from("");
    for c in bytes.iter() {
        if *c == 0x0 {break};
        new_str.push(*c as char);
    }
    new_str
}