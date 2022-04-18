use std::convert::TryInto;
use std::convert::TryFrom;
use from_value::{FromValue, IntoValue};
use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, FromValue, IntoValue)]
pub struct MyStruct {
    pub string: String,
}

#[test]
pub fn main() {
    let hey = MyStruct {
        string: "Hey!".to_string()
    };
    let value: Value = hey.try_into().unwrap();
    println!("{}",serde_json::to_string_pretty(&value).unwrap());

    let hey = MyStruct::try_from(value).unwrap();

}