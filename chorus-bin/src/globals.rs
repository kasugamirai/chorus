use chorus_lib::config::Config;
use chorus_lib::store::Store;
use hyper::server::conn::Http;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::sync::OnceLock;
use std::time::Instant;
use tokio::sync::broadcast::Sender as BroadcastSender;
use tokio::sync::watch::Sender as WatchSender;

pub struct Globals {
    pub start_time: Instant,
    pub bytes_inbound: AtomicU64,
    pub bytes_outbound: AtomicU64,
    pub config: RwLock<Config>,
    pub store: OnceLock<Store>,
    pub http_server: Http,
    pub rid: OnceLock<String>,

    /// This is a broadcast channel where new incoming events are advertised by their offset.
    /// Every handler needs to listen to it and check if the incoming event matches any
    /// subscribed fitlers for their client, and if so, send the event to their client under
    /// that subscription.
    pub new_events: BroadcastSender<usize>,

    pub num_clients: AtomicUsize,
    pub shutting_down: WatchSender<bool>,
}

lazy_static! {
    pub static ref GLOBALS: Globals = {
        let mut http_server = hyper::server::conn::Http::new();
        http_server.http1_only(true);
        http_server.http1_keep_alive(true);

        let (new_events, _) = tokio::sync::broadcast::channel(512);
        let (shutting_down, _) = tokio::sync::watch::channel(false);

        Globals {
            start_time: Instant::now(),
            bytes_inbound: AtomicU64::new(0),
            bytes_outbound: AtomicU64::new(0),
            config: RwLock::new(Default::default()),
            store: OnceLock::new(),
            http_server,
            rid: OnceLock::new(),
            new_events,
            num_clients: AtomicUsize::new(0),
            shutting_down,
        }
    };
}
