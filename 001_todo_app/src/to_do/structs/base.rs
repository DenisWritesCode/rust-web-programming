// Use the TaskStatus defined in mod.rs of parent/ directory.
// super -> mod.rs of this directory, super again for one level up mod.rs
use super::super::enums::TaskStatus;

// Define a base struct for to-do items.
// A String title & a TaskStatus status.
pub struct Base {
    pub title: String,
    pub status: TaskStatus
}