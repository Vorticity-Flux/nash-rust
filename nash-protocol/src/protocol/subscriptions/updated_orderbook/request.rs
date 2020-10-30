use crate::graphql;
use graphql::updated_orderbook;
use graphql_client::GraphQLQuery;

// Subscribe to order book updates on `Market`.
#[derive(Clone, Debug)]
pub struct SubscribeOrderbook {
    pub market: String,
}

impl SubscribeOrderbook {
    pub fn make_query(&self) -> graphql_client::QueryBody<updated_orderbook::Variables> {
        graphql::UpdatedOrderbook::build_query(updated_orderbook::Variables {
            market_name: self.market.clone(),
        })
    }
}
