pub mod file_edit {
    use std::fs;

    pub fn read(addr: String) -> (Vec<u32>, Vec<u32>) {
        let things = match fs::read_to_string(addr) {
            Ok(txt) => txt,
            Err(err) => panic!("Error reading file: {err}"),
        };

        things
            .split('\n')
            .filter(|line| !line.trim().is_empty())
            .filter_map(|line| {
                let parts: Vec<&str> = line
                    .split("  ")
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect();

                match parts.as_slice() {
                    [first, second] => match (first.parse::<u32>(), second.parse::<u32>()) {
                        (Ok(num1), Ok(num2)) => Some((num1, num2)),
                        _ => None,
                    },
                    _ => None,
                }
            })
            .unzip()
    }
}
pub mod mathematica {
    pub fn minimum(nums: Vec<u32>) -> u32 {
        let mut min: u32 = nums[0];
        let _ = nums.into_iter().map(|x| {
            if x < min {
                min = x
            }
        });
        min
    }
    pub fn abs_value(n1: u32, n2: u32) -> u32 {
        if n1 > n2 {
            n1 - n2
        } else {
            n2 - n1
        }
    }
}
