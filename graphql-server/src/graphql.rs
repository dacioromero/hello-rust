use crate::models::*;
use crate::schema;
use diesel::{
    delete, insert_into,
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PoolError, PooledConnection},
    update, ExpressionMethods, QueryDsl, RunQueryDsl,
};
use juniper::{FieldResult, ID};

type PgConnectionManager = ConnectionManager<PgConnection>;
type PgPool = Pool<PgConnectionManager>;
type PgPooledConnection = PooledConnection<PgConnectionManager>;

pub struct Context(PgPool);

impl Context {
    pub fn connection(&self) -> Result<PgPooledConnection, PoolError> {
        self.0.get()
    }
}

impl juniper::Context for Context {}

pub fn create_context(database_url: &String) -> Context {
    let manager = PgConnectionManager::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to build pool");

    Context(pool)
}

// TODO: Determine way to constrain to keys of models::Item
#[derive(GraphQLInputObject)]
pub struct ItemWhereInput {
    id: Option<ID>,
    name: Option<String>,
    completed: Option<bool>,
}

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    fn item(ctx: &Context, id: ID) -> FieldResult<Item> {
        use schema::items::dsl;

        let connection = ctx.connection()?;
        let result = dsl::items
            .filter(dsl::id.eq(id.parse::<i32>()?))
            .get_result(&connection)?;

        Ok(result)
    }

    // TODO: Report issue about how r#where doesn't work
    fn items(ctx: &Context, where_: Option<ItemWhereInput>) -> FieldResult<Vec<Item>> {
        use schema::items::dsl;

        let connection = ctx.connection()?;
        let mut query = dsl::items.into_boxed();

        // TODO: Research a better pattern for this
        if let Some(w) = where_ {
            if let Some(id) = w.id {
                query = query.filter(dsl::id.eq(id.parse::<i32>()?));
            }

            if let Some(name) = w.name {
                query = query.filter(dsl::name.eq(name))
            }

            if let Some(completed) = w.completed {
                query = query.filter(dsl::completed.eq(completed))
            }
        }

        let result = query.load(&connection)?;

        Ok(result)
    }
}

pub struct Mutation;

#[graphql_object(Context = Context)]
impl Mutation {
    fn createItem(ctx: &Context, data: ItemCreateData) -> FieldResult<Item> {
        use schema::items::dsl;

        let connection = ctx.connection()?;

        let result = insert_into(dsl::items)
            .values(&data)
            .get_result(&connection)?;

        Ok(result)
    }

    fn deleteItem(ctx: &Context, id: ID) -> FieldResult<Item> {
        use schema::items::dsl;

        let connection = ctx.connection()?;

        let result =
            delete(dsl::items.filter(dsl::id.eq(id.parse::<i32>()?))).get_result(&connection)?;

        Ok(result)
    }

    fn updateItem(ctx: &Context, id: ID, data: ItemUpdateData) -> FieldResult<Item> {
        use schema::items::dsl;

        let connection = ctx.connection()?;

        let result = update(dsl::items.filter(dsl::id.eq(id.parse::<i32>()?)))
            .set(&data)
            .get_result(&connection)?;

        Ok(result)
    }
}

pub struct Subscription;

#[graphql_object(Context = Context)]
impl Subscription {}

pub type RootNode = juniper::RootNode<'static, Query, Mutation, Subscription>;

pub fn create_root_node() -> RootNode {
    RootNode::new(Query, Mutation, Subscription)
}
