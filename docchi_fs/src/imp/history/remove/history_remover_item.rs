use std::collections::{HashMap};
use crate::imp::history::file_name::file_name_props::FileNameProps;
use std::sync::atomic::AtomicBool;
use crate::imp::history::file_hist::file_history_item::FileHistoryItem;
use std::sync::atomic::Ordering;
use crate::imp::history::remove::composite_remover::composite_remover;

pub(crate) struct HistoryRemoverItem<'a>{
    cumulative : bool,
    removable : AtomicBool,
    props : Option<&'a FileNameProps>,
    children : HashMap<u32, HistoryRemoverItem<'a>>,
}

pub(crate) struct RemoveCueItem<'a>{
    ctl : u32,
    order : &'a [u32],
    order_last : Option<u32>,
}
impl<'a> RemoveCueItem<'a>{
    pub(crate) fn new(ctl : u32, order : &'a [u32], order_last : Option<u32>) -> RemoveCueItem<'a>{
        RemoveCueItem{ ctl, order, order_last }
    }

    pub(crate) fn ctl(&self) -> u32{ self.ctl }
    pub(crate) fn order(&self) -> &'a [u32]{ self.order }
    pub(crate) fn order_last(&self) -> Option<u32>{ self.order_last }

    pub(crate) fn from(props : &'a FileNameProps) -> RemoveCueItem<'a>{
        Self::new(props.control(), props.order(), None)
    }
}

impl<'a> HistoryRemoverItem<'a>{
    pub(crate) fn new(cumulative : bool,
               props : Option<&'a FileNameProps>,
               children : HashMap<u32, HistoryRemoverItem<'a>>) -> HistoryRemoverItem<'a>{
        HistoryRemoverItem{ cumulative, removable : AtomicBool::new(true), props, children }
    }

    pub(crate) fn children(&self) -> &HashMap<u32, HistoryRemoverItem<'a>>{
        &self.children
    }

    /// 自分自身のRemovableをfalseにし、直近の親のorderを返す
    /// &mut selfにするとlifetimeの整合性が取れなかったので、interior mutabilityを使う(AtomicBoolなのでスレッドセーフ)
    pub(crate) fn keep(&self) -> Option<RemoveCueItem<'a>>{
        let props = if let Some(props) = self.props { props } else {
            return None;
        };

        //特に何とも関わっていないと思うのでRelaxedでいいと思う
        if self.removable.load(Ordering::Relaxed) == false{ return None; }

        self.removable.store(false,Ordering::Relaxed);
        if self.cumulative {
            let order_last = props.order_last();
            if order_last != 0 {
                return Some(RemoveCueItem::new(props.control(), props.order_base(), Some(order_last - 1)));
            }
        }
        if props.order().len() != 1{
            return Some(RemoveCueItem::new(props.prev_ctl(), props.order_base(), None));
        } else{
            return None;
        }
    }

    pub(crate) fn get_removable_props(&self, r : &mut Vec<&'a FileNameProps>){
        if self.removable.load(Ordering::Relaxed){
            if let Some(props) = self.props {
                r.push(props)
            }
        }
        for (_,child) in &self.children{
            child.get_removable_props(r);
        }
    }

    pub(crate) fn from(src : &'a FileHistoryItem, props : Option<&'a FileNameProps>,
                       cur_phase : usize, max_phase : usize, cumulative_option : bool) -> HistoryRemoverItem<'a>{

        let r = composite_remover(src.items(), src.children(), cur_phase, max_phase, cumulative_option);
        let cumulative = cur_phase == max_phase && cumulative_option;
        HistoryRemoverItem::new(cumulative, props,  r)
    }
}
