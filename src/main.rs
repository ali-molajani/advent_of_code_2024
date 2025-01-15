mod utils;

use utils::file_edit::read;
use utils::mathematica::abs_value;
fn main() {
    let mut nums1;
    let mut nums2;
    let mut diff: Vec<u32> = vec![];
    (nums1, nums2) = read("./input.txt".to_string());
    nums1.sort();
    nums2.sort();
    for i in 0..nums1.len() {
        diff.push(abs_value(nums1[i], nums2[i]));
    }

    let ds = diff.iter().sum::<u32>();
    println!("Diff vector: {:?}", diff);
    println!("the sum of diff paths : {ds}");
    // println!("second vector: {:?}", nums2);
}
