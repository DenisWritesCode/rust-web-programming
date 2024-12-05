use serde::ser::{SeializeStruct, Serialize, Serializer};

pub enum TaskStatus {
    DONE,
    PENDING,
}

impl Serialize for TaskStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Ok(serializer.serialize_str(&self.stringify().as_str())?)
    }
}
