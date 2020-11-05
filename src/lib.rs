// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! BlindBid impl

use dusk_plonk::prelude::*;

const V_RAW_MIN: u64 = 50_000u64;
const V_RAW_MAX: u64 = 250_000u64;

/// The minimum amount user is permitted to bid
pub const V_MIN: &'static JubJubScalar =
    &JubJubScalar::from_raw([V_RAW_MIN, 0, 0, 0]);
/// The maximum amount user is permitted to bid
pub const V_MAX: &'static JubJubScalar =
    &JubJubScalar::from_raw([V_RAW_MAX, 0, 0, 0]);

pub mod bid;
#[cfg(feature = "canon")]
pub mod proof;
#[cfg(feature = "canon")]
pub mod score_gen;
#[cfg(feature = "canon")]
pub mod tree;
pub mod utils;

/*


/// BlindBidCircuit instance
pub use proof::BlindBidCircuit;
*/
