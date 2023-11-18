//! `/unstable/` ([spec])
//!
//! [spec]: https://spec.matrix.org/latest/client-server-api/#post_matrixclientv3keysupload

use std::collections::BTreeMap;

use js_int::UInt;
use ruma_common::{
    api::{request, response, Metadata},
    encryption::{DeviceKeys, OneTimeCryptoID, OneTimeKey},
    metadata,
    serde::Raw,
    DeviceKeyAlgorithm, OwnedDeviceKeyId,
};

const METADATA: Metadata = metadata! {
    method: POST,
    rate_limited: false,
    authentication: AccessToken,
    history: {
        unstable => "/_matrix/client/unstable/org.matrix.msc4080/keys/upload",
    }
};

/// Request type for the `upload_keys` endpoint.
#[request(error = crate::Error)]
#[derive(Default)]
pub struct Request {
    /// Identity keys for the device.
    ///
    /// May be absent if no new identity keys are required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_keys: Option<Raw<DeviceKeys>>,

    /// One-time public keys for "pre-key" messages.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub one_time_keys: BTreeMap<OwnedDeviceKeyId, Raw<OneTimeKey>>,

    /// One-time cryptoIDs for invites.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub one_time_cryptoids: BTreeMap<OwnedDeviceKeyId, Raw<OneTimeCryptoID>>,

    /// Fallback public keys for "pre-key" messages.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub fallback_keys: BTreeMap<OwnedDeviceKeyId, Raw<OneTimeKey>>,
}

/// Response type for the `upload_keys` endpoint.
#[response(error = crate::Error)]
pub struct Response {
    /// For each key algorithm, the number of unclaimed one-time keys of that
    /// type currently held on the server for this device.
    pub one_time_key_counts: BTreeMap<DeviceKeyAlgorithm, UInt>,

    /// For each cryptoID algorithm, the number of unclaimed one-time cryptoIDs of that
    /// type currently held on the server for this device.
    pub one_time_cryptoid_counts: BTreeMap<DeviceKeyAlgorithm, UInt>,
}

impl Request {
    /// Creates an empty `Request`.
    pub fn new() -> Self {
        Default::default()
    }
}

impl Response {
    /// Creates a new `Response` with the given one time key counts.
    pub fn new(
        one_time_key_counts: BTreeMap<DeviceKeyAlgorithm, UInt>,
        one_time_cryptoid_counts: BTreeMap<DeviceKeyAlgorithm, UInt>,
    ) -> Self {
        Self { one_time_key_counts, one_time_cryptoid_counts }
    }
}
