

///上のphaseへの移行をすべきか
pub fn should_shift_to_parent_phase(
    parent_size : u64, descendant_total_size : u64, descendant_total_len : usize,
    current_total_size: u64, current_total_len: usize) -> bool{

    //平行phase shiftをした場合。currentが繰り返されるものと仮定。
    let size = parent_size + descendant_total_size + current_total_size;
    let len = 1 + descendant_total_len + current_total_len;
    let average_normal_shift = size as f64 / len as f64;

    //継続せず次のcumにshiftした場合
    let size = parent_size + descendant_total_size;
    let len = 1 + descendant_total_len;
    let average_parent_shift = size as f64 / len as f64;


    return average_normal_shift > average_parent_shift;
}