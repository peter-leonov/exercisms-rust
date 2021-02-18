pub fn raindrops(n: u32) -> String {
    if n % 3 == 0 {
        // +Pling
        if n % 5 == 0 {
            // +Plang
            if n % 7 == 0 {
                // +Plong
                return String::from("PlingPlangPlong");
            } else {
                // -Plong
                return String::from("PlingPlang");
            }
        } else {
            // - Plang
            if n % 7 == 0 {
                // +Plong
                return String::from("PlingPlong");
            } else {
                // -Plong
                return String::from("Pling");
            }
        }
    }
    // -Pling
    if n % 5 == 0 {
        // +Plang
        if n % 7 == 0 {
            // +Plong
            return String::from("PlangPlong");
        } else {
            // -Plong
            return String::from("Plang");
        }
    }
    // - Plang
    if n % 7 == 0 {
        // +Plong
        return String::from("Plong");
    }
    // -Plong

    n.to_string()
}
