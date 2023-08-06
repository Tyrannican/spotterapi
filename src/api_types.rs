use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ThingType {
    #[serde(rename = "thing")]
    Thing,
    #[serde(rename = "mammal")]
    Mammal,
    #[serde(rename = "plant")]
    Plant,
    #[serde(rename = "fungi")]
    Fungi,
    #[serde(rename = "insect")]
    Insect,
    #[serde(rename = "bird")]
    Bird,
    #[serde(rename = "reptile")]
    Reptile,
    #[serde(rename = "marine")]
    Marine,
}

impl ThingType {
    pub fn from_id(num: i32) -> Self {
        match num {
            0 => Self::Thing,
            1 => Self::Mammal,
            2 => Self::Plant,
            3 => Self::Fungi,
            4 => Self::Insect,
            5 => Self::Bird,
            6 => Self::Reptile,
            7 => Self::Marine,
            _ => Self::Thing,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Thing {
    pub name: String,
    pub thing: ThingType,
    pub lat: f64,
    pub lng: f64,
    pub count: i32,
    pub description: Option<String>,
    pub image: Option<String>,
    pub timestamp: String,
}
