//! `POST /_matrix/client/unstable/org.matrix.msc_cryptoids/sendPDUs`
//!
//! Send a set of PDUs for a room to the homeserver.

pub mod v4 {
    //! `POST /_matrix/client/unstable/org.matrix.msc_cryptoids/sendPDUs` ([MSC])
    //!
    //! [MSC]: TODO: add msc link

    use ruma_common::{
        api::{request, response, Metadata},
        metadata,
        serde::Raw,
        RoomVersionId,
    };
    use ruma_events::AnyTimelineEvent;

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: true,
        authentication: AccessToken,
        history: {
            unstable => "/_matrix/client/unstable/org.matrix.msc_cryptoids/sendPDUs",
        }
    };

    /// Request type for the `aliases` endpoint.
    #[request(error = crate::Error)]
    pub struct Request {
        /// The room ID to get aliases of.
        pub room_version: RoomVersionId,

        /// List of signed events for the homeserver to process.
        pub pdus: Vec<Raw<AnyTimelineEvent>>,
    }

    /// Response type for the `aliases` endpoint.
    #[response(error = crate::Error)]
    pub struct Response {}

    impl Request {
        /// Creates a new `Request` with the given room ID.
        pub fn new(room_version: RoomVersionId, pdus: Vec<Raw<AnyTimelineEvent>>) -> Self {
            Self { room_version, pdus }
        }
    }

    impl Response {
        /// Creates a new `Response` with the given aliases.
        pub fn new() -> Self {
            Self {}
        }
    }
}
