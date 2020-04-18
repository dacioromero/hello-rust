use std::collections::HashMap;
use std::sync::RwLock;

macro_rules! get_write_lock {
    ($x:expr) => {
        $x.write().expect("Failed to get write lock")
    };
}

macro_rules! get_read_lock {
    ($x:expr) => {
        $x.read().expect("Failed to get read lock")
    };
}

macro_rules! item_not_exist {
    ($getter:expr) => {
        $getter.expect("Item {} doesn't exist")
    };
}

pub struct Context {
    item_map: RwLock<HashMap<String, Item>>,
}

impl juniper::Context for Context {}

pub fn create_context() -> Context {
    let item_map = RwLock::new(HashMap::new());

    Context { item_map }
}

#[derive(juniper::GraphQLObject, Clone)]
struct Item {
    id: String,
    name: String,
    done: bool,
}

#[derive(juniper::GraphQLEnum)]
enum ItemState {
    Doing = 0,
    Done = 1,
}

pub struct Query;

#[juniper::object(Context = Context)]
impl Query {
    fn items(ctx: &Context, state: Option<ItemState>) -> juniper::FieldResult<Vec<Item>> {
        let item_map = get_read_lock!(ctx.item_map);

        let items = item_map
            .values()
            .filter(|&item| match state {
                Some(ItemState::Doing) => !item.done,
                Some(ItemState::Done) => item.done,
                None => true,
            })
            .cloned()
            .collect();

        Ok(items)
    }
}

pub struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    fn createItem(ctx: &Context, name: String) -> juniper::FieldResult<Item> {
        let mut item_map = get_write_lock!(ctx.item_map);

        let id = item_map.len().to_string();
        let done = false;
        let item = Item { id, name, done };
        item_map.insert(item.id.to_owned(), item.clone());

        Ok(item.clone())
    }

    fn deleteItem(ctx: &Context, id: String) -> juniper::FieldResult<Item> {
        let mut item_map = get_write_lock!(ctx.item_map);

        let item = item_not_exist!(item_map.remove(&id));

        Ok(item)
    }

    fn markItem(ctx: &Context, id: String, done: bool) -> juniper::FieldResult<Item> {
        let mut item_map = get_write_lock!(ctx.item_map);

        let mut item = item_not_exist!(item_map.get_mut(&id));
        item.done = done;

        Ok(item.clone())
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query, Mutation)
}
