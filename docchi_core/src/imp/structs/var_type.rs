use crate::imp::structs::qv::QvType;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VarType {
    Normal = 0,
    Nullable = 1,
    Undefiable = 2,
    UndefNullable = 3,
}

impl VarType {
    pub(crate) fn undefiable(&self) -> bool{
        match self{
            VarType::Undefiable | VarType::UndefNullable => true,
            _ => false,
        }
    }

    pub(crate) fn to_suffix(&self) -> String{
        let s = match self{
            VarType::Normal => "",
            VarType::Nullable => "?",
            VarType::Undefiable => "!",
            VarType::UndefNullable => "!?",
        };
        s.to_string()
    }




    pub(crate) fn acceptable(&self, t : &QvType) -> bool {
        match self{
            VarType::Normal => {
                match t {
                    QvType::Val => true,
                    _ => false,
                }
            },
            VarType::Nullable => {
                match t {
                    QvType::Val | QvType::Null => true,
                    _ => false,
                }
            },
            VarType::Undefiable => {
                match t {
                    QvType::Val | QvType::Undefined => true,
                    _ => false,
                }
            },
            VarType::UndefNullable => true,
        }
    }

    // pub(crate) fn compatible(&self, other : &Self) -> bool{
    //     match self{
    //         VarType::Normal => match other{
    //             VarType::Normal => true,
    //             _ => false,
    //         },
    //         VarType::Nullable => match other{
    //             VarType::Normal | VarType::Nullable => true,
    //             _ => false,
    //         }
    //         VarType::Undefiable => match other{
    //             VarType::Normal | VarType::Undefiable => true,
    //             _ => false,
    //         }
    //         VarType::UndefNullable => true,
    //     }
    // }
}
