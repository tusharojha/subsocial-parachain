// Copyright (C) DAPPFORCE PTE. LTD., dappforce@gmail.com.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0.

// Full notice is available at https://github.com/dappforce/subsocial-parachain/blob/main/HEADER-GPL3. 
// Full license is available at https://github.com/dappforce/subsocial-parachain/blob/main/LICENSE.

use frame_support::pallet_prelude::*;

use subsocial_support::SpaceId;

use crate::mock::*;
use crate::utils::{ACCOUNT2, SPACE1};

/// Account 2 follows Space 1
pub(crate) fn _default_follow_space() -> DispatchResult {
    _follow_space(None, None)
}

pub(crate) fn _follow_space(origin: Option<RuntimeOrigin>, space_id: Option<SpaceId>) -> DispatchResult {
    SpaceFollows::follow_space(
        origin.unwrap_or_else(|| RuntimeOrigin::signed(ACCOUNT2)),
        space_id.unwrap_or(SPACE1),
    )
}

pub(crate) fn _default_unfollow_space() -> DispatchResult {
    _unfollow_space(None, None)
}

pub(crate) fn _unfollow_space(origin: Option<RuntimeOrigin>, space_id: Option<SpaceId>) -> DispatchResult {
    SpaceFollows::unfollow_space(
        origin.unwrap_or_else(|| RuntimeOrigin::signed(ACCOUNT2)),
        space_id.unwrap_or(SPACE1),
    )
}
