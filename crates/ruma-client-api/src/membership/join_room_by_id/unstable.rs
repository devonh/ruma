//! `/unstable/` ([spec])
//!
//! [spec]: https://spec.matrix.org/latest/client-server-api/#post_matrixclientv3roomsroomidjoin

use ruma_common::{
    api::{request, response, Metadata},
    metadata,
    serde::Raw,
    OwnedRoomId, OwnedServerName, RoomVersionId,
};
use ruma_events::AnyTimelineEvent;

use crate::membership::{join_room_by_id, ThirdPartySigned};

const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: true,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/unstable/org.matrix.msc4080/rooms/:room_id/join",
    }
};

/// Request type for the `join_room_by_id` endpoint.
#[request(error = crate::Error)]
pub struct Request {
    /// The room where the user should be invited.
    #[ruma_api(path)]
    pub room_id: OwnedRoomId,

    /// The signature of a `m.third_party_invite` token to prove that this user owns a third
    /// party identity which has been invited to the room.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_signed: Option<ThirdPartySigned>,

    /// Optional reason for joining the room.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl From<join_room_by_id::v3::Request> for Request {
    fn from(value: join_room_by_id::v3::Request) -> Self {
        Request {
            room_id: value.room_id,
            third_party_signed: value.third_party_signed,
            reason: value.reason,
        }
    }
}

/// Response type for the `join_room_by_id` endpoint.
#[response(error = crate::Error)]
pub struct Response {
    /// The room that the user joined.
    pub room_id: OwnedRoomId,

    /// The room's version.
    pub room_version: RoomVersionId,

    /// If the join is remote, this is the remote server used to initiate the join.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_server: Option<OwnedServerName>,

    /// Proto join event needed to join the room.
    pub pdu: Raw<AnyTimelineEvent>,
}

impl Request {
    /// Creates a new `Request` with the given room id.
    pub fn new(room_id: OwnedRoomId) -> Self {
        Self { room_id, third_party_signed: None, reason: None }
    }
}

impl Response {
    /// Creates a new `Response` with the given room id.
    pub fn new(
        room_id: OwnedRoomId,
        room_version: RoomVersionId,
        via_server: Option<OwnedServerName>,
        pdu: Raw<AnyTimelineEvent>,
    ) -> Self {
        Self { room_id, room_version, via_server, pdu }
    }
}
