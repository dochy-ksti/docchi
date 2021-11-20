//
// use crate::docchi_json5::JVal;
// use crate::error::CoreResult;
//
// #[allow(dead_code)]
// pub(crate ) fn untyped_example() -> CoreResult<JVal> {
//
//
//
//
//     let data = r#"
// {
//   hogeNumber : 10,
//   hogeString : "hoge",
//   hogeBool : true,
//   "@HugaNumber" : 30, //先頭が大文字のメンバ名はすべてシステムが予約しているので認められない。しかしメンバ名をどうしても大文字で始めたい場合、
//                       //jsonでは戦闘に@をつけ、プログラム側では@なしでもアクセス可能にするという技も使えなくはなかろうと思う。
//
//   //名前が?で終わるメンバはnull値を含むことが出来る。
//   //プログラム側のメンバ名にはこの?は反映されず、型が変わるだけ(もし俺以外の実装が現れたらわからないが・・・）
//   //初期値にnullを入れるには、特殊な記法を使う必要がある
//   "hegoNumber?" : ["Num", null], //型を指定し、null値を入れる。これでnullになる。
//
//   //!で終わる場合、バージョン違いでこのメンバを持っていなかった場合、デフォルト値でなくundefinedが入る。
//   //undefinedを代入する手段はない。
//   "pugyaNumber!" : 10,
//   "pugyaNumber2!?" : ["Num", null], //!?も出来る。?!ではダメ
//
//   //配列はいまのところnumber配列、string配列、number配列の配列の4通り。
//   numArray : [ "Num-Array", 0, 3, 10 ],
//
//   //numArrayはタグなしでも書ける特殊記法がある
//   numArrayNoTag : [ 5.8, 8, 10 ],
//   emptyNumArray : [ "Num-Array" ], //初期値が空配列のnum-array
//   "nullableNumArray?" : [ "Num-Array", null ], //nullable配列の初期値をnullにする場合
//   num2Array : [ "Num2-Array", [2,3], [3,1] ], //二次元のnumarray
//
//   strArray : [ "Str-Array", "hoge", "hogehoge" ], //文字列配列
//   //そもそも配列なんてこのシステムに必要なんだろうか・・・？　まともに差分生成出来る気もしないしなあ。
//
//   Renamed : [ "prevName->currentName",
//               "prevName2->currentName2" ], //メンバ名の変更をした場合、これを書いておくことで自動でメンバ名の対応表を作ってくれる。
//               //参照可能なListの名前が変わった場合参照先も追跡できる
//
//   Include : { someList : "someList.json5_back" }, //メンバの中身を別ファイルに書くことが出来る。
//
//   unkoList : [
// 	"List", //Listは配列とは違う。オブジェクトのコレクションを作るためにはlistを使う必要がある。
// 	["Reffered"], //参照可能になる
// 	["Default", {
// 		hogeNumber : 0,
// 		hogeString : "hoge"
// 	}], //デフォルト値を設定
// 	{
// 		ID : "first",
// 		hogeNumber : 12,
// 		hogeString : "huga"
// 	},
// 	{
// 		ID : "second",
// 		//デフォルト値から変更がない場合は書かなくても良い
// 	}],
//
//
//
//   usable : [
//     "List",
//     ["Reffered"],
//     ["Default",{ num : 0 }],
//     {
//       ID : "yakusou",
//       num : 3
//     },
//     {
//       ID : "dokukesisou",
//       num : 31
//     }
//   ],
//
//   weapon : [
//     "List",
//     ["Default", { atk : 0 }],
//     ["Reffered"],
//     {
//       ID : "katana",
//       atk : 5
//     },
//     {
//       ID : "doutanuki",
//       atk : 7
//     }
//   ],
//
//   itemList : [
//     "List",
//     ["AutoID"], //RefferedとAutoIDは同時には使えない
//     ["Default",{
//       "atk?" : ["Num", null],
//       "Ref": { weapon : "" }, //Refの基本の形、なにか入れないとエラーになる。
//     }],
//     {
//       Ref : { weapon : "doutanuki" }, //どうたぬきを参照。
//       atk : 8 //override的ななにか
//     }
//   ],
//
//   hogeList : [
//     "List",
//     ["Reffered"],
//     ["Default",{ mem : "" }],
//     {
//       ID : "hogehoge",
//       mem : "a",
//     }
//   ],
//
//   hugaList: [
//     "List",
//     ["Reffered"],
//     ["Default",{}],
//     {
//       ID : "hugahuga"
//     }
//   ],
//
//   hegoList: [
//     "List",
//     ["Reffered"],
//     ["Redef", "hugahuga->hegohego"],
//     ["Default", {
//         mem : "",
//         Renamed: ["memOld->mem"] //DefaultメンバではRenamedの設定が可能
//     }],
//     {
//       ID : "hegohego",
//       mem : "b",
//     },
//     {
//       ID : "nantoka",
//       Obsolete : true //被参照リストでは要らなくなっても削除は危険なのでObsoleteするのが良いと思われる。
//     }
//   ],
//
//   itemList3 : [
//     "List",
//     ["AutoID"],
//     ["Default",{
//       "memOverride?" : ["Str", null],
//       "Ref" : { hogeList : "", "hugaList?" : null, hegoList : "" },
//     }],
//     {
//       Ref : { //RefListIDが設定されている場合、RefIDが必要。必要なメンバを設定する。
//         hogeList : "hogehoge", //RefListIDと、RefIDをセットで記述していく。
//         //nullableだと入力しなければデフォルトでnull
//         hegoList : "hegohego",
//       },
//     }
//   ],
//
// }
//
// //使用側 概念コード
// //let item = itemList3[0]; //item は hogehoge と hegohego を参照している
// //item.RefIDs.hoge.mem <これは"a"である
// //item.hoge.mem <直接メンバとして呼び出すことも可能。同名のメンバがある場合はそちらが優先。
// //list idは究極的なところ、一度作ったらもう二度と変えられないと思う。メンバ名は変えられるようになってるので、必要ならそっちを変えればよろしかろう。
// //item.hoge.mem = "b" //参照越しに書き換えてもいいのか？　わからないけれど、システムレベルで禁止する必要はないだろう。必要なければ使わなければ良い。
// //存在自体が害悪になるような機能ならライブラリ側で勝手に隠せば良い。
// //個別にoverrideしたいケースがあるかもしれないが、override機能を実装するのはやはりおかしいと思うので（これはただのデータ記述システムであり、オブジェクト指向言語じゃない。
// //nullableなメンバを上書きして、実質overrideのようにした方が良かろうと思う。
// ////item.memOverride = "c" //nullableなメンバを書き換える
// //if(item.memOverride != null){ return item.memOverride } else{ return item.mem }
//
//
//
//
//
//
// "#;
//
//     // Parse the string of data into serde_json::Value.
//     match crate::docchi_json5::from_str(data){
//         Ok(v) =>{
//             //println!("{}", serde_json::to_string_pretty(&v).unwrap());
//             Ok(v)
//         },
//         Err(e) =>{ println!("{:?}", e); panic!(); },
//     }
//
// }