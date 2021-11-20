
pub(crate) struct SourceBuilder{
    pub(crate) s : String,
}
impl SourceBuilder{
    pub(crate) fn new() -> SourceBuilder{ SourceBuilder{ s : String::new() }}
    pub(crate) fn push(&mut self, tabs : usize, s : &str){
        for line in s.split('\n'){
            self.s.push_str(&tab_line(tabs, line));
        }
    }
    pub(crate) fn push_without_newline(&mut self, tabs : usize, s : &str) {
        let s =
        if s.ends_with('\n'){
            &s[0..(s.len()-1)]
        } else{
            s
        };
        for line in s.split('\n') {
            self.s.push_str(&tab_line(tabs, line));
        }

    }

    pub(crate) fn append(&mut self, s : &str) {
        self.s.push_str(s);
    }

    pub(crate) fn to_string(&self) -> String{
        self.s.to_string()
    }
}


fn tab_line(tabs : usize, s : &str) -> String{
    let mut r : String = String::new();
    for _ in 0..tabs{
        r.push('\t');
    }
    r.push_str(s);
    r.push('\n');
    r
}