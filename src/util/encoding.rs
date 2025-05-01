use std::borrow::Cow;
use std::fmt::Write;

pub fn slugify2(nickname: &str) -> Cow<str> {
    let slugify2_symbols = "\t !\"#$%&'()*-/<=>?@[\\]^_`{|},.:+";
    let mut result = String::with_capacity(nickname.len() * 4);

    for symbol in nickname.chars() {
        if slugify2_symbols.contains(symbol) || symbol as u32 >= 128 {
            write!(&mut result, "-{}-", symbol as u32).unwrap();
        } else {
            result.push(symbol);
        }
    }

    Cow::Owned(result)
}

pub fn encode(nickname: &str) -> Cow<str> {
    urlencoding::encode(nickname)
}
