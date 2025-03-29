pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();

    if trimmed.is_empty() {
        "Fine. Be that way!"
    } else if trimmed.ends_with('?') {
        if trimmed.chars().any(|c| c.is_alphabetic()) && trimmed.chars().all(|c| !c.is_lowercase())
        {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if trimmed.chars().any(|c| c.is_alphabetic())
        && trimmed.chars().all(|c| !c.is_lowercase())
    {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
