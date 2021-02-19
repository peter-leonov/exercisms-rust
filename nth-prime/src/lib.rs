pub fn nth(n: u32) -> u32 {
    // using 6k ± 1 opt. from Wiki
    fn is_prime(&n: &u32) -> bool {
        if n <= 3 {
            return n > 1;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }

        let mut i = 5;
        // 5 _ 7 _ _ _
        // 11 _ 13 _ _ _
        // 17 _ 19 _ _ _
        // 23 _ 25 _ _ _
        // 29 _ 21 _ _ _
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }

        return true;
    }

    (2..).filter(is_prime).nth(n as usize).unwrap()
}
