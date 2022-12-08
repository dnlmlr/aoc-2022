#![feature(portable_simd)]
#![feature(array_chunks)]
#![feature(iter_array_chunks)]
#![feature(core_intrinsics)]
#![feature(split_array)]
#![feature(array_windows)]
#![feature(slice_internals)]

use aoc_runner_derive::aoc_lib;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

aoc_lib! { year = 2022 }
