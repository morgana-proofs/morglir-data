use std::fmt;
use serde::{
    de::{Deserializer, Visitor, MapAccess, Unexpected},
    Serialize, Deserialize
};
use serde::de::SeqAccess;
use crate::circuit::Circuit;


static CURRENT_VERSION: &str = "0.1.0";  // todo: get crate version


#[derive(PartialEq, Eq, Debug, Serialize)]
pub struct Program (
    pub Metadata,
    pub Circuit,
);

impl<'de> Deserialize<'de> for Program {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {

        struct ProgramVisitor {}
        impl ProgramVisitor {
            fn new() -> Self {
                ProgramVisitor {}
            }
        }

        impl<'de> Visitor<'de> for ProgramVisitor
        {
            // The type that our Visitor is going to produce.
            type Value = Program;

            // Format a message stating what data this Visitor expects to receive.
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("program tuple: Metadata, Circuit")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let metadata: Metadata = seq.next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(0, &self))?;

                if metadata.version != CURRENT_VERSION {
                    return Err(serde::de::Error::invalid_value(Unexpected::Str(&metadata.version), &format!("currently supported version is {}", CURRENT_VERSION).as_str()));
                }
                let circuit = seq.next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(1, &self))?;
                Ok(Program(metadata, circuit))
            }
        }

        deserializer.deserialize_tuple_struct("Program", 2, ProgramVisitor::new())
    }
}


#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub version: String,
}

impl Metadata {
    pub fn new() -> Self {
        Self {
            version: CURRENT_VERSION.to_string(),
        }
    }
}