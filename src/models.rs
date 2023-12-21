use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Entry {
    pub id: i32,
    pub body: String,
    pub created_at: String
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewEntry<'a> {
    pub body: &'a str,
    pub created_at: &'a str,
}
