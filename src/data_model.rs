use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// #[derive(Debug, Serialize, Deserialize)]
// pub enum AttributeValue {
//     Text(String),
//     Integer(i32),
//     Float(f64),
//     Bool(bool),
//     Array(Vec<AttributeValue>)
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
    pub name: String,
    pub rows: HashMap<String,HashMap<String,String>>
}