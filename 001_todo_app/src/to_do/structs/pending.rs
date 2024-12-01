// Import Base Module from this directory's mod.rs ->
// Remember to export the base module from mod.rs
use super::base::Base;
// Import TaskStatus enum from parent directory mod.rs.
use super::super::enums::TaskStatus;

// Define public Pending struct -> type/tuple.
pub struct Pending {
    pub super_struct: Base
}

// Implement Type
impl Pending {
    // constructor for generating new Pending Task using Base struct attributes.
    pub fn new(input_title: &str) -> Self {
        let base = Base{
            title: input_title.to_string(),
            status: TaskStatus::PENDING
        };
        return Pending{super_struct : base }
    }
}
