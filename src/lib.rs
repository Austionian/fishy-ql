use async_graphql::{EmptySubscription, Object, ID};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::middleware::from_extractor;
use axum::{extract::Extension, routing::post, Router};
use http::{HeaderValue, Method};
use tower::ServiceBuilder;
use tower_http::cors::Any;
use tower_http::{compression::CompressionLayer, cors::CorsLayer};

mod fish;
mod fishs;
mod fishs_by_lake;
mod router_auth;
mod thing;

use crate::fish::{get_fish, FishData};
use crate::fishs::{get_fishs, Fish};
use crate::fishs_by_lake::get_fishs_by_lake;
use crate::thing::{CreateThing, Thing};

struct Query;

#[Object]
impl Query {
    async fn fish_by_id(&self, id: ID) -> Option<FishData> {
        get_fish(id.to_string()).await
    }

    async fn fish_type(&self) -> Option<Vec<Fish>> {
        get_fishs().await
    }
    async fn fishs_by_lake(&self, lake: String) -> Option<Vec<Fish>> {
        get_fishs_by_lake(lake).await
    }
}

struct Mutation;

#[Object]
impl Mutation {
    // TODO: Fill in mutation resolvers
    async fn create_thing(&self, thing: CreateThing) -> Thing {
        let CreateThing { id, name } = thing;
        Thing { id, name }
    }
}

type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

async fn graphql_handler(schema: Extension<Schema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[must_use]
pub fn app() -> Router {
    let schema: Schema = Schema::build(Query, Mutation, EmptySubscription)
        .enable_federation()
        .limit_complexity(100)
        .finish();

    let cors = CorsLayer::new()
        .allow_methods([Method::POST])
        .allow_headers(Any)
        .allow_origin(
            "https://studio.apollographql.com"
                .parse::<HeaderValue>()
                .expect("Can enable sandbox CORS"),
        );

    Router::new().route("/", post(graphql_handler)).layer(
        ServiceBuilder::new()
            .layer(Extension(schema))
            .layer(CompressionLayer::new())
            .layer(cors)
            .layer(from_extractor::<router_auth::RequireRouterAuth>()),
    )
}
