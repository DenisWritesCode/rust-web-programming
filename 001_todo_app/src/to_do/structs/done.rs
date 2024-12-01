use super::base::Base;

pub struct Done {
    pub super_struct : Base
}

use super::super::traits::get::Get;
use super::super::traits::delete::Delete;
use super::super::traits::edit::Edit;

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}
