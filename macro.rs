#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_macros)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

// 整数１つ読み込み(macro)
macro_rules! read {
    ($($t:ty),*) => {
        {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
            let mut iter = input.split_whitespace();
            ($(iter.next().unwrap().parse::<$t>().unwrap()),*)
        }
    };
}

// 文字列１つ読み込み(macro)
macro_rules! read_str {
    () => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        input.trim().to_string()
    }};
}

// 整数の１次元ベクトル読み込み(macro)
macro_rules! read_vec {
    ($t:ty) => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    }};
}

// 文字列の１次元ベクトル読み込み(macro)
macro_rules! read_str_vec {
    () => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        input
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }};
}

// 整数の２次元ベクトル読み込み(macro)
// 2次元ベクトル読み込み(macro)
macro_rules! read_2d_vec {
    ($t:ty, $rows:expr, $cols:expr) => {{
        let mut v = Vec::with_capacity($rows);
        for _ in 0..$rows {
            v.push(read_vec!($t));
        }
        v
    }};
}

// charの２次元ベクトル読み込み(macro)
macro_rules! read_2d_char_vec {
    ($rows:expr, $cols:expr) => {{
        let mut v = Vec::with_capacity($rows);
        for _ in 0..$rows {
            v.push(read_char_vec!());
        }
        v
    }};
}

macro_rules! read_vecdeque {
    ($t:ty) => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let mut iter = input.split_whitespace();
        let mut vecdeque = VecDeque::new();
        while let Some(token) = iter.next() {
            if let Ok(num) = token.parse::<$t>() {
                vecdeque.push_back(num);
            } else {
                eprintln!("Invalid number: {}", token);
            }
        }
        vecdeque
    }};
}

// バイト列１つ読み込み(macro)
macro_rules! read_bytes {
    () => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        input.trim().as_bytes().to_vec()
    }};
}

fn chmin<T: PartialOrd>(a: &mut T, b: T) -> bool {
    if *a > b {
        *a = b;
        true
    } else {
        false
    }
}

fn chmax<T: PartialOrd>(a: &mut T, b: T) -> bool {
    if *a < b {
        *a = b;
        true
    } else {
        false
    }
}

// 文字の１次元ベクトル読み込み(macro)
macro_rules! read_char_vec {
    () => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        input
            .trim()
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
    }};
}

macro_rules! print_space_separated {
    ($coll:expr) => {
        println!(
            "{}",
            $coll
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    };
}

macro_rules! print_space_separated_hashmap {
    ($map:expr) => {
        println!(
            "{}",
            $map.iter()
                .map(|(k, v)| format!("{} {}", k, v))
                .collect::<Vec<String>>()
                .join(" ")
        );
    };
}

const MIN_USIZE: usize = std::usize::MIN;

const MAX_USIZE: usize = std::usize::MAX;

const MIN_ISIZE: isize = std::isize::MIN;

const MAX_ISIZE: isize = std::isize::MAX;

// macro gcd
macro_rules! gcd {
    ($a:expr, $b:expr) => {{
        let mut a = $a;
        let mut b = $b;
        while b > 0 {
            let tmp = b;
            b = a % b;
            a = tmp;
        }
        a
    }};
}

// macro lcm
macro_rules! lcm {
    ($a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        a / gcd!(a, b) * b
    }};
}

// macro mod_pow
macro_rules! mod_pow {
    ($a:expr, $b:expr, $m:expr) => {{
        let mut a = $a;
        let mut b = $b;
        let m = $m;
        let mut ret = 1;
        while b > 0 {
            if b & 1 == 1 {
                ret = ret * a % m;
            }
            a = a * a % m;
            b >>= 1;
        }
        ret
    }};
}

// macro mod_comb
macro_rules! mod_comb {
    ($n:expr, $k:expr, $m:expr) => {{
        let n = $n;
        let k = $k;
        let m = $m;
        let mut ret = 1;
        for i in 0..k {
            ret = ret * (n - i) % m;
            ret = ret * mod_pow!(i + 1, m - 2, m) % m;
        }
        ret
    }};
}

// macro mod_fact
macro_rules! mod_fact {
    ($n:expr, $m:expr) => {{
        let n = $n;
        let m = $m;
        let mut ret = 1;
        for i in 1..n + 1 {
            ret = ret * i % m;
        }
        ret
    }};
}

