pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return "".to_string();
    }
    let mut verbs = list[1..]
        .iter()
        .enumerate()
        .map(|(i, w)| format!("For want of a {} the {} was lost.", list.get(i).unwrap(), w))
        .collect::<Vec<_>>();
    verbs.extend([format!(
        "And all for the want of a {}.",
        list.first().unwrap()
    )]);
    verbs.join("\n")
}
