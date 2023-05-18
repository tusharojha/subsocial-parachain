
//! Autogenerated weights for pallet_post_follows
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-15, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `benchmarks-ci`, CPU: `Intel(R) Xeon(R) CPU E5-2697A v4 @ 2.60GHz`
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
    // pallet_post_follows
    // --extrinsic
    // *
    // --steps
    // 50
    // --repeat
    // 20
    // --heap-pages
    // 4096
    // --output
    // pallets/post-follows/src/weights.rs
    // --template
    // ./.maintain/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(non_snake_case)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_post_follows.
pub trait WeightInfo {
    fn follow_post() -> Weight;
    fn unfollow_post() -> Weight;
}

/// Weights for pallet_post_follows using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
        impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
            // Storage: PostFollows PostFollowedByAccount (r:1 w:1)
            // Storage: Posts PostById (r:1 w:0)
            // Storage: PostFollows PostFollowers (r:1 w:1)
            // Storage: PostFollows PostsFollowedByAccount (r:1 w:1)
        fn follow_post() -> Weight {
        // Minimum execution time: 53_831 nanoseconds.
        Weight::from_ref_time(59_835_000)
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
        }
            // Storage: Posts PostById (r:1 w:0)
            // Storage: PostFollows PostFollowedByAccount (r:1 w:1)
            // Storage: PostFollows PostsFollowedByAccount (r:1 w:1)
            // Storage: PostFollows PostFollowers (r:1 w:1)
        fn unfollow_post() -> Weight {
        // Minimum execution time: 61_978 nanoseconds.
        Weight::from_ref_time(62_615_000)
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(3))
        }
    }

    // For backwards compatibility and tests
    impl WeightInfo for () {
            // Storage: PostFollows PostFollowedByAccount (r:1 w:1)
            // Storage: Posts PostById (r:1 w:0)
            // Storage: PostFollows PostFollowers (r:1 w:1)
            // Storage: PostFollows PostsFollowedByAccount (r:1 w:1)
        fn follow_post() -> Weight {
        // Minimum execution time: 53_831 nanoseconds.
        Weight::from_ref_time(59_835_000)
            .saturating_add(RocksDbWeight::get().reads(4))
            .saturating_add(RocksDbWeight::get().writes(3))
        }
            // Storage: Posts PostById (r:1 w:0)
            // Storage: PostFollows PostFollowedByAccount (r:1 w:1)
            // Storage: PostFollows PostsFollowedByAccount (r:1 w:1)
            // Storage: PostFollows PostFollowers (r:1 w:1)
        fn unfollow_post() -> Weight {
        // Minimum execution time: 61_978 nanoseconds.
        Weight::from_ref_time(62_615_000)
            .saturating_add(RocksDbWeight::get().reads(4))
            .saturating_add(RocksDbWeight::get().writes(3))
        }
    }