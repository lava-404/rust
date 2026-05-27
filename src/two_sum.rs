use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<usize> {
    let mut map = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let complement = target-num;

        if let Some(&index) = map.get(&complement){
            return vec![index, i];
        }

        map.insert(*num, i);

    }
    vec![]
}

fn main() {
    let vec = two_sum(vec![1, 2, 3, 4], 3);
    println!("{vec:?}");
}