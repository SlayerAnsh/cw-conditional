use cw_storage_plus::{Item, Map};

use crate::types::{Condition, Variable};

pub const VARIABLES: Map<&str, Variable> = Map::new("variables");

pub const CONDITION: Item<Condition> = Item::new("condition");
