pub fn is_leap_year(year: u64) -> bool {
    if year & 0b11 != 0 {
        return false;
    }

    if year % 100 == 0 {
        if year % 400 == 0 {
            return true;
        }
        return false;
    }
    return true;
}
