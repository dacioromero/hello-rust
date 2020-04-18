use std::collections::HashMap;
use std::sync::RwLock;

#[derive(juniper::GraphQLObject, Clone)]
struct Item {
    id: String,
    name: String,
    done: bool
}

pub struct Context {
    item_map: RwLock<HashMap<String, Item>>
}

pub fn create_context () -> Context {
    let item_map = RwLock::new(HashMap::new());

    Context { item_map }
}

impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    fn items(ctx: &Context) -> juniper::FieldResult<Vec<Item>> {
        let item_map = ctx.item_map.read().unwrap();
        let items = item_map.values().cloned().collect();

        Ok(items)
    }
}

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {
    fn createTodo(ctx: &Context, name: String) -> juniper::FieldResult<Item> {
        let mut item_map = ctx.item_map.write().unwrap();
        let id: String = item_map.len().to_string();

        let item = Item {
            id,
            name,
            done: false
        };

        item_map.insert(item.id.to_string(), item.clone());

        Ok(item.clone())
    }

    fn finishTodo(ctx: &Context, id: String) -> juniper::FieldResult<Item> {
        let mut item_map = ctx.item_map.write().unwrap();
        let mut item = item_map.get_mut(&id).unwrap();

        item.done = true;

        Ok(item.clone())
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema () -> Schema {
    Schema::new(Query, Mutation)
}
