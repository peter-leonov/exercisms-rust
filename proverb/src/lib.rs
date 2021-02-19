use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    if let Some(first) = list.first() {
        list.windows(2)
            .map(|w| format!("For want of a {} the {} was lost.\n", w[0], w[1]))
            .chain(once(format!("And all for the want of a {}.", first)))
            .collect()
    } else {
        "".to_owned()
    }
}

// pub fn build_proverb(list: &[&str]) -> String {
//     let first = match list.first() {
//         Some(first) => format!("And all for the want of a {}.", first),
//         None => return "".to_owned(),
//     };
//     list.windows(2).rfold(first, |acc, w| {
//         format!("For want of a {} the {} was lost.\n{}", w[0], w[1], acc)
//     })
// }
