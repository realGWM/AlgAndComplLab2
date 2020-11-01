use rand::rngs::ThreadRng;
use rand::distributions::{Distribution, Uniform};
use std::time::Instant;
use std::fs::File;
use std::io::prelude::*;
use std::io;

const TIMES: usize = 2_000;       //how many times do we test each size in order to minimize statistical error
const MIN:   usize = 0;           //min array size
const MAX:   usize = 100_000;     //max array size
const STEP:  usize = 1_000;       //step for incrementing array size

fn main() {
    let program_start = Instant::now();

    let mut rng = rand::thread_rng();

    //Those are functions (closures) calculating a range for a given size
    let normal_dist = |size: usize| Uniform::new_inclusive(     -(size as i32), size as i32);
    let wide_dist   = |size: usize| Uniform::new_inclusive(-(size as i32) * 10, (size as i32) * 10);
    let narrow_dist = |size: usize| Uniform::new_inclusive(                  0, (size as i32) / 2);

    println!("Testing [-n;n]...");
    let (normal_sizes, normal_totals) = worker(MIN, MAX, STEP, &mut rng, normal_dist);
    println!("Testing [-10n;10n]...");
    let (wide_sizes,   wide_totals)   = worker(MIN, MAX, STEP, &mut rng, wide_dist);
    println!("Testing [0;n/2]...");
    let (narrow_sizes, narrow_totals) = worker(MIN, MAX, STEP, &mut rng, narrow_dist);

    //Sizes are actually the same everytime, so they could be saved just one time, but I'm kinda lazy...
    save_results(&normal_sizes, &normal_totals, "../r/normal_sizes.txt", "../r/normal_totals.txt");
    save_results(&wide_sizes, &wide_totals, "../r/wide_sizes.txt", "../r/wide_totals.txt");
    save_results(&narrow_sizes, &narrow_totals, "../r/narrow_sizes.txt", "../r/narrow_totals.txt");

    let program_end = Instant::now();
    let time_taken = program_end.duration_since(program_start).as_secs();
    println!("Program has been running for {} seconds!..", time_taken);
}

//TODO: replace (min, max, step) with some kind of a range or an iterator?..
//F is a function that calculates a range of random values for a given array size
//Returns 2 vectors to be processed by the R language:
//A vector of sizes
//A vector of running times for each size
fn worker<F>(min: usize, max: usize, step: usize, rng: &mut ThreadRng, f: F) -> (Vec<usize>, Vec<u128>)
    where F: Fn(usize) -> Uniform<i32>
{
    //Compiler is bad. Compiler is REALLY BAD. Compiler optimizes-out the loop completely without this. Don't be like compiler.
    let mut useless_variable: usize = 0;

    let mut sizes: Vec<usize> = Vec::new();
    let mut totals: Vec<u128> = Vec::new();

    for size in (min..=max).step_by(step) {        
        let dist = f(size);
        let mut total: u128 = 0;
        let mut haystack: Vec<i32> = vec![0; size];
        
        for _ in 0..TIMES {

            for i in haystack.iter_mut() {
                *i = dist.sample(rng);
            }

            let needle = dist.sample(rng);

            let start = Instant::now();

            for value in haystack.iter() {
                if needle == *value {
                    useless_variable += 1;
                    break;
                }
            }

            let end = Instant::now();
            let time_taken = end.duration_since(start).as_nanos();
            total += time_taken;
        }

        total /= TIMES as u128;
        println!("size = {}, total = {}", size, total);
        sizes.push(size);
        totals.push(total);
    }

    println!("Garbage data with the only purpose to fight the compiler which is too good at optimizing-out things: {}", useless_variable);

    (sizes, totals)
}

//Save results to a file and print them to stdout
fn save_results(sizes: &[usize], totals: &[u128], sizes_file_name: &str, totals_file_name: &str) {
    let mut sizes_string = sizes.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" ");
    let mut totals_string = totals.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(" ");
    sizes_string.push('\n');
    totals_string.push('\n');
    io::stdout().write_all(sizes_string.as_bytes()).unwrap();
    io::stdout().write_all(totals_string.as_bytes()).unwrap();
    let mut sizes_file = File::create(sizes_file_name).unwrap();
    let mut totals_file = File::create(totals_file_name).unwrap();
    sizes_file.write_all(sizes_string.as_bytes()).unwrap();
    totals_file.write_all(totals_string.as_bytes()).unwrap();
}