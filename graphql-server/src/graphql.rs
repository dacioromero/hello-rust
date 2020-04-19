use crate::models::*;
use crate::schema;
use diesel::*;
use juniper::*;

type PgConnectionManager = r2d2::ConnectionManager<pg::PgConnection>;
type PgPool = r2d2::Pool<PgConnectionManager>;
type PgPooledConnection = r2d2::PooledConnection<PgConnectionManager>;

pub struct Context(PgPool);

impl Context {
    pub fn connection(&self) -> PgPooledConnection {
        self.0.get().expect("Failed to connect to pool")
    }
}

impl juniper::Context for Context {}

pub fn create_context(database_url: &String) -> Context {
    let manager = PgConnectionManager::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to build pool");

    Context(pool)
}
pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    fn items(ctx: &Context, is_done: Option<bool>) -> juniper::FieldResult<Vec<Item>> {
        use schema::items::dsl;

        let connection = ctx.connection();
        let mut query = dsl::items.into_boxed();

        match is_done {
            Some(d) => {
                query = query.filter(dsl::done.eq(d));
            }
            None => {}
        };

        let result = query
            .load::<Item>(&connection)
            .expect("Error loading items");

        Ok(result)
    }
}

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    fn createItem(ctx: &Context, name: String) -> juniper::FieldResult<Item> {
        use schema::items::dsl;

        let connection = ctx.connection();

        let new_item = NewItem {
            name: &name,
            done: &false,
        };

        let result = diesel::insert_into(dsl::items)
            .values(&new_item)
            .get_result(&connection)
            .expect("Error saving post");

        Ok(result)
    }

    fn deleteItem(ctx: &Context, id: i32) -> juniper::FieldResult<Item> {
        use schema::items::dsl;

        let pool = ctx.connection();

        let result = diesel::delete(dsl::items.filter(dsl::id.eq(id)))
            .get_result(&pool)
            .expect("Error deleting post");

        Ok(result)
    }

    fn markItem(ctx: &Context, id: i32, done: bool) -> juniper::FieldResult<Item> {
        use schema::items::dsl;

        let pool = ctx.connection();

        let result = diesel::update(dsl::items.filter(dsl::id.eq(id)))
            .set(dsl::done.eq(done))
            .get_result(&pool)
            .expect("Failed to update item");

        Ok(result)
    }
}

pub type RootNode = juniper::RootNode<'static, Query, Mutation, EmptySubscription<Context>>;

pub fn create_root_node() -> RootNode {
    RootNode::new(Query, Mutation, EmptySubscription::new())
}
