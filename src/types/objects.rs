use tes3::esp::TES3Object;

use crate::prelude::*;

pub type ObjectId = String;
pub type Tag = &'static [u8; 4];
pub type TaggedId = (Tag, ObjectId);
pub type Objects = HashMap<TaggedId, TES3Object>;
