/// History's filenames have a tag(arbitrary string given by user<optional>)
/// control number(increases when start_new is called)
/// order( numbers ancestors of this history file have.
/// if order is (1,2,1,1) ancestors have orders
/// (1)<- grand grandparent
/// (1,2)<- grand parent
/// (1,2,1)<- parent )
pub fn calc_filename(tag : Option<&str>, control : u32, prev_ctl : Option<u32>, order: &[u32]) -> String{
    let mut s = String::new();

    if let Some(tag) = tag{
        s.push('#');
        s.push_str(&tag);
        s.push('#');
    }
    if let Some(prev_ctl) = prev_ctl{
        s.push('(');
        s.push_str(&prev_ctl.to_string());
        s.push(')');
    }
    s.push('_');
    s.push_str(&control.to_string());
    for phase in order {
        s.push('_');
        s.push_str(&*phase.to_string())
    }
    s.push_str(".his");
    s
}