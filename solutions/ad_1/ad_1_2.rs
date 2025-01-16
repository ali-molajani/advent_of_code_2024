mod utils;

use utils::file_edit::read;
use utils::mathematica::freq;
fn main() {
    let nums1;
    let nums2;
    (nums1, nums2) = read("./input.txt".to_string());
    let frequency = freq(nums1, nums2);
    let ds = frequency.iter().sum::<u32>();
    println!("the sum of frequencies : {ds}");
    // println!("second vector: {:?}", nums2);
}
