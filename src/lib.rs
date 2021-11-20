//! [Dochy User's Manual](https://github.com/dochy-ksti/dochy/blob/master/dochy_manual/manual/index.md)
//!
//! Dochy is a language, so this API document is not sufficient to explain what it is.

#![feature(vec_into_raw_parts)]
#![feature(test)]
extern crate test;



pub mod intf;
pub mod core;
pub mod fs;
pub mod error;



//TODO: ファイルは一旦tmpファイルに書き出してから書き出してからリネームするようにする
//TODO: フォルダ削除時に途中で強制終了があると中途半端なファイルだけ残り不正な状態になる可能性があるので、まずフォルダ名を変更しフォルダ内のファイルを無効にしてから削除するようにする？　最新ファイルから順々に過去ファイルを消し、最後にフォルダも消す？
//TODO: CopySaveFileToPhase0を作る SaveAsyncを作る
//TODO: バックエンドを変更可能にする。デバッグのためのインメモリ、leveldb-rsを使ってKVPなど。ファイルももっと安全に、フォルダ削除時に新しいファイルから消していくような、構造が壊れない処理を
//TODO: Pathの同一性が怪しいから、Pathを自作のものに置き換える??
//TODO: 現状max_phaseのload時に派生元をmax_phase-1からにしているが、最新アイテムから派生しても別にいいような気がするんだが・・・
//TODO: 2回セーブしてロードの繰り返しできちんとHistoryを構成するtestが書きたいので、max_phaseの最新ファイル以外からもロード後に派生できるようにする <-いずれ・・


//TODO: 全部documentationする
//TODO: Redditに出して反応を見る このペースだと何ヶ月かかるんじゃあ・・・
