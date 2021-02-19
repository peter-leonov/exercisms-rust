pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = list
        .windows(2)
        .map(|w| format!("For want of a {} the {} was lost.", w[0], w[1]))
        .collect::<Vec<String>>();

    if let Some(first) = list.first() {
        proverb.push(format!("And all for the want of a {}.", first));
    }

    proverb.join("\n")
}
