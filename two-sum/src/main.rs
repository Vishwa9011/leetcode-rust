// nums = [2,7,11,15], target = 9

use std::collections::HashMap;

fn main() {
    let nums: Vec<i32> = vec![2, 7, 11, 15];
    let result: Vec<i32> = two_sum(nums, 9);
    println!("target indexes are {:?}", result);
}

// first approach
// fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     for (i, &num1) in nums.iter().enumerate() {
//         for (j, &num2) in nums.iter().enumerate() {
//             let sum: i32 = num1 + num2;
//             if sum == target && i != j {
//                 return vec![i as i32, j as i32];
//             }
//         }
//     }
//     return vec![];
// }

// second approach
// fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     let mut map: HashMap<i32, i32> = HashMap::new();

//     for (i, &val) in nums.iter().enumerate() {
//         map.insert(val, i as i32);
//     }

//     for (i, &val) in nums.iter().enumerate() {
//         let temp: i32 = target - val;
//         if map.contains_key(&temp) {
//             if let Some(&index) = map.get(&temp) {
//                 return vec![i as i32, index];
//             }
//         }
//     }

//     return vec![];
// }

// third approach
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for (i, &val) in nums.iter().enumerate() {
        let temp: i32 = target - val;
        if let Some(&value) = map.get(&temp) {
            return vec![i as i32, value];
        }
        map.insert(val, i as i32);
    }

    return vec![];
}
