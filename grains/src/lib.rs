// pub fn square(s: u32) -> u64 {
//     if !(1..=64).contains(&s) {
//         panic!("Square must be between 1 and 64");
//     }
//     (2u64).pow(s - 1)
// }

pub fn square(s: u32) -> u64 {
    match s {
        // 1. range check on unsigned is just one sub and one test
        // 2. rust does not optimize 2.pow(), thus the shift
        1..=64 => 1 << s - 1,
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    u64::MAX
}
