//! The types exchanged across our network

#![cfg_attr(not(test), no_std)]

use serde::{Deserialize, Serialize};

/// Represents a message that can be sent
/// between the client and server. The len
/// must be one less than the max datagram
/// size as we will be conveying the len
/// in the first byte.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Message(pub heapless::String<31>);

/// The maximum size of a datagram packet
pub const MAX_DATAGRAM_SIZE: usize = 32;
