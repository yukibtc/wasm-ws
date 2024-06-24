// Copyright (c) 2019-2022 Naja Melan
// Copyright (c) 2023-2024 Yuki Kishimoto
// Distributed under the MIT software license

#![forbid(unsafe_code)]
#![allow(clippy::arc_with_non_send_sync)]

use wasm_bindgen_futures::spawn_local;

mod error;
mod event;
mod message;
mod pharos;
mod socket;
mod state;
mod stream;

pub use self::error::Error;
pub use self::event::{CloseEvent, WsEvent};
pub use self::message::WsMessage;
use self::pharos::SharedPharos;
pub use self::socket::WebSocket;
pub use self::state::WsState;
pub use self::stream::io::WsStreamIo;
pub use self::stream::WsStream;

/// Helper function to reduce code bloat
pub(crate) fn notify(pharos: SharedPharos<WsEvent>, evt: WsEvent) {
    let notify = async move {
        pharos
            .notify(evt)
            .await
            .map_err(|e| unreachable!("{:?}", e))
            .unwrap(); // only happens if we closed it.
    };
    spawn_local(notify);
}
