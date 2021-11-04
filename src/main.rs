#![deny(warnings)]

use std::pin::Pin;

use futures::{Stream, StreamExt};
use juniper::{
    graphql_object, graphql_subscription, http::GraphQLRequest, DefaultScalarValue, EmptyMutation,
    FieldError, FieldResult, RootNode, SubscriptionCoordinator,
};
use juniper_subscriptions::Coordinator;

#[derive(Clone)]
pub struct Context {
    db: sqlx::SqlitePool
}

impl juniper::Context for Context {}

impl Context {
    fn new() -> Self {
        Self { db: sqlx::SqlitePool::connect_lazy("sqlite::memory:").unwrap() }
    }
}

pub struct Query;

#[graphql_object(context = Context)]
impl Query {
    async fn hello_world() -> FieldResult<String> {
        sqlx::query_scalar("select 'Hello, world!'")
            .fetch_one(&executor.context().db).await
            .map_err(|err: sqlx::Error| err.into())
    }
}

pub struct Subscription;

type StringStream = Pin<Box<dyn Stream<Item = Result<String, FieldError>> + Send>>;

#[graphql_subscription(context = Context)]
impl Subscription {
    async fn hello_world(
        context: &Context
    ) -> StringStream {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));

        let stream = async_stream::stream! {
            loop {
                interval.tick().await;

                yield sqlx::query_scalar("select 'tic'")
                    .fetch_one(&executor.context().db).await
                    .map_err(|err: sqlx::Error| err.into());

                yield Ok("tac".into())
            }
        };

        Box::pin(stream)
    }
}

type Schema = RootNode<'static, Query, EmptyMutation<Context>, Subscription>;

fn schema() -> Schema {
    Schema::new(Query {}, EmptyMutation::new(), Subscription {})
}

#[tokio::main]
async fn main() {
    let schema = schema();
    let ctx = Context::new();
    let query_req: GraphQLRequest<DefaultScalarValue> = serde_json::from_str(
        r#"{
            "query": "query { helloWorld }"
        }"#,
    )
    .unwrap();
    let result = query_req.execute(&schema, &ctx).await;
    println!("{}", serde_json::to_string(&result).unwrap());

    let subscription_req: GraphQLRequest<DefaultScalarValue> = serde_json::from_str(
        r#"{
            "query": "subscription { helloWorld }"
        }"#,
    )
    .unwrap();
    let coordinator = Coordinator::new(schema);
    let mut conn = coordinator.subscribe(&subscription_req, &ctx).await.unwrap();
    while let Some(result) = conn.next().await {
        println!("{}", serde_json::to_string(&result).unwrap());
    }
}
