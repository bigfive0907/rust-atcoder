use std::{cmp::Reverse, collections::BinaryHeap};

const INF: usize = std::usize::1<<31;


struct Dijkstra {
  distance: Vec<usize>,
  parent: Vec<usize>,
}

impl Dijkstra {
  // n 頂点の数、edge[i]は(頂点への経路、重み),init 起点（スタート地点）
  //　なぜvec二次元なのか
  pub fn new(n: usize, edge: Vec<Vec<(usize, usize)>>, init: usize) -> Self {
    let mut distance = vec![];
    let mut parent = vec![];
    let mut heap = BinaryHeap::new(); //heapに隣接頂点を格納

    //初期化　binray_heapはpopで最大を出力するのでReverse
    for i in 0..n {
      if i == init {
        heap.push((Reverse(0),i)); //優先度最高
      }
      else{
        heap.push((Reverse(std::usize::INF),i)); //優先度最低
      }
    }
    // dとtargetを処理で扱う
    while let Some((Reverse(d), target)) = heap.pop(){
      //既に最短経路が求まっていればskip
      //1 -> 2を考えると1の距離について
      if distance[target] < d{
        continue;
      }
      distance[target] = d;
      //heap(重みd、目的地target)edge(目的地next、重みcost)に注意すること
      for &(next, cost) in &edge[target] {
        //次の点への移動距離が古い暫定距離より小さかったら
        //1 -> 2を考えると1 -> 2の距離について1までのd + 2までの重みcost
        if distance[next] > d + cost {
          distance[next] = d + cost;
          // 新規に最短経路が求まったのでheapにnextの暫定距離をpushする
          // INFと比べて優先度大で、2までの距離をheapで更新,
          heap.push((Reverse(distance[next]), next));
          // 暫定距離を求めた点の親を、暫定で最短距離の現在の点に更新する
          parent[next] = target;
        }
      }
    }
    Self { distance, parent }
  }
  //targetへの最短距離を返す
  pub fn distance(&self, target:usize) -> Vec<usize> {
    self.distance[target];
  }

  pub fn path(&self, target: usize) -> Vec<usize> {
    let mut current = target;
    let mut res = vec![current]; //current , parent[current] , parent[parent[current]] , ...
    while self.parent[current] != INF as usize {
      let next = self.parent[current];
      res.push(next);
      current = next;
    }
    res.reverse();
    res
  }
}

/*
ChatGPTの解説
pub fn new(n: usize, edge: Vec<Vec<(usize, usize)>>, init: usize) -> Self {: Dijkstra 
構造体の新しいインスタンスを生成するための関数です。n はグラフの頂点の数、edge は隣接リストを表す二次元ベクタ、init は始点の頂点を示します。

let mut distance = vec![]; と let mut parent = vec![];:
各頂点への最短距離と親頂点を格納するための空のベクタを作成します。後で計算中にこれらのベクタが埋められます。

let mut heap = BinaryHeap::new();: 
ダイクストラのアルゴリズムで使用するためのバイナリヒープを生成します。このヒープは、まだ探索されていない頂点とその最短距離を保持します。

for i in 0..n { ... }: 
各頂点の初期化を行います。始点の場合は距離を0にしてヒープに追加し、それ以外の場合は距離を無限大にしてヒープに追加します。

while let Some((Reverse(d), target)) = heap.pop() { ... }: 
ヒープから最短距離が最小の頂点を取り出し、その頂点からの最短経路を更新します。このループはヒープが空になるまで続きます。

if distance[target] < d { continue; }: 
最短距離が既に計算されている場合はスキップします。

distance[target] = d;: 
頂点 target までの最短距離を更新します。

for &(next, cost) in &edge[target] { ... }: 
頂点 target に隣接する各頂点 next について、新たな最短経路を探索します。

if distance[next] > d + cost { ... }: 
現在の最短距離よりも新たな経路の方が短ければ、最短距離と親頂点を更新し、ヒープに新しい経路を追加します。

Self { distance, parent }: 
最終的な Dijkstra 構造体を作成し、最短距離と親頂点を保持します。

このようにして、ダイクストラのアルゴリズムはグラフ内の始点から各頂点への最短経路を計算し、それを Dijkstra 構造体として返します。

example

let edge = vec![
  //0の隣接点vec (頂点、重み)
  vec![(1, 5), (2, 3)],
  //1
  vec![(0, 5), (2, 4), (3, 7)],
  //2
  vec![(0, 3), (1, 4), (3, 2)],
  //3
  vec![(1, 7), (2, 2)]
];

let dijkstra = Dijkstra::new(4, edge, 0);
*/