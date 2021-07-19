#![no_main]
use itertools::Itertools;
use libfuzzer_sys::arbitrary::{self, Arbitrary};
use libfuzzer_sys::fuzz_target;
use ordered_multimap::ListOrderedMultimap;

#[derive(Arbitrary, Debug)]
enum Capacity {
    New,
    WithCapacity { k: usize, v: usize },
}

// #[derive(Arbitrary, Debug)]
// enum KeyType {
//     Normal,
//     RandomEq { seed: u64 },
//     RandomHash { seed: u64 },
//     RandomHashEq { seed: u64 },
// }

#[derive(Arbitrary, Debug)]
enum Action {
    PackToFit,
    Insert(u64),
}

#[derive(Arbitrary, Debug)]
struct Setup {
    // k: KeyType,
    seed: u64,
    p_duplicate_key: u8,
    capacity: Capacity,
    actions: Vec<Action>,
}

thread_local! {
    // TODO thread local rng
}

const K: usize = 1024;
const M: usize = K * K;

fn fuzz(data: &Setup) {
    // TODO seed rng

    let mut counter = {
        let mut curr = 0u64;
        move || {
            let out = curr;
            curr += 1;
            out
        }
    };

    let mut map: ListOrderedMultimap<u64, u64> = match data.capacity {
        Capacity::New => ListOrderedMultimap::new(),
        Capacity::WithCapacity { k, v } => ListOrderedMultimap::with_capacity(k % M, v % M),
    };
    let mut keys: Vec<u64> = Vec::new();

    for action in &data.actions {
        match *action {
            Action::PackToFit => {
                map.pack_to_fit();
            }
            Action::Insert(k) => {
                keys.push(k);
                map.insert(k, counter());
            }
        }
    }

    assert!(map.keys().eq(keys.iter().unique()));
}

fuzz_target!(|data: Setup| {
    fuzz(&data)
    // match data.k {
    //     KeyType::Normal => fuzz::<u64>(&data),
    //     KeyType::RandomEq { seed } => fuzz::<RandomEq>(&data),
    //     KeyType::RandomHash { seed } => fuzz::<RandomHash>(&data),
    //     KeyType::RandomHashEq { seed } => fuzz::<RandomHashEq>(&data),
    // }
});
