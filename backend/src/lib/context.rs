use futures::stream::SplitSink;
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::RwLock;
use warp::ws::{Message, WebSocket};

use super::database::rb_db::RbDb;

lazy_static! {
    pub static ref AVATAR: String = env::var("AVATAR").unwrap();
    pub static ref SC: &'static [u8] = b"jfhkfljfjdj";
    pub static ref CONTEXT: Context = Arc::new(PrimContext {
        rbdb: Arc::new(RbDb::new()),
        users: Users::default()
    });
}

pub type Users = Arc<RwLock<HashMap<u32, SplitSink<WebSocket, Message>>>>;
pub type Context = Arc<PrimContext>;
pub type ARbDb = Arc<RbDb>;

pub struct PrimContext {
    pub rbdb: ARbDb,
    pub users: Users,
}
