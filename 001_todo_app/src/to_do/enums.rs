

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
}

// Achieve Stringify-ing of Task Status with implementing the Display trait.
//
// import format module
// use std::fmt;
//
// Trait is only utilised when needed
// impl fmt::Display for TaskStatus {
//     // println!("{}", TaskStatus::DONE);
//     // let outcome = TaskStatus::DONE.to_string();
//     // println!("{}", outcome);
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match &self {
//             &Self::DONE => { write!(f, "DONE")},
//             &Self::PENDING => { write!(f, "PENDING")},
//         }
//     }
// }
