pub fn verse(n: u32) -> String {
    if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
    }

    if n == 1 {
        return format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    }

    if n == 1 {
        return format!("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    }

    if n == 2 {
        return format!("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
    }

    format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n - 1)
}

pub fn sing(start: u32, end: u32) -> String {
    (end..(start + 1))
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<String>>()
        .join("\n")
}
