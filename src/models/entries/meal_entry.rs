use super::MetaEntry;
use crate::schema::meal_entries;
use diesel::dsl::{Eq, Filter, Select};
use diesel::prelude::*;
use serde::Serialize;

type AllColumns = (
    meal_entries::id,
    meal_entries::meta_entry_id,
    meal_entries::meal_type_id,
);

pub type All = Select<meal_entries::table, AllColumns>;

pub type WithID<'a> = Eq<meal_entries::id, &'a i32>;
pub type ByID<'a> = Filter<All, WithID<'a>>;


#[derive(Queryable, Serialize, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "meal_entries"]
#[belongs_to(MetaEntry)]
pub struct MealEntry {
    pub id: i32,
    pub meta_entry_id: i32,
    pub meal_type_id: i32,
}

impl MealEntry {
    pub fn with_id(id: &i32) -> WithID {
        meal_entries::id.eq(id)
    }

    pub fn all() -> All {
        meal_entries::table.select(meal_entries::all_columns)
    }

    pub fn by_id(id: &i32) -> ByID {
        Self::all().filter(Self::with_id(id))
    }
}
