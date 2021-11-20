
use crate::{HashM, HashMt, IdentityEqual};
use crate::imp::structs::ref_value::RefSabValue;
use crate::imp::structs::list_value::{ListDefValue};
use crate::imp::structs::rust_param::RustParam;
use crate::imp::structs::util::set_sabun::{SetSabunError, verify_set_sabun};
use crate::imp::structs::list_def_obj::ListDefObj;
use crate::imp::structs::linked_m::LinkedMap;
use crate::imp::structs::util::hash_m::{HashS};
use crate::imp::structs::list_sab_value::ListSabValue;
use std::sync::Arc;
use once_cell::sync::Lazy;


///アイテムごとにIDをもち、Refで参照することが可能である
#[derive(Debug, Clone, PartialEq)]
pub struct ConstTable {
    default : Box<ListDefObj>,
    list : Box<HashM<String, ConstItem>>,
    ///oldに設定されたIDはjsonから参照出来ない。変数名の末尾に"_Old"をつけないとプログラムからも使えない。
    old : Box<HashS<String>>,
}

impl ConstTable {
    pub(crate) fn new(default : ListDefObj, list : HashM<String, ConstItem>, old : HashS<String>) -> ConstTable {
        ConstTable { default : Box::new(default), list : Box::new(list), old : Box::new(old) }
    }
    pub(crate) fn default(&self) -> &ListDefObj{ self.default.as_ref() }
    pub(crate) fn list(&self) -> &HashM<String, ConstItem>{ self.list.as_ref() }
    pub(crate) fn old(&self) -> &HashS<String>{ self.old.as_ref() }
}

///IDを持たず、参照できない。バージョン違い読み出し時の動作の違いが一番大きな違いで、それが存在理由。
/// MutListは旧バージョンのアイテムが残り新バージョンの初期値が消えるが、ConstListでは新バージョンのアイテムが残り旧バージョンは消える。
/// Constなので当然といえるだろう・・・
#[derive(Debug, Clone, PartialEq)]
pub struct ConstList {
    default : Box<ListDefObj>,
    list : Box<Vec<ConstItem>>,
}

impl ConstList {
    pub(crate) fn new(default : ListDefObj, list : Vec<ConstItem>) -> ConstList { ConstList { default : Box::new(default), list : Box::new(list) } }
    pub(crate) fn default(&self) -> &ListDefObj{ self.default.as_ref() }
    pub(crate) fn list(&self) -> &Vec<ConstItem>{ self.list.as_ref() }
}

// ///追加、削除、順番の変更等ができるリスト
// /// ConstListとMutListはstruct定義を見ると近い存在なので、まとめてもいいように思うかもしれないけれど、意味が全く別ものなので型を分けたほうが混乱が少ない。
// #[derive(Debug, Clone)]
// pub struct MutList{
//     default : Box<ListDefObj>,
//     list : Box<LinkedMap<MutItem>>,
//     //compatible : Box<HashS<String>>,
// }

// impl MutList{
//     pub(crate) fn new(default : ListDefObj, list : LinkedMap<MutItem>) -> MutList{
//         MutList{ default : Box::new(default), list : Box::new(list) }
//     }
//     pub fn default(&self) -> &ListDefObj{ self.default.as_ref() }
//     pub fn list(&self) -> &LinkedMap<MutItem>{ self.list.as_ref() }
//     pub fn list_mut(&mut self) -> &mut LinkedMap<MutItem>{ self.list.as_mut() }
//     //pub(crate) fn distribute_mut(&mut self) -> (&mut ListDefObj, &mut LinkedMap<MutListItem>, &mut HashS<String>){ (self.default.as_mut(), self.list.as_mut(), self.compatible.as_mut()) }
//     pub(crate) fn next_id(&self) -> u64{ self.list.as_ref().next_id() }
//     //pub(crate) fn list_mut(&mut self) -> &mut LinkedHashM<u64, MutListItem>{ self.list.as_mut() }
//     //pub(crate) fn increment_next_id(&mut self){ self.prop.next_id += 1 }
//     // pub(crate) fn append_new_item(&mut self) -> u64{
//     //     self.list.insert(MutListItem::construct(HashMt::new(), HashMt::new()))
//     // }
//
//     //pub(crate) fn compatible(&self) -> &HashS<String>{ self.compatible.as_ref() }
//     pub(crate) fn deconstruct(self) -> (ListDefObj, LinkedMap<MutItem>){
//         (*self.default, *self.list)
//     }
// }
//
// impl IdentityEqual for MutList{
//     fn identity_eq(&self, other: &Self) -> bool {
//         self.list.identity_eq(other.list())
//     }
// }