// macro mod_inv
macro_rules! mod_inv {
    ($a:expr, $m:expr) => {{
        let mut a = $a;
        let m = $m;
        let mut b = m;
        let mut u = 1;
        let mut v = 0;
        while b > 0 {
            let t = a / b;
            a -= t * b;
            std::mem::swap(&mut a, &mut b);
            u -= t * v;
            std::mem::swap(&mut u, &mut v);
        }
        u %= m;
        if u < 0 {
            u += m;
        }
        u
    }};
}

// 縦に整数の読み込み（引数に型と個数）
macro_rules! read_vec_vertical {
    ($t:ty, $n:expr) => {{
        let mut v = Vec::with_capacity($n);
        for _ in 0..$n {
            v.push(read!($t));
        }
        v
    }};
}

// 縦に文字列の読み込み（引数に個数）
macro_rules! read_str_vec_vertical {
    ($n:expr) => {{
        let mut v = Vec::with_capacity($n);
        for _ in 0..$n {
            v.push(read_str!());
        }
        v
    }};
}

// 縦にbyte型の読み込み（引数に個数）
macro_rules! read_bytes_vec_vertical {
    ($n:expr) => {{
        let mut v = Vec::with_capacity($n);
        for _ in 0..$n {
            v.push(read_bytes!());
        }
        v
    }};
}

const MOD: usize = 1_000_000_007;
const INF: isize = 2_000_000_000;

pub type Graph = Vec<Vec<usize>>;

#[cfg(target_pointer_width = "64")]
pub type fsize = f64;
#[cfg(target_pointer_width = "32")]
pub type fsize = f32;

fn main() {
    let (h, w) = read!(usize, usize);
    let a = read_2d_vec!(usize, h, w);

    let mut row = vec![0; h];
    let mut col = vec![0; w];

    for i in 0..h {
        for j in 0..w {
            row[i] += a[i][j];
            col[j] += a[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", row[i] + col[j] - a[i][j]);
            if j != w - 1 {
                print!(" ");
            }
        }
        println!();
    }
}


pub use __cargo_equip::prelude::*;

use acl_dsu::Dsu;
use acl_fenwicktree::FenwickTree;
use acl_lazysegtree::LazySegtree;
use acl_modint::{ModInt1000000007, ModInt998244353};
use acl_segtree::{Additive, Max, Min, Multiplicative, Segtree};
use fixedbitset::FixedBitSet;
use itertools::Itertools;
use my_atcoder_library::*;
use num_integer::*;
use num_traits::clamp;
use permutohedron::factorial;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::line::LineSource,
};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    f64::{self, consts::PI},
    io::{self, BufReader},
    iter::FromIterator,
    mem::swap,
};
use superslice::Ext;

#[macro_export]
macro_rules! max {
        ($x: expr) => ($x);
        ($x: expr, $( $y: expr ),+) => {
            std::cmp::max($x, max!($( $y ),+))
        }
    }
#[macro_export]
macro_rules! min {
        ($x: expr) => ($x);
        ($x: expr, $( $y: expr ),+) => {
            std::cmp::min($x, min!($( $y ),+))
        }
    }
#[macro_export]
macro_rules! abs {
    ($x: expr) => {
        if $x < 0_isize {
            $x * (-1)
        } else {
            $x
        }
    };
}
#[macro_export]
macro_rules! absf {
    ($x: expr) => {
        if $x < 0.0 {
            $x * (-1.0)
        } else {
            $x
        }
    };
}
// 整数１つ読み込み(macro)
macro_rules! read {
    ($($t:ty),*) => {
        {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).ok();
            let mut iter = input.split_whitespace();
            ($(iter.next().unwrap().parse::<$t>().unwrap()),*)
        }
    };
}

// 文字列１つ読み込み(macro)
macro_rules! read_str {
    () => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        input.trim().to_string()
    }};
}

// 整数の１次元ベクトル読み込み(macro)
macro_rules! read_vec {
    ($t:ty) => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    }};
}

// 文字列の１次元ベクトル読み込み(macro)
macro_rules! read_str_vec {
    () => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        input
            .trim()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
    }};
}

// 整数の２次元ベクトル読み込み(macro)
// 2次元ベクトル読み込み(macro)
macro_rules! read_2d_vec {
    ($t:ty, $rows:expr, $cols:expr) => {{
        let mut v = Vec::with_capacity($rows);
        for _ in 0..$rows {
            v.push(read_vec!($t));
        }
        v
    }};
}

// charの２次元ベクトル読み込み(macro)
macro_rules! read_2d_char_vec {
    ($rows:expr, $cols:expr) => {{
        let mut v = Vec::with_capacity($rows);
        for _ in 0..$rows {
            v.push(read_char_vec!());
        }
        v
    }};
}

