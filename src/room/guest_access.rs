//! Types for the *m.room.guest_access* event.

use ruma_events_macros::ruma_event;
use serde::{Deserialize, Serialize};

ruma_event! {
    /// Controls whether guest users are allowed to join rooms.
    ///
    /// This event controls whether guest users are allowed to join rooms. If this event is absent,
    /// servers should act as if it is present and has the value `GuestAccess::Forbidden`.
    GuestAccessEvent {
        kind: StateEvent,
        event_type: RoomGuestAccess,
        content: {
            /// A policy for guest user access to a room.
            pub guest_access: GuestAccess,
        },
    }
}

/// A policy for guest user access to a room.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum GuestAccess {
    /// Guests are allowed to join the room.
    #[serde(rename = "can_join")]
    CanJoin,

    /// Guests are not allowed to join the room.
    #[serde(rename = "forbidden")]
    Forbidden,

    /// Additional variants may be added in the future and will not be considered breaking changes
    /// to ruma-events.
    #[doc(hidden)]
    #[serde(skip)]
    __Nonexhaustive,
}

impl_enum! {
    GuestAccess {
        CanJoin => "can_join",
        Forbidden => "forbidden",
    }
}
