
use eframe::egui::Color32;
use serde::{Deserialize, Deserializer};



pub mod color {
    use super::*;
    use crate::color::ColorHex;

    pub fn serialize<S>(val: &Color32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let hex = val.to_hex();
        serializer.serialize_str(&hex)
    }

    pub fn deserialize<'de, D>(deser: D) -> Result<Color32, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = String::deserialize(deser)?;
        let color = Color32::from_hex_panic(&val);
        Ok(color)
    }
}
