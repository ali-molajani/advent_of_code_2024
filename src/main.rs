mod utils;

use utils::file_edit::minimum;
use utils::file_edit::read;
fn main() {
    let (nums1, nums2) = read("./input.txt".to_string());
    let min1 = minimum(nums1);
    let min2 = minimum(nums2);
    println!("First min is: {:?}", min1);
    println!("second min is: {:?}", min2);
}
