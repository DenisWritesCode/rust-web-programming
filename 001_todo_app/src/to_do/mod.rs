// Make structs in to_do/structs/* publicly available
pub mod structs;

// Make enum in to_do/enums.rs publicly available as a module.
pub mod enums;

// Use TaskStatus enum + Done & Pending structs. 
use enums::TaskStatus;
use structs::done::Done;
use structs::pending::Pending;

// All Possible To-Do Items for now. They are largely the same but differ in status.
pub enum ItemTypes {
    Pending(Pending),
    Done(Done)
}

// Generator of To-Do's.
// Accepts title and status as input.
// Returns an Item with the correct initialization.
pub fn to_do_factory(title: &str, status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => {
            ItemTypes::Done(Done::new(title))
        },
        TaskStatus::PENDING => {
            ItemTypes::Pending(Pending::new(title))
        }
    }
}
