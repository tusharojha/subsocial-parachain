// Copyright (C) DAPPFORCE PTE. LTD., dappforce@gmail.com.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0.

// Full notice is available at https://github.com/dappforce/subsocial-parachain/blob/main/HEADER-GPL3. 
// Full license is available at https://github.com/dappforce/subsocial-parachain/blob/main/LICENSE.


//! Autogenerated weights for pallet_spaces
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `benchmarks-ci`, CPU: `Intel(R) Xeon(R) Platinum 8280 CPU @ 2.70GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
    // ./scripts/../target/release/subsocial-collator
    // benchmark
    // pallet
    // --chain
    // dev
    // --execution
    // wasm
    // --wasm-execution
    // Compiled
    // --pallet
    // pallet_spaces
    // --extrinsic
    // *
    // --steps
    // 50
    // --repeat
    // 20
    // --heap-pages
    // 4096
    // --output
    // pallets/spaces/src/weights.rs
    // --template
    // ./.maintain/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_spaces.
pub trait WeightInfo {
    fn create_space() -> Weight;
    fn update_space() -> Weight;
}

/// Weights for pallet_spaces using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
        impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
            // Storage: Spaces SpaceIdsByOwner (r:1 w:1)
            // Storage: Spaces NextSpaceId (r:1 w:1)
            // Storage: Timestamp Now (r:1 w:0)
            // Storage: Spaces SpaceById (r:0 w:1)
        fn create_space() -> Weight {
        // Minimum execution time: 45_683 nanoseconds.
        Weight::from_ref_time(46_598_000)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(3))
        }
            // Storage: Spaces SpaceById (r:1 w:1)
            // Storage: SpaceFollows SpaceFollowedByAccount (r:1 w:0)
        fn update_space() -> Weight {
        // Minimum execution time: 52_466 nanoseconds.
        Weight::from_ref_time(53_333_000)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
        }
    }

    // For backwards compatibility and tests
    impl WeightInfo for () {
            // Storage: Spaces SpaceIdsByOwner (r:1 w:1)
            // Storage: Spaces NextSpaceId (r:1 w:1)
            // Storage: Timestamp Now (r:1 w:0)
            // Storage: Spaces SpaceById (r:0 w:1)
        fn create_space() -> Weight {
        // Minimum execution time: 45_683 nanoseconds.
        Weight::from_ref_time(46_598_000)
            .saturating_add(RocksDbWeight::get().reads(3))
            .saturating_add(RocksDbWeight::get().writes(3))
        }
            // Storage: Spaces SpaceById (r:1 w:1)
            // Storage: SpaceFollows SpaceFollowedByAccount (r:1 w:0)
        fn update_space() -> Weight {
        // Minimum execution time: 52_466 nanoseconds.
        Weight::from_ref_time(53_333_000)
            .saturating_add(RocksDbWeight::get().reads(2))
            .saturating_add(RocksDbWeight::get().writes(1))
        }
    }
