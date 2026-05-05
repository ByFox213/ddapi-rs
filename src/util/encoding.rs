use std::borrow::Cow;
use std::fmt::Write;

const NON_ASCII_CHARACTER_THRESHOLD: u32 = 128;

#[inline]
fn is_slugify2_symbol(c: char) -> bool {
    matches!(
        c,
        '\t' | ' '
            | '!'
            | '"'
            | '#'
            | '$'
            | '%'
            | '&'
            | '\''
            | '('
            | ')'
            | '*'
            | '-'
            | '/'
            | '<'
            | '='
            | '>'
            | '?'
            | '@'
            | '['
            | '\\'
            | ']'
            | '^'
            | '_'
            | '`'
            | '{'
            | '|'
            | '}'
            | ','
            | '.'
            | ':'
    )
}

/// Converts a nickname to a URL-safe slug format for API requests
///
/// This function handles special characters and non-ASCII characters in nicknames
/// by encoding them into a format that can be safely used in URLs. Characters that
/// are not ASCII or are one of the slugify2 separator symbols are converted to their Unicode code points
/// surrounded by hyphens.
///
/// # Arguments
///
/// * `nickname` - The player nickname to slugify
///
/// # Returns
///
/// Returns `Cow<'_, str>` - Borrowed if no conversion needed, Owned if conversion occurred
///
/// # Examples
///
/// ```
/// use ddapi_rs::prelude::slugify2;
///
/// // ASCII-only nicknames without special symbols are returned as-is
/// assert_eq!(slugify2("Player1"), "Player1");
///
/// // Special symbols and non-ASCII characters are encoded
/// assert_eq!(slugify2("Player@"), "Player-64-");
/// assert_eq!(slugify2("玩家"), "-29609--23478-");
///
/// // Mixed characters
/// assert_eq!(slugify2("Test_Player"), "Test-95-Player");
/// ```
pub fn slugify2(nickname: &str) -> Cow<'_, str> {
    let needs_processing = nickname
        .chars()
        .any(|c| is_slugify2_symbol(c) || (c as u32) >= NON_ASCII_CHARACTER_THRESHOLD);

    if !needs_processing {
        return Cow::Borrowed(nickname);
    }

    let mut result = String::with_capacity(nickname.len() * 4);

    for c in nickname.chars() {
        if is_slugify2_symbol(c) || (c as u32) >= NON_ASCII_CHARACTER_THRESHOLD {
            write!(&mut result, "-{}-", c as u32).unwrap();
        } else {
            result.push(c);
        }
    }

    Cow::Owned(result)
}

/// Encodes a nickname for safe use in URLs
///
/// This function ensures that nicknames containing special characters, spaces,
/// or non-ASCII characters are properly URL-encoded. ASCII nicknames without
/// control characters are returned as-is for better performance.
///
/// # Arguments
///
/// * `nickname` - The player nickname to URL-encode
///
/// # Returns
///
/// Returns `Cow<'_, str>` -
/// - `Cow::Borrowed` if the nickname is already URL-safe (ASCII without control characters)
/// - `Cow::Owned` with URL-encoded string if encoding is required
///
/// # Examples
///
/// ```
/// use ddapi_rs::prelude::encode;
///
/// // Safe ASCII nicknames are returned without changes
/// assert_eq!(encode("Player1"), "Player1");
/// assert_eq!(encode("abc_XYZ"), "abc_XYZ");
///
/// // Characters requiring encoding are properly handled
/// assert_eq!(encode("Player Server"), "Player%20Server");
/// assert_eq!(encode("Player@Server"), "Player%40Server");
/// assert_eq!(encode("玩家"), "%E7%8E%A9%E5%AE%B6");
/// assert_eq!(encode("emoji🎮"), "emoji%F0%9F%8E%AE");
///
/// // Special cases
/// assert_eq!(encode(""), "");
/// assert_eq!(encode("a b"), "a%20b");
/// ```
pub fn encode(nickname: &str) -> Cow<'_, str> {
    if nickname
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~')
    {
        Cow::Borrowed(nickname)
    } else {
        urlencoding::encode(nickname)
    }
}
