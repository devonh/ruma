//! [GET /_matrix/client/r0/rooms/{roomId}/messages](https://matrix.org/docs/spec/client_server/r0.2.0.html#get-matrix-client-r0-rooms-roomid-messages)

use ruma_api_macros::ruma_api;
use ruma_events::collections::only;
use ruma_identifiers::RoomId;
use serde::{Deserialize, Serialize};

ruma_api! {
    metadata {
        description: "Get message events for a room.",
        method: GET,
        name: "get_message_events",
        path: "/_matrix/client/r0/rooms/:room_id/messages",
        rate_limited: false,
        requires_authentication: true,
    }

    request {
        /// The room to get events from.
        #[ruma_api(path)]
        pub room_id: RoomId,
        /// The token to start returning events from.
        ///
        /// This token can be obtained from a
        /// prev_batch token returned for each room by the sync API, or from a start or end token
        /// returned by a previous request to this endpoint.
        pub from: String,
        /// The token to stop returning events at.
        ///
        /// This token can be obtained from a prev_batch
        /// token returned for each room by the sync endpoint, or from a start or end token returned
        /// by a previous request to this endpoint.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub to: Option<String>,
        /// The direction to return events from.
        pub dir: Direction,
        /// The maximum number of events to return.
        ///
        /// Default: 10.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<u64>,
    }

    response {
        /// The token the pagination starts from.
        pub start: String,
        /// A list of room events.
        pub chunk: Vec<only::RoomEvent>,
        /// The token the pagination ends at.
        pub end: String,
    }
}

/// The direction to return events from.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Direction {
    /// Return events backwards in time from the requested `from` token.
    #[serde(rename = "b")]
    Backward,
    /// Return events forwards in time from the requested `from` token.
    #[serde(rename = "f")]
    Forward,
}
