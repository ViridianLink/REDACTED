use std::sync::Arc;

use twilight_cache_inmemory::{InMemoryCache, InMemoryCacheBuilder, ResourceType};
use twilight_gateway::{Intents, Shard, ShardId};
use twilight_http::Client as HttpClient;

use crate::sqlx_lib::PostgresPool;
use crate::Result;

pub struct Client {
    pub shard: Shard,
    pub http: Arc<HttpClient>,
    pub cache: InMemoryCache,
    pub data: Arc<Data>,
}

impl Client {
    pub async fn new(token: String) -> Self {
        let shard = Shard::new(
            ShardId::ONE,
            token.clone(),
            Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT,
        );

        let http = Arc::new(HttpClient::new(token));

        let cache = InMemoryCacheBuilder::new()
            .resource_types(ResourceType::GUILD)
            .build();

        let data = Arc::new(Data::new().await.unwrap());

        Self {
            shard,
            http,
            cache,
            data,
        }
    }
}

pub struct Data {
    pub pool: sqlx::PgPool,
}

impl Data {
    pub async fn new() -> Result<Self> {
        let pool = PostgresPool::init().await?;

        //     let mut data = client.data.write().await;
        //     // data.insert::<State>(State::new());
        //     // data.insert::<ImageCache>(ImageCache::new());
        //     // data.insert::<GoodMorningLockedUsers>(Vec::new());
        //     // data.insert::<GoodNightLockedUsers>(Vec::new());
        //     drop(data);

        Ok(Self { pool })
    }
}
