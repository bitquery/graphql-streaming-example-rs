use graphql_client::GraphQLQuery;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Decimal(pub String);

impl Decimal {
    pub fn new(decimal: String) -> Self {
        Self(decimal)
    }
}

impl From<String> for Decimal {
    fn from(item: String) -> Self {
        Self::new(item)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Timestamp(pub i64);

impl Timestamp {
    pub fn new(timestamp: i64) -> Self {
        Self(timestamp)
    }
}

impl From<i64> for Timestamp {
    fn from(item: i64) -> Self {
        Self::new(item)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct BigInt(pub String);

impl BigInt {
    pub fn new(big_int: String) -> Self {
        Self(big_int)
    }
}

impl From<String> for BigInt {
    fn from(item: String) -> Self {
        Self::new(item)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct DateTime(pub String);

impl DateTime {
    pub fn new(date_time: String) -> Self {
        Self(date_time)
    }
}

impl From<String> for DateTime {
    fn from(item: String) -> Self {
        Self::new(item)
    }
}

#[derive(GraphQLQuery, Debug, Clone, Deserialize)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "graphql/subscriptions/dextrades.graphql",
    response_derives = "Debug, Clone"
)]
pub struct DexTrades;
