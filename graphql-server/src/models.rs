use crate::schema::items;

#[derive(GraphQLObject, Clone, Queryable)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub completed: bool,
}

#[derive(GraphQLInputObject, Insertable)]
#[diesel(table_name = items)]
pub struct ItemCreateData {
    name: String,
    completed: Option<bool>,
}

#[derive(GraphQLInputObject, AsChangeset)]
#[diesel(table_name = items)]
pub struct ItemUpdateData {
    name: Option<String>,
    completed: Option<bool>,
}