///Table or CListの内部に作るList。ListDefObjの内部にはDefaultだけ書き、CItemの内部にはListのItemの羅列のみを書く。
#[derive(Debug, Clone, PartialEq)]
pub struct ConstListVal {
    list : Arc<Vec<ConstItem>>,
}

impl ConstListVal {
    pub(crate) fn new(list : Vec<ConstItem>) -> ConstListVal { ConstListVal { list : Arc::new(list) }}
    pub(crate) fn list(&self) -> &Vec<ConstItem>{ self.list.as_ref() }
    pub(crate) fn empty() -> &'static Vec<ConstItem>{
        static EMP_VEC: Vec<ConstItem> = Vec::new();
        &EMP_VEC
    }
}

#[derive(Debug,  Clone, PartialEq)]
pub struct MutListVal {
    list : Arc<LinkedMap<MutItem>>,
}

impl MutListVal {
    pub fn new(list : LinkedMap<MutItem>) -> MutListVal { MutListVal { list : Arc::new(list) } }
    pub(crate) fn deconstruct(self) -> Arc<LinkedMap<MutItem>>{ self.list }
    pub fn list(&self) -> &LinkedMap<MutItem>{ self.list.as_ref() }
    pub fn list_mut(&mut self) -> &mut LinkedMap<MutItem>{ Arc::make_mut(&mut self.list) }
    pub fn crate_empty_list() -> MutListVal{ MutListVal { list : Arc::new(LinkedMap::new())}}
    pub fn empty_lists_ref() -> &'static LinkedMap<MutItem> {
        static EMP: Lazy<LinkedMap<MutItem>> = Lazy::new(|| LinkedMap::new());
        &EMP
    }
}

