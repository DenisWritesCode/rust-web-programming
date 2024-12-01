// Import Base Module from this directory's mod.rs ->
// Remember to export the base module from mod.rs
use super::base::Base;
use super::super::traits::get::Get;
use super::super::traits::edit::Edit;
use super::super::traits::create::Create;

// Define public Pending struct -> type/tuple.
pub struct Pending {
    pub super_struct: Base
}

// Implement Type
impl Get for Pending {}
impl Edit for Pending {}
impl Create for Pending {}
