use async_tungstenite::tungstenite::{
    client::IntoClientRequest,
    http::{header, HeaderValue},
    Message,
};
use eyre::Result;
use futures::StreamExt;
use graphql_client::GraphQLQuery;
use graphql_ws_client::{
    graphql::{GraphQLClient, StreamingOperation},
    AsyncWebsocketClient, GraphQLClientClientBuilder, SubscriptionStream,
};

pub mod queries;
mod tokio_spawner;

use tokio_spawner::TokioSpawner;

pub type DexTradesQuery = queries::DexTrades;
pub type DexTradesVariables = queries::dex_trades::Variables;


pub async fn subscribe<T: GraphQLQuery + Send + Sync + Unpin + 'static>(
    api_key: &str,
    variables: T::Variables,
) -> Result<(
    AsyncWebsocketClient<GraphQLClient, Message>,
    SubscriptionStream<GraphQLClient, StreamingOperation<T>>,
)>
where
    <T as GraphQLQuery>::Variables: Send + Sync + Unpin,
    <T as GraphQLQuery>::ResponseData: std::fmt::Debug,
{
    let mut request = "wss://streaming.bitquery.io/eap".into_client_request()?;
    request.headers_mut().insert(
        header::SEC_WEBSOCKET_PROTOCOL,
        HeaderValue::from_str("graphql-transport-ws")?,
    );
    request
        .headers_mut()
        .insert("Authorization", HeaderValue::from_str(format!("Bearer {}", api_key).as_str())?);

    let (connection, _) = async_tungstenite::tokio::connect_async(request).await?;

    let (sink, stream) = connection.split::<Message>();

    let mut client = GraphQLClientClientBuilder::new()
        .build(stream, sink, TokioSpawner::current())
        .await?;

    let stream = client
        .streaming_operation(StreamingOperation::<T>::new(variables))
        .await?;

    Ok((client, stream))
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let (_client, mut stream) = subscribe::<DexTradesQuery>(
        &std::env::var("API_KEY")?,
        DexTradesVariables {program_id: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P".to_string()},
    )
    .await?;

    while let Some(response) = stream.next().await {
        dbg!(&response);
    }

    Ok(())
}
