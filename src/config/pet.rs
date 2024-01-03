use serde::de::{self, Deserialize, Deserializer, Unexpected, Visitor};
use std::fmt;

pub struct Pet {
    pub r#type: String,
    pub name: String,
}

struct PetVisitor;

impl<'de> Visitor<'de> for PetVisitor {
    type Value = Pet;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Pet")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Pet, V::Error>
    where
        V: de::MapAccess<'de>,
    {
        let mut r#type = None;
        let mut name = None;
        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "type" => {
                    let pet_type: String = map.next_value()?;
                    if !["cat", "dog", "bird"].contains(&pet_type.as_str()) {
                        return Err(de::Error::invalid_value(Unexpected::Str(&pet_type), &self));
                    }
                    r#type = Some(pet_type);
                }
                "name" => {
                    let pet_name: String = map.next_value()?;
                    if pet_name.len() > 20 {
                        return Err(de::Error::invalid_length(pet_name.len(), &self));
                    }
                    name = Some(pet_name);
                }
                _ => return Err(de::Error::unknown_field(&key, &["type", "name"])),
            }
        }
        let r#type = r#type.ok_or_else(|| de::Error::missing_field("type"))?;
        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
        Ok(Pet { r#type, name })
    }
}

impl<'de> Deserialize<'de> for Pet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Pet", &["type", "name"], PetVisitor)
    }
}

