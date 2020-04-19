use super::schema::items;

#[derive(juniper::GraphQLObject, Clone, Queryable)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub done: bool,
}

#[derive(Insertable)]
#[table_name = "items"]
pub struct NewItem<'a> {
    pub name: &'a str,
    pub done: &'a bool,
}
