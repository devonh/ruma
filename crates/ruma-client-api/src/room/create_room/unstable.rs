//! `POST /_matrix/client/unstable/org.matrix.msc_cryptoids/createRoom` ([MSC])
//!
//! [MSC]: TODO: add msc link

use ruma_common::{
    api::{request, response, Metadata},
    metadata,
    serde::Raw,
    OwnedRoomId, OwnedUserId, RoomVersionId,
};
use ruma_events::{
    room::power_levels::RoomPowerLevelsEventContent, AnyInitialStateEvent, AnyTimelineEvent,
};

use crate::{
    membership::Invite3pid,
    room::{create_room, Visibility},
};

const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/unstable/org.matrix.msc_cryptoids/createRoom",
    }
};

/// Request type for the `create_room` endpoint.
#[request(error = crate::Error)]
#[derive(Default)]
pub struct Request {
    /// Extra keys to be added to the content of the `m.room.create`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_content: Option<Raw<create_room::v3::CreationContent>>,

    /// List of state events to send to the new room.
    ///
    /// Takes precedence over events set by preset, but gets overridden by name and topic keys.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub initial_state: Vec<Raw<AnyInitialStateEvent>>,

    /// A list of user IDs to invite to the room.
    ///
    /// This will tell the server to invite everyone in the list to the newly created room.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub invite: Vec<OwnedUserId>,

    /// List of third party IDs of users to invite.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub invite_3pid: Vec<Invite3pid>,

    /// If set, this sets the `is_direct` flag on room invites.
    #[serde(default, skip_serializing_if = "ruma_common::serde::is_default")]
    pub is_direct: bool,

    /// If this is included, an `m.room.name` event will be sent into the room to indicate the
    /// name of the room.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Power level content to override in the default power level event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_level_content_override: Option<Raw<RoomPowerLevelsEventContent>>,

    /// Convenience parameter for setting various default state events based on a preset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<create_room::v3::RoomPreset>,

    /// The desired room alias local part.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_alias_name: Option<String>,

    /// Room version to set for the room.
    ///
    /// Defaults to homeserver's default if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_version: Option<RoomVersionId>,

    /// If this is included, an `m.room.topic` event will be sent into the room to indicate
    /// the topic for the room.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    /// A public visibility indicates that the room will be shown in the published room list.
    ///
    /// A private visibility will hide the room from the published room list. Defaults to
    /// `Private`.
    #[serde(default, skip_serializing_if = "ruma_common::serde::is_default")]
    pub visibility: Visibility,

    /// The senderID for the user creating this room.
    pub sender_id: String,
}

impl From<create_room::v3::Request> for Request {
    fn from(value: create_room::v3::Request) -> Self {
        Request {
            creation_content: value.creation_content,
            initial_state: value.initial_state,
            invite: value.invite,
            invite_3pid: value.invite_3pid,
            is_direct: value.is_direct,
            name: value.name,
            power_level_content_override: value.power_level_content_override,
            preset: value.preset,
            room_alias_name: value.room_alias_name,
            room_version: value.room_version,
            topic: value.topic,
            visibility: value.visibility,
            // TODO: don't hardcode this
            sender_id: String::from("PFKvhmuW9Hj0WSAkD09J99R0RQU8qlpQZ6vV6uR6B9c"),
        }
    }
}

/// Response type for the `create_room` endpoint.
#[response(error = crate::Error)]
pub struct Response {
    /// The created room's ID.
    pub room_id: OwnedRoomId,

    /// The created room's version.
    pub room_version: RoomVersionId,

    /// Proto events needed to create the room.
    pub pdus: Vec<Raw<AnyTimelineEvent>>,
}

impl Request {
    /// Creates a new `Request` will all-default parameters.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Response {
    /// Creates a new `Response` with the given room id.
    pub fn new(
        room_id: OwnedRoomId,
        room_version: RoomVersionId,
        pdus: Vec<Raw<AnyTimelineEvent>>,
    ) -> Self {
        Self { room_id, room_version, pdus }
    }
}