impl IdentityEqual for MutListVal {
    fn identity_eq(&self, other: &Self) -> bool {
        self.list.identity_eq(&other.list)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstItem {
    ///ListItemの値は常にDefaultからの差分である
    values : Box<HashM<String, ListSabValue>>,
    ///ListItemの値はRefも常にDefaultからの差分である
    refs : Box<HashM<String, RefSabValue>>,
}

impl ConstItem {
    pub(crate) fn new(values : HashM<String, ListSabValue>, refs : HashM<String, RefSabValue>) -> ConstItem {
        ConstItem { values : Box::new(values), refs : Box::new(refs) }
    }
    pub(crate) fn values(&self) -> &HashM<String, ListSabValue>{ self.values.as_ref() }
    pub(crate) fn refs(&self) -> &HashM<String, RefSabValue>{ self.refs.as_ref() }
}

///たとえばキャラクターAとキャラクターBの間で出来事Cが起こったとする。
/// キャラクターAのIDをa, BのIDをbとする。
/// グローバルの出来事リストに出来事Cを記録し、next_idからidを振り、そのidをcとする。その出来事のオブジェクトにはaとbもvaluesに記録されている。
/// AのRelationリストのID bの項目にアクセスし、なければ作成し、insertする。
/// AのRelationリストbにある出来事ID保持用のlistに出来事ID cを記憶しておく。ID保持用のlistは、idだけで中身のないオブジェクトを集めたlistだ。
/// 同様にキャラクターBのRelationリストaの出来事リストにも、出来事ID cを記録。
/// これにより、たとえば出来事Cを削除したい場合、Cにあるaとbを読み、AのbにあるID cのものを削除、 Bのaにあるcも削除、さらに出来事リストからCも削除すると、全部消える。
/// AとBとの間で何があったかの一覧がほしいなら、Aのbにアクセスし、出来事IDリストを取得、出来事リストからid検索し、出来事を取得、という感じになる。
/// 出来事リストのIDはnext_id方式により、時系列に積み上がっていくため、何年何月に起きた出来事はID x から y という情報があれば、
/// その間の出来事を全部調べたり、一定期間が過ぎた出来事データのうち重要じゃないものは消す、といった処理もできる。
/// キャラクターBを削除したい場合、他のキャラクターのRelationリストのbの部分を全部消し、BのRelationリストから取れる出来事IDを全部調べて
/// 出来事リストから全部消す、といった感じで消していくことが可能だ。
///
/// こういったユースケース（あるのか？）のためにLinkedHashM(u64,MutListItem)を使うとRelationを効率的に処理できるだろう。
/// あるいはBTreeMap(index_value, u64)でindex_valueでソートされたMapを作り、「index_valueがAからBの間にあるアイテム」といった条件で検索が可能になる。
/// そういったシステムを、出来事リストを読み出して外部にRelationを構築したり、パラメータをindex-keyとしてBTreeを構築したりすることで
/// （パラメータは上書きされうるので、その場合(item_id, BTreeのid)のRelationも使って、上書き時にBTreeをアップデートできるようにしておく必要もあり大変だが)
/// Relationとパラメータ範囲での検索が効率的にできるシステムが作れる。ただそれは外部に作ればいいので、このシステム自体の守備範囲ではない
/// それが出来る土台として、idとLinkedHashMで出来たMutListがある
#[derive(Debug, Clone, PartialEq)]
pub struct MutItem {
    ///ListItemの値は常にDefaultからの差分である
    values : Arc<HashM<String, ListSabValue>>,
    ///ListItemの値はRefでも常にDefaultからの差分である
    refs : Arc<HashM<String, RefSabValue>>,
}

impl MutItem {
    pub fn new(values : HashM<String, ListSabValue>, refs : HashM<String, RefSabValue>) -> MutItem {
        MutItem { values : Arc::new(values), refs : Arc::new(refs) }
    }
    pub(crate) fn default() -> MutItem {
        MutItem { values : Arc::new(HashMt::new()), refs : Arc::new(HashMt::new()) }
    }
    pub fn muts(&mut self) -> (&mut HashM<String, ListSabValue>,
                                  &mut HashM<String, RefSabValue>){
         (Arc::make_mut(&mut self.values), Arc::make_mut(&mut self.refs))
    }
    pub fn construct(values : Arc<HashM<String, ListSabValue>>,
                     refs : Arc<HashM<String, RefSabValue>>) -> MutItem{
        MutItem{ values, refs }
    }

    pub fn values(&self) -> &HashM<String, ListSabValue>{ self.values.as_ref() }
    pub fn values_mut(&mut self) -> &mut HashM<String, ListSabValue>{ Arc::make_mut(&mut self.values) }
    pub fn refs(&self) -> &HashM<String, RefSabValue>{ self.refs.as_ref() }
    pub fn refs_mut(&mut self) -> &mut HashM<String, RefSabValue>{ Arc::make_mut(&mut self.refs) }
    pub(crate) fn set_sabun(&mut self, def :&ListDefObj, name : String, param : RustParam) -> Result<Option<RustParam>, SetSabunError> {
        let (p, vt) =
            if let Some(ListDefValue::Param(p, vt)) = def.default().get(&name) {
                (p, vt)
            } else {
                return Err(SetSabunError::ParamNotFound);
            };
        verify_set_sabun(p, vt, &param)?;
        let op = self.values_mut().insert(name, ListSabValue::Param(param));
        Ok(op.map(|v| match v{
            ListSabValue::Param(p) => p,
            _ => unreachable!(),
        }))
    }
}

impl IdentityEqual for MutItem{
    fn identity_eq(&self, other: &Self) -> bool {
        self.values.identity_eq(&other.values) &&
            self.refs.identity_eq(&other.refs)
    }
}