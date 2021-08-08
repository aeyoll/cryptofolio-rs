use serde::{Deserialize, Serialize};

#[crud_table]
#[derive(Debug, Serialize, Deserialize)]
pub struct Cryptocurrency {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub price: Option<f64>,
    pub spent: Option<f64>,
}
