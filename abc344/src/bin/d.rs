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
    fastout,
    input,
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
        t:Chars,
        n:usize,
    }
    let mut vec = vec![];
    for i in 0 ..n {
        input! {
            m:usize,
            v:[String;m],
        }
        vec.push(v);
    }
    let mut map = HashMap::new();
    let mut vv = vec![];
    let mut count = 1;
    // vvの基礎をつくる
    // vvに文字列を追加していく
    //dpで解く
    //i番目までの文字列を選ぶ,選ばない。i+1番目の文字列を選ぶ,選ばない。i+2番目の...の組み合わせ
    //vvに対して毎回行を追加、mapに対して毎回文字列を追加、keyがありvalueが小さければ更新する
    for i in 0..vec[0].len() {
        vv.push(vec[0][i].clone());
        map.insert(vec[0][i].clone(),count);
    }
    for i in 0..n {
        for j in 0..vec[i].len() {
            vv.push(vec[i][j].clone());
            if map.get(&vec[i][j]).unwrap() < &count {
                map.insert(vec[i][j].clone(),count);
            }
            if map.contains_key(&vec[i][j]) {
                map.insert(vec[i][j].clone(),map.get(&vec[i][j]).unwrap()+1);
            } else {
                map.insert(vec[i][j].clone(),1);
            }
        }
    }
    let ans = map.get(t).unwrap();
    println!("{}",ans);
}