macro_rules! read_vecdeque {
    ($t:ty) => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        let mut iter = input.split_whitespace();
        let mut vecdeque = VecDeque::new();
        while let Some(token) = iter.next() {
            if let Ok(num) = token.parse::<$t>() {
                vecdeque.push_back(num);
            } else {
                eprintln!("Invalid number: {}", token);
            }
        }
        vecdeque
    }};
}

// バイト列１つ読み込み(macro)
macro_rules! read_bytes {
    () => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        input.trim().as_bytes().to_vec()
    }};
}

fn chmin<T: PartialOrd>(a: &mut T, b: T) -> bool {
    if *a > b {
        *a = b;
        true
    } else {
        false
    }
}

fn chmax<T: PartialOrd>(a: &mut T, b: T) -> bool {
    if *a < b {
        *a = b;
        true
    } else {
        false
    }
}

// 文字の１次元ベクトル読み込み(macro)
macro_rules! read_char_vec {
    () => {{
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).ok();
        input
            .trim()
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
    }};
}

macro_rules! print_space_separated {
    ($coll:expr) => {
        println!(
            "{}",
            $coll
                .iter()
                .map(|&x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    };
}

macro_rules! print_space_separated_hashmap {
    ($map:expr) => {
        println!(
            "{}",
            $map.iter()
                .map(|(k, v)| format!("{} {}", k, v))
                .collect::<Vec<String>>()
                .join(" ")
        );
    };
}

const MIN_USIZE: usize = std::usize::MIN;

const MAX_USIZE: usize = std::usize::MAX;

const MIN_ISIZE: isize = std::isize::MIN;

const MAX_ISIZE: isize = std::isize::MAX;

// macro gcd
macro_rules! gcd {
    ($a:expr, $b:expr) => {{
        let mut a = $a;
        let mut b = $b;
        while b > 0 {
            let tmp = b;
            b = a % b;
            a = tmp;
        }
        a
    }};
}

// macro lcm
macro_rules! lcm {
    ($a:expr, $b:expr) => {{
        let a = $a;
        let b = $b;
        a / gcd!(a, b) * b
    }};
}

// macro mod_pow
macro_rules! mod_pow {
    ($a:expr, $b:expr, $m:expr) => {{
        let mut a = $a;
        let mut b = $b;
        let m = $m;
        let mut ret = 1;
        while b > 0 {
            if b & 1 == 1 {
                ret = ret * a % m;
            }
            a = a * a % m;
            b >>= 1;
        }
        ret
    }};
}

// macro mod_comb
macro_rules! mod_comb {
    ($n:expr, $k:expr, $m:expr) => {{
        let n = $n;
        let k = $k;
        let m = $m;
        let mut ret = 1;
        for i in 0..k {
            ret = ret * (n - i) % m;
            ret = ret * mod_pow!(i + 1, m - 2, m) % m;
        }
        ret
    }};
}

// macro mod_fact
macro_rules! mod_fact {
    ($n:expr, $m:expr) => {{
        let n = $n;
        let m = $m;
        let mut ret = 1;
        for i in 1..n + 1 {
            ret = ret * i % m;
        }
        ret
    }};
}

// macro mod_inv
macro_rules! mod_inv {
    ($a:expr, $m:expr) => {{
        let mut a = $a;
        let m = $m;
        let mut b = m;
        let mut u = 1;
        let mut v = 0;
        while b > 0 {
            let t = a / b;
            a -= t * b;
            std::mem::swap(&mut a, &mut b);
            u -= t * v;
            std::mem::swap(&mut u, &mut v);
        }
        u %= m;
        if u < 0 {
            u += m;
        }
        u
    }};
}

// 縦に整数の読み込み（引数に型と個数）
macro_rules! read_vec_vertical {
    ($t:ty, $n:expr) => {{
        let mut v = Vec::with_capacity($n);
        for _ in 0..$n {
            v.push(read!($t));
        }
        v
    }};
}

// 縦に文字列の読み込み（引数に個数）
macro_rules! read_str_vec_vertical {
    ($n:expr) => {{
        let mut v = Vec::with_capacity($n);
        for _ in 0..$n {
            v.push(read_str!());
        }
        v
    }};
}

// 縦にbyte型の読み込み（引数に個数）
macro_rules! read_bytes_vec_vertical {
    ($n:expr) => {{
        let mut v = Vec::with_capacity($n);
        for _ in 0..$n {
            v.push(read_bytes!());
        }
        v
    }};
}
fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}