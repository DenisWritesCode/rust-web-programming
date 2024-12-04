

// A public enum of Task Status. Can Either be Done or Pending.
pub enum TaskStatus {
    DONE,
    PENDING
}

// Method to allow our TaskStatus to be represented in string format.
impl TaskStatus {
    // Stringify -> Convert the current TaskStatus enum to Strings.
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => {"DONE".to_string()},
            &Self::PENDING => {"PENDING".to_string()},
        }
    }

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} not supported", input_string)
        }
    }
}
