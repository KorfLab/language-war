use rand::prelude::*;
use rand::rngs::Xoshiro128PlusPlus;
use std::collections::HashMap;
use std::thread;

pub fn count_genotypes(
    iterations: usize,
    depth: usize,
    err_rate: f64,
) -> (HashMap<String, usize>, HashMap<String, usize>) {
    let mut homo_count = HashMap::<String, usize>::new();
    let mut hetero_count = HashMap::<String, usize>::new();

    let mut rng: Xoshiro128PlusPlus = rand::make_rng();

    for _ in 0..iterations {
        // 0: A, 1: C, 2: G, 3: T
        let mut homo = [0; 4];
        for _ in 0..depth {
            if rng.random_range(0.0..1.0) < err_rate {
                let nucleotide: usize = rng.random_range(1..=3);
                homo[nucleotide] += 1;
            } else {
                homo[0] += 1;
            }
        }
        homo.sort_unstable_by(|a, b| b.cmp(a));
        let sig = format!("{}.{}.{}.{}", homo[0], homo[1], homo[2], homo[3]);
        *homo_count.entry(sig).or_insert(0) += 1;
    }

    for _ in 0..iterations {
        // 0: A, 1: C, 2: G, 3: T
        let mut hetero = [0; 4];
        for _ in 0..depth {
            let nucleotide: usize = if rng.random_range(0.0..1.0) < 0.5 {
                // from mom: 'A'
                if rng.random_range(0.0..1.0) < err_rate {
                    // error: choose CGT
                    rng.random_range(1..=3)
                } else {
                    0
                }
            } else {
                // from dad: 'T'
                if rng.random_range(0.0..1.0) < err_rate {
                    // error: choose ACG
                    rng.random_range(0..=2)
                } else {
                    3
                }
            };
            hetero[nucleotide] += 1;
        }
        hetero.sort_unstable_by(|a, b| b.cmp(a));
        let sig = format!("{}.{}.{}.{}", hetero[0], hetero[1], hetero[2], hetero[3]);
        *hetero_count.entry(sig).or_insert(0) += 1;
    }

    (homo_count, hetero_count)
}

pub fn count_genotypes_parallel(
    iterations: usize,
    depth: usize,
    err_rate: f64,
    threads: usize,
) -> (HashMap<String, usize>, HashMap<String, usize>) {
    let mut homo_count = HashMap::<String, usize>::new();
    let mut hetero_count = HashMap::<String, usize>::new();

    let iter_each = iterations / threads;
    let iter_rem = iterations % threads;

    let mut join_handles = Vec::new();

    for _ in 0..threads - 1 {
        join_handles.push(thread::spawn(move || {
            count_genotypes(iter_each, depth, err_rate)
        }));
    }

    join_handles.push(thread::spawn(move || {
        count_genotypes(iter_each + iter_rem, depth, err_rate)
    }));

    for handle in join_handles {
        let (homo, hetero) = handle.join().unwrap();
        for (key, value) in homo {
            *homo_count.entry(key).or_insert(0) += value;
        }
        for (key, value) in hetero {
            *hetero_count.entry(key).or_insert(0) += value;
        }
    }

    (homo_count, hetero_count)
}
