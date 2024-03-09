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
const MOD: usize = 1_000_000_007;
const INF: isize = 2_000_000_000;
pub type Graph = Vec<Vec<usize>>;

#[fastout]
fn main() {
  input! {
     mut c: String,
  }
  let mut s = "".to_string();
  let mut key_s = 0;
  let mut key_e = 0;
  for i in 0..c.len() {
    if c.chars().nth(i).unwrap() == '|' {
      key_s = i;
      break;
    }
  }
  for i in key_s+1..c.len() {
    if c.chars().nth(i).unwrap() == '|' {
      key_e = i;
      break;
    }
  }
  for i in 0..c.len() {
    if i >= key_s && i <= key_e  {
      continue;
    }
    s.push(c.chars().nth(i).unwrap());
  }
  println!("{}", s);
}
