// Copyright (C) DAPPFORCE PTE. LTD., dappforce@gmail.com.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0.

// Full notice is available at https://github.com/dappforce/subsocial-parachain/blob/main/HEADER-GPL3. 
// Full license is available at https://github.com/dappforce/subsocial-parachain/blob/main/LICENSE.


//! Autogenerated weights for pallet_domains
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
    // pallet_domains
    // --extrinsic
    // *
    // --steps
    // 50
    // --repeat
    // 20
    // --heap-pages
    // 4096
    // --output
    // pallets/domains/src/weights.rs
    // --template
    // ./.maintain/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_domains.
pub trait WeightInfo {
    fn register_domain() -> Weight;
    fn force_register_domain() -> Weight;
    fn set_inner_value() -> Weight;
    fn force_set_inner_value() -> Weight;
    fn set_outer_value() -> Weight;
    fn set_domain_content() -> Weight;
    fn reserve_words(s: u32, ) -> Weight;
    fn support_tlds(s: u32, ) -> Weight;
}

/// Weights for pallet_domains using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
        impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
            // Storage: Domains DomainsByOwner (r:1 w:1)
            // Storage: Domains ReservedWords (r:1 w:0)
            // Storage: Domains RegisteredDomains (r:1 w:1)
            // Storage: Timestamp Now (r:1 w:0)
        fn register_domain() -> Weight {
        // Minimum execution time: 67_660 nanoseconds.
        Weight::from_ref_time(68_600_000)
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(2))
        }
            // Storage: Domains DomainsByOwner (r:1 w:1)
            // Storage: Domains RegisteredDomains (r:1 w:1)
            // Storage: Timestamp Now (r:1 w:0)
        fn force_register_domain() -> Weight {
        // Minimum execution time: 45_166 nanoseconds.
        Weight::from_ref_time(46_046_000)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
        }
            // Storage: Domains RegisteredDomains (r:1 w:1)
            // Storage: Domains DomainByInnerValue (r:0 w:2)
        fn set_inner_value() -> Weight {
        // Minimum execution time: 45_544 nanoseconds.
        Weight::from_ref_time(46_048_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(3))
        }
            // Storage: Domains RegisteredDomains (r:1 w:1)
            // Storage: Domains DomainByInnerValue (r:0 w:2)
        fn force_set_inner_value() -> Weight {
        // Minimum execution time: 44_561 nanoseconds.
        Weight::from_ref_time(45_635_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(3))
        }
            // Storage: Domains RegisteredDomains (r:1 w:1)
        fn set_outer_value() -> Weight {
        // Minimum execution time: 57_639 nanoseconds.
        Weight::from_ref_time(58_498_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
        }
            // Storage: Domains RegisteredDomains (r:1 w:1)
        fn set_domain_content() -> Weight {
        // Minimum execution time: 38_702 nanoseconds.
        Weight::from_ref_time(38_974_000)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
        }
            // Storage: Domains ReservedWords (r:0 w:1)
            /// The range of component `s` is `[1, 2860]`.
        fn reserve_words(s: u32, ) -> Weight {
        // Minimum execution time: 24_683 nanoseconds.
        Weight::from_ref_time(153_633_074)
            // Standard Error: 6_880
            .saturating_add(Weight::from_ref_time(2_532_838).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().writes(313))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
        }
            // Storage: Domains SupportedTlds (r:0 w:1)
            /// The range of component `s` is `[1, 2860]`.
        fn support_tlds(s: u32, ) -> Weight {
        // Minimum execution time: 24_766 nanoseconds.
        Weight::from_ref_time(154_478_755)
            // Standard Error: 6_885
            .saturating_add(Weight::from_ref_time(2_495_524).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().writes(313))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
        }
    }

    // For backwards compatibility and tests
    impl WeightInfo for () {
            // Storage: Domains DomainsByOwner (r:1 w:1)
            // Storage: Domains ReservedWords (r:1 w:0)
            // Storage: Domains RegisteredDomains (r:1 w:1)
            // Storage: Timestamp Now (r:1 w:0)
        fn register_domain() -> Weight {
        // Minimum execution time: 67_660 nanoseconds.
        Weight::from_ref_time(68_600_000)
            .saturating_add(RocksDbWeight::get().reads(4))
            .saturating_add(RocksDbWeight::get().writes(2))
        }
            // Storage: Domains DomainsByOwner (r:1 w:1)
            // Storage: Domains RegisteredDomains (r:1 w:1)
            // Storage: Timestamp Now (r:1 w:0)
        fn force_register_domain() -> Weight {
        // Minimum execution time: 45_166 nanoseconds.
        Weight::from_ref_time(46_046_000)
            .saturating_add(RocksDbWeight::get().reads(3))
            .saturating_add(RocksDbWeight::get().writes(2))
        }
            // Storage: Domains RegisteredDomains (r:1 w:1)
            // Storage: Domains DomainByInnerValue (r:0 w:2)
        fn set_inner_value() -> Weight {
        // Minimum execution time: 45_544 nanoseconds.
        Weight::from_ref_time(46_048_000)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(3))
        }
            // Storage: Domains RegisteredDomains (r:1 w:1)
            // Storage: Domains DomainByInnerValue (r:0 w:2)
        fn force_set_inner_value() -> Weight {
        // Minimum execution time: 44_561 nanoseconds.
        Weight::from_ref_time(45_635_000)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(3))
        }
            // Storage: Domains RegisteredDomains (r:1 w:1)
        fn set_outer_value() -> Weight {
        // Minimum execution time: 57_639 nanoseconds.
        Weight::from_ref_time(58_498_000)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
        }
            // Storage: Domains RegisteredDomains (r:1 w:1)
        fn set_domain_content() -> Weight {
        // Minimum execution time: 38_702 nanoseconds.
        Weight::from_ref_time(38_974_000)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
        }
            // Storage: Domains ReservedWords (r:0 w:1)
            /// The range of component `s` is `[1, 2860]`.
        fn reserve_words(s: u32, ) -> Weight {
        // Minimum execution time: 24_683 nanoseconds.
        Weight::from_ref_time(153_633_074)
            // Standard Error: 6_880
            .saturating_add(Weight::from_ref_time(2_532_838).saturating_mul(s.into()))
            .saturating_add(RocksDbWeight::get().writes(313))
            .saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(s.into())))
        }
            // Storage: Domains SupportedTlds (r:0 w:1)
            /// The range of component `s` is `[1, 2860]`.
        fn support_tlds(s: u32, ) -> Weight {
        // Minimum execution time: 24_766 nanoseconds.
        Weight::from_ref_time(154_478_755)
            // Standard Error: 6_885
            .saturating_add(Weight::from_ref_time(2_495_524).saturating_mul(s.into()))
            .saturating_add(RocksDbWeight::get().writes(313))
            .saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(s.into())))
        }
    }
