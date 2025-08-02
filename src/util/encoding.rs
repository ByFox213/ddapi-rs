use std::borrow::Cow;
use std::fmt::Write;

const SLUGIFY_SYMBOLS: &str = "\t !\"#$%&'()*-/<=>?@[\\]^_`{|},.:+";

pub fn slugify2(nickname: &str) -> Cow<'_, str> {
    if !nickname
        .chars()
        .any(|c| SLUGIFY_SYMBOLS.contains(c) || c.is_ascii())
    {
        return Cow::Borrowed(nickname);
    }

    let mut result = String::with_capacity(nickname.len() * 4);

    for c in nickname.chars() {
        if SLUGIFY_SYMBOLS.contains(c) || !c.is_ascii() {
            write!(&mut result, "-{}-", c as u32).unwrap();
        } else {
            result.push(c);
        }
    }

    Cow::Owned(result)
}

pub fn encode(nickname: &str) -> Cow<'_, str> {
    if nickname
        .chars()
        .all(|c| c.is_ascii() && !c.is_ascii_control())
    {
        Cow::Borrowed(nickname)
    } else {
        urlencoding::encode(nickname)
    }
}
