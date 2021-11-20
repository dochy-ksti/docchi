use crate::imp::history::algo::phase_data::PhaseData;
use crate::imp::history::algo::should_shift_to_parent_phase::should_shift_to_parent_phase;
use crate::imp::history::algo::history_options::HistoryOptions;

pub(crate) fn calc_next_phase(data : &PhaseData, op : &HistoryOptions) -> usize{
    let len = data.len();
    let max_phase = op.max_phase();

    if len <= max_phase{
        return len;
    }

    for i in 0..(len - 1){
        let current = data.get(i).unwrap();
        let parent = data.get(i+1).unwrap();

        if should_shift_to_parent_phase(
            parent.my_size(),
            parent.descendant_total_size(),
            parent.descendant_total_len(),
            current.my_size() + current.descendant_total_size(),
            1 + current.descendant_total_len()
        ) == false{
            if i == 0 {
                if let Some(cu) = op.cumulative() {
                    if let Some(limit_count) = cu.limit_count() {
                        if limit_count <= parent.descendant_total_len() {
                            continue;
                        }
                    }
                    if let Some(limit_nth) = cu.limit_nth() {
                        if let Some(size) = data.get_nth_largest_files_size(limit_nth) {
                            if size <= parent.descendant_total_size() {
                                continue;
                            }
                        }
                    }
                }
                return len - 1;
            }else {
                return len - 1 - i;
            }
        }
    }
    if op.update_phase_0() == false{
        return 1;
    }

    return 0;
}