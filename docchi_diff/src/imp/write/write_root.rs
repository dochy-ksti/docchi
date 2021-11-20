use crate::imp::structs_write::{RootDiffW};
use docchi_compaction::kval_enum::KVal;
use crate::imp::write::store_ids::{StoredIDs, store_ids};
use docchi_core::structs::{RustParam, MetaTable, MetaValue};
use crate::imp::write::write_param::write_param;
use std::collections::BTreeMap;
use crate::imp::write::write_list::{write_ld_lists};
use crate::diff_error::DiffError;
use crate::imp::write::write_store_ids::write_stored_ids;

///jsonの定義でnullableでない変数にnullを入れるのも不可能ではない。
/// それは間違いだし保存できないのでエラーであるが、panic即落ちは避けてErrorを返している
///
/// それとは別に、metatableの型の整合性がない、といった明らかなバグについてはpanicしている。
///
/// 全部パニックのほうがデバッグも楽だし簡単なんだが・・・どうも踏ん切りがつかない。
/// Errorが帰ってきても仕方ないのでパニックでいいとは思うのだが、汎用ライブラリである以上即落ちはよくないという判断もある
/// そもそもjson定義をいじる過程でnullableを非nullableに変え、さらに旧バージョンを無理やり読み出したりすることもないとは言えないような気がする
///
/// その場合、即落ちしないので「保存に失敗しました」というダイアログを出せる。ログを取得して報告する事もできるかもしれない。
/// だからErrorの方がいいという考え方はあるだろう。それはResultとか?だらけにして可読性を下げるのに見合うのか？
/// 俺の考えは「見合わない」だが、いまのところ余力があるので、かすかなメリットでも得てみようという気持ちになっている。
/// もしかしたら俺の見立てが間違いでこれが大正解である可能性もないわけではないだろうし・・・
pub(crate) fn write_root(root : RootDiffW) -> Result<Vec<KVal>, DiffError>{
    let params = root.params();
    let lists = root.lists();
    let meta = root.meta_table();

    let mut r : Vec<KVal> = Vec::with_capacity((params.len() + lists.len()) * 2);

    let ids = store_ids(params, lists);
    write_stored_ids(&ids, &mut r);
    match ids{
        StoredIDs::Zero =>{
            Ok(r)
        },
        _ =>{
            write_params(params, meta, &mut r)?;
            write_ld_lists(lists,meta, &mut r)?;
            Ok(r)
        },
    }
}

pub(crate ) fn write_params(params : &BTreeMap<usize, &RustParam>, meta : &MetaTable, r : &mut Vec<KVal>) -> Result<(), DiffError>{
    for (&id, &param) in params{
        if let Some((_,MetaValue::Param(meta_param))) = meta.get(id) {
            write_param(param, meta_param, r)?
        } else{
            panic!("invalid meta");
        }
    }
    Ok(())
}

// fn write_lists(lists : &BTreeMap<usize, ListDiffW>, r : &mut Vec<KVal>) -> Result<(), DiffError>{
//     for (_id, ld) in lists {
//         write_list(ld, r)?
//     }
//     Ok(())
// }
//
