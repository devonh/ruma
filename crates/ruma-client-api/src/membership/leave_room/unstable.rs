//! `/unstable/` ([spec])
//!
//! [spec]: https://spec.matrix.org/latest/client-server-api/#post_matrixclientv3roomsroomidleave

use ruma_common::{
    api::{request, response, Metadata},
    metadata,
    serde::Raw,
    OwnedRoomId,
};
use ruma_events::AnyTimelineEvent;

const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: true,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/unstable/org.matrix.msc4080/rooms/:room_id/leave",
    }
};

/// Request type for the `leave_room` endpoint.
#[request(error = crate::Error)]
pub struct Request {
    /// The room to leave.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    /// Optional reason to be included as the `reason` on the subsequent membership event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Response type for the `leave_room` endpoint.
#[response(error = crate::Error)]
pub struct Response {
    /// Proto leave event needed to invite to the room.
    pub pdu: Raw<AnyTimelineEvent>,
}

impl Request {
    /// Creates a new `Request` with the given room id.
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self { room_id, reason: None }
    }
}

impl Response {
    /// Creates an empty `Response`.
    pub fn new(pdu: Raw<AnyTimelineEvent>) -> Self {
        Self { pdu }
    }
}
