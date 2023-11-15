//! `POST /_matrix/client/unstable/org.matrix.msc_cryptoids/sendPDUs`
//!
//! Send a set of PDUs for a room to the homeserver.

pub mod unstable {
    //! `POST /_matrix/client/unstable/org.matrix.msc_cryptoids/sendPDUs` ([MSC])
    //!
    //! [MSC]: TODO: add msc link

    use ruma_common::{
        api::{request, response, Metadata},
        metadata,
        serde::Raw,
        OwnedServerName, OwnedTransactionId, RoomVersionId,
    };
    use ruma_events::AnyTimelineEvent;
    use serde::{Deserialize, Serialize};

    const METADATA: Metadata = metadata! {
        method: POST,
        rate_limited: true,
        authentication: AccessToken,
        history: {
            unstable => "/_matrix/client/unstable/org.matrix.msc_cryptoids/send_pdus/:txn_id",
        }
    };

    /// Request parameters for the `/send_pdus` endpoint.
    #[derive(Clone, Debug, Deserialize, Serialize)]
    #[cfg_attr(not(feature = "unstable-exhaustive-types"), non_exhaustive)]
    pub struct PDUInfo {
        /// The remote server to send the event via.
        #[serde(skip_serializing_if = "Option::is_none")]
        pub via_server: Option<OwnedServerName>,

        /// The room ID to get aliases of.
        pub room_version: RoomVersionId,

        /// Signed event for the homeserver to process.
        pub pdu: Raw<AnyTimelineEvent>,
    }

    impl PDUInfo {
        /// Creates a new `PDUInfo` with the provided arguments.
        pub fn new(
            via_server: Option<OwnedServerName>,
            room_version: RoomVersionId,
            pdu: Raw<AnyTimelineEvent>,
        ) -> Self {
            Self { via_server, room_version, pdu }
        }
    }

    /// Request type for the `aliases` endpoint.
    #[request(error = crate::Error)]
    pub struct Request {
        /// A transaction ID for these events.
        #[ruma_api(path)]
        pub txn_id: OwnedTransactionId,

        /// List of signed events for the homeserver to process.
        pub pdus: Vec<PDUInfo>,
    }

    /// Response type for the `aliases` endpoint.
    #[response(error = crate::Error)]
    pub struct Response {}

    impl Request {
        /// Creates a new `Request` with the given room ID.
        pub fn new(txn_id: OwnedTransactionId, pdus: Vec<PDUInfo>) -> Self {
            Self { txn_id, pdus }
        }
    }

    impl Response {
        /// Creates a new `Response` with the given aliases.
        pub fn new() -> Self {
            Self {}
        }
    }
}
