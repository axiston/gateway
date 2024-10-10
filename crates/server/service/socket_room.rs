use std::collections::HashMap;
use std::fmt;
use std::pin::Pin;
use std::sync::{Arc, Mutex, RwLock};
use std::task::{Context, Poll};

use axum::extract::ws::{Message, WebSocket};
use axum::Error;
use futures::stream::SplitSink;
use futures::Sink;

/// TODO.
#[must_use]
#[derive(Debug, Default, Clone)]
pub struct WebsocketServer {
    inner: Arc<RwLock<WebsocketServerInner>>,
}

#[derive(Debug, Default)]
struct WebsocketServerInner {
    server_id: String,
    next_room: u32, // Id as a string?
    // timestamp?
    rooms_map: HashMap<u32, WebsocketRoom>,
}

impl WebsocketServer {
    /// Creates a new [`WebsocketServer`].
    pub fn new() -> Self {
        let inner = Arc::new(RwLock::new(WebsocketServerInner {
            server_id: "".to_owned(),
            next_room: 0,
            rooms_map: HashMap::new(),
        }));

        Self { inner }
    }

    pub fn with_server_id(mut self, server_id: &str) -> Self {
        let binding = self.inner.clone();
        let mut lock = binding.write().unwrap();
        lock.server_id = server_id.to_owned();
        self
    }

    pub fn is_same_server_id(&self, server_id: &str) -> bool {
        todo!()
    }

    pub fn create_new_room(&self) -> (u32, WebsocketRoom) {
        // let mut lock = self.inner.write().unwrap();
        // let room = WebsocketRoom::new();
        // lock.rooms_map.insert(lock.next_room, room.clone());
        // lock.next_room += 1;
        // (lock.next_room - 1, room)

        todo!()
    }

    pub fn find_room_by_id(&self, room_id: u32) -> Option<WebsocketRoom> {
        todo!()
    }

    // TODO: with_room_id, sets room_id header on auth,
    // used to reestablish conn to the same server.
}

// impl CheckHealth for WebsocketRoom {
//     type Error = Error;
// }

#[derive(Default, Clone)]
pub struct WebsocketRoom {
    inner: Arc<Mutex<WebsocketRoomInner>>,
}

#[derive(Default)]
struct WebsocketRoomInner {
    /// u64 is a unique user identifier
    tx: HashMap<u64, SplitSink<WebSocket, Message>>,
}

impl WebsocketRoom {
    /// Returns an empty [`WebsocketRoom`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }
}

impl fmt::Debug for WebsocketRoom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebsocketRoom").finish_non_exhaustive()
    }
}

impl Sink<Message> for WebsocketRoom {
    type Error = Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn start_send(self: Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        todo!()
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        todo!()
    }
}
