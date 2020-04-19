use crate::schema::items;

#[derive(GraphQLObject, Clone, Queryable)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub done: bool,
}

#[derive(GraphQLInputObject, Insertable)]
#[table_name = "items"]
pub struct ItemCreateData {
    name: String,
    done: Option<bool>,
}

#[derive(GraphQLInputObject, AsChangeset)]
#[table_name = "items"]
pub struct ItemUpdateData {
    name: Option<String>,
    done: Option<bool>,
}
