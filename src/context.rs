use std::{ops::Deref, sync::Arc, time::Instant};

use crate::structs::Rule;
use sqlx::PgPool;
use tokio::sync::RwLock;
use twilight_http::Client as HttpClient;
use twilight_model::id::{marker::ApplicationMarker, Id};

pub struct Data {
    pub rules: Vec<Rule>,
}

#[derive(Clone)]
pub struct ContextRef {
    pub app_id: Id<ApplicationMarker>,
    pub http: Arc<HttpClient>,
    pub data: Arc<RwLock<Data>>,
    pub start: Instant,
    pub db: PgPool,
    pub pfp: String,
}

impl ContextRef {
    fn new(
        app_id: Id<ApplicationMarker>,
        http: Arc<HttpClient>,
        db: PgPool,
        pfp: String,
        rules: Vec<Rule>,
    ) -> Self {
        Self {
            http,
            app_id,
            db,
            data: Arc::new(Data { rules }.into()),
            pfp,
            start: Instant::now(),
        }
    }

    pub fn interaction(&self) -> twilight_http::client::InteractionClient<'_> {
        self.http.interaction(self.app_id)
    }
}

#[derive(Clone)]
pub struct Context(pub Arc<ContextRef>);

impl Context {
    pub fn new(
        app_id: Id<ApplicationMarker>,
        http: Arc<HttpClient>,
        db: PgPool,
        pfp: String,
        rules: Vec<Rule>,
    ) -> Self {
        Self(Arc::new(ContextRef::new(app_id, http, db, pfp, rules)))
    }
}

impl Deref for Context {
    type Target = ContextRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
