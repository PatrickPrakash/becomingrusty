use std::collections::HashMap;

fn two_sum(_sum_of_pairs: &[u32], _target: &u32) -> (u32, u32) {
    let mut _pre_computed_sum = HashMap::new();
    
    for (index, &_item) in _sum_of_pairs.iter().enumerate() {
        let _pre_calculated_difference: i32 = (*_target as i32) - (_item as i32);

        if let Some(&remaining_value) = _pre_computed_sum.get(&(_pre_calculated_difference as u32)) {
            return (remaining_value, _item);
        }

        _pre_computed_sum.insert(_item, _item);
    }
    (0, 0)
}

fn main() {
    let a = [1, 3, 34, 7, 11, 23];
    let (value1, value2) = two_sum(&a, &37);
    
    if value1 != 0 || value2 != 0 {
        println!("Values: {}, {}", value1, value2);
    } else {
        println!("No pairs found.");
    }
}
