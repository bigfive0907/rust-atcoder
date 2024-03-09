#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_macros)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_must_use)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_mut)]
#![allow(unused_imports)]

//use acl_dsu::Dsu;
//use acl_fenwicktree::FenwickTree;
//use acl_lazysegtree::LazySegtree;
//use acl_modint::{ModInt1000000007, ModInt998244353};
//use acl_segtree::{Additive, Max, Min, Multiplicative, Segtree};
//use my_atcoder_library::*;

use fixedbitset::FixedBitSet;
use itertools::Itertools;

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

const MOD: usize = 1_000_000_007;
const INF: isize = 2_000_000_000;

pub type Graph = Vec<Vec<usize>>;

#[cfg(target_pointer_width = "64")]
pub type fsize = f64;
#[cfg(target_pointer_width = "32")]
pub type fsize = f32;

#[fastout]
fn main() {
  input! {
      h: usize,
      w: usize,
      n: usize,
      t: Chars,
      mut s: [Chars;h],
  }
  
  // let mut s = vec![vec!['#'; w]; h];
  /* for i in 0..h {
    input! {
        mut s: [[char; w];h],
    }
    for j in 0..w {
      let b = a.chars().nth(j).unwrap();
      s[i][j] = b;
    }
  } */

  let mut count = 0;
  for i in 1..h {
    for j in 1..w {
      let mut y: usize = i;
      let mut x: usize = j;
      let mut flag = true;
      if x == 0 || x > w || y == 0 || y > h || s[y][x] == '#' {
        continue;
      }
      for &c in &t {
        match c {
            'L' => {
                x -= 1;
            },
            'R' => {
                x += 1;
            },
            'U' => {
                y -= 1;
            },
            'D' => {
                y += 1;
            },
            _ => { panic!() }
        }
        if x == 0 || x > w || y == 0 || y > h || s[y][x] == '#' {
          flag = false;
          break;
        }
      }

      if flag {
        count += 1;
      }
    }
  }
  println!("{}", count);
}
