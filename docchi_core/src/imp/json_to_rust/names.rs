use std::fmt::Display;

pub(crate) struct Names<'a>{
    pub(crate) name : &'a str,
    pub(crate) next : Option<&'a Names<'a>>,
}

impl<'a> Names<'a>{
    pub fn to_string(&self) -> String{
        let mut vec : Vec<String> = vec![];
        let mut cur = self;
        loop{
            vec.push(cur.name.to_string());
            if cur.next.is_none(){
                break;
            }
            cur = cur.next.unwrap();
        }
        vec.reverse();
        vec.join(".")
    }

    pub fn append(&'a self, name : &'a str) -> Self{
        Names::<'a>{ name, next : Some(self)}
    }

    pub fn new(name : &'a str) -> Self{
        Names::<'a>{ name, next : None }
    }
}

impl<'a> Display for Names<'a>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, "{}", self.to_string())
    }

}