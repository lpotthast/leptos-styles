//! CSS declarations written as text, checked for structure but not for meaning.

use std::{borrow::Cow, error::Error, fmt, str::FromStr};

/// Error returned when the text of an unchecked declaration is malformed or could inject other
/// declarations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvalidUncheckedDeclaration {
    declaration: String,
    reason: InvalidUncheckedReason,
}

impl InvalidUncheckedDeclaration {
    fn new(declaration: &str, reason: InvalidUncheckedReason) -> Self {
        Self {
            declaration: declaration.trim_matches(is_whitespace).to_owned(),
            reason,
        }
    }

    /// Return the rejected declaration, trimmed.
    ///
    /// For [`UncheckedDeclaration::new`] and [`UncheckedDeclaration::try_new`], the declaration is
    /// `property:value`, with both parts trimmed. For [`UncheckedDeclaration::parse`] and
    /// [`Styles::parse`](crate::Styles::parse), it is the text of the declaration as written. If
    /// a string, a block, or a `url()` is not closed, the declaration reaches to the end of the
    /// input.
    #[must_use]
    pub fn declaration(&self) -> &str {
        &self.declaration
    }

    /// Return the rule that the declaration breaks.
    #[must_use]
    pub const fn reason(&self) -> InvalidUncheckedReason {
        self.reason
    }
}

impl fmt::Display for InvalidUncheckedDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid CSS declaration {:?}: {}",
            self.declaration, self.reason
        )
    }
}

impl Error for InvalidUncheckedDeclaration {}

/// The rule that the text of an [`UncheckedDeclaration`] breaks. `Display` writes a short
/// description.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum InvalidUncheckedReason {
    /// A declaration has no `:` between the property name and the value.
    MissingSeparator,
    /// The value is empty or only whitespace.
    MissingValue,
    /// A property name starts with `--` but is not followed by name characters.
    InvalidCustomPropertyName,
    /// A standard property name is not an ASCII identifier.
    InvalidPropertyName,
    /// A `;` outside of strings, `url()`, and blocks would start another declaration.
    TopLevelSemicolon,
    /// A `!` outside of strings, `url()`, and blocks could raise the priority, as in `!important`.
    TopLevelExclamationMark,
    /// Parentheses, brackets, or braces are not balanced.
    Unbalanced,
    /// A `{` appears outside of a block.
    TopLevelBrace,
    /// A `\` escape appears outside of strings and `url()`.
    Escape,
    /// The text contains a comment.
    Comment,
    /// The text contains a control character other than whitespace.
    ControlCharacter,
    /// A non-ASCII character comes directly before `url(`, where browsers disagree on the tokens.
    NonAsciiBeforeUrl,
    /// An unquoted `url()` has no closing `)`.
    UnterminatedUrl,
    /// An unquoted `url()` contains whitespace before its end.
    WhitespaceInUrl,
    /// An unquoted `url()` ends with a `\` or escapes a newline.
    InvalidEscapeInUrl,
    /// An unquoted `url()` contains a quote or a `(`.
    InvalidCharacterInUrl,
    /// A string has no closing quote.
    UnterminatedString,
    /// A string contains a newline that is not escaped.
    NewlineInString,
}

impl fmt::Display for InvalidUncheckedReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::MissingSeparator => "missing ':' separator",
            Self::MissingValue => "missing property value",
            Self::InvalidCustomPropertyName => {
                "custom property names must be `--` followed by name characters"
            }
            Self::InvalidPropertyName => "property names must be ASCII identifiers",
            Self::TopLevelSemicolon => "top-level ';' would start another declaration",
            Self::TopLevelExclamationMark => "top-level '!' is not allowed",
            Self::Unbalanced => "unbalanced parentheses, brackets, or braces",
            Self::TopLevelBrace => "braces are only allowed inside a block",
            Self::Escape => "escapes are only allowed inside strings and url()",
            Self::Comment => "comments are not allowed",
            Self::ControlCharacter => "control characters are not allowed",
            Self::NonAsciiBeforeUrl => "non-ASCII character before url()",
            Self::UnterminatedUrl => "unterminated url()",
            Self::WhitespaceInUrl => "whitespace inside unquoted url()",
            Self::InvalidEscapeInUrl => "invalid escape inside url()",
            Self::InvalidCharacterInUrl => "invalid character inside url()",
            Self::UnterminatedString => "unterminated string",
            Self::NewlineInString => "newline inside string",
        })
    }
}

/// A CSS declaration written as text, for CSS that `typed-css` does not support.
///
/// The property name and the value are not checked against any CSS grammar, so the browser may
/// ignore a declaration with a typo. They are checked for structure, so that the text cannot add
/// other declarations or raise its own priority. The rules are:
///
/// - The property name is an identifier, such as `display` or `-webkit-line-clamp`, or a custom
///   property name such as `--accent`.
/// - The value is not empty, and its quotes, parentheses, brackets, and braces are balanced.
/// - The value has no `;` or `!` outside of strings, `url()`, and blocks, which are functions and
///   text inside parentheses or brackets. A data URL such as `url(data:image/png;base64,AAAA)` is
///   fine.
/// - The value has no braces outside of a block.
/// - The value has no comments, no escapes outside of strings and `url()`, and no control
///   characters. A non-ASCII character directly before `url(` is also rejected, because
///   browsers may disagree on whether it is part of a name.
///
/// The property name and the value are trimmed. A standard property name is also lowercased. A
/// custom property name keeps its case, because custom property names are case-sensitive.
///
/// `Display` writes the declaration as `property:value;`.
///
/// ```rust
/// use leptos_styles::UncheckedDeclaration;
///
/// let declaration = UncheckedDeclaration::new(" Display ", "grid");
/// assert_eq!(declaration.property_name(), "display");
///
/// let image = UncheckedDeclaration::new("background-image", "url(data:image/png;base64,AAAA)");
/// assert_eq!(image.to_string(), "background-image:url(data:image/png;base64,AAAA);");
///
/// let error = UncheckedDeclaration::try_new("color", "red; position: fixed").unwrap_err();
/// assert_eq!(error.declaration(), "color:red; position: fixed");
/// assert!(UncheckedDeclaration::try_new("color", "red !important").is_err());
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct UncheckedDeclaration {
    property: Cow<'static, str>,
    value: Cow<'static, str>,
}

impl UncheckedDeclaration {
    /// Create an unchecked declaration from text that is known to be valid, such as literals.
    ///
    /// # Panics
    ///
    /// Panics if the property name or the value breaks one of the rules in the type
    /// documentation. Use [`UncheckedDeclaration::try_new`] for text that is only known at
    /// runtime.
    #[track_caller]
    #[must_use]
    pub fn new(
        property: impl Into<Cow<'static, str>>,
        value: impl Into<Cow<'static, str>>,
    ) -> Self {
        let (property, value) = (property.into(), value.into());
        if let Err(error) = validate(&property, &value) {
            panic!("{error}");
        }
        Self::normalized(property, value)
    }

    /// Attempt to create an unchecked declaration, for example from text that users entered.
    ///
    /// # Errors
    ///
    /// Returns an error if the property name or the value breaks one of the rules in the type
    /// documentation.
    pub fn try_new<'a>(
        property: impl Into<Cow<'a, str>>,
        value: impl Into<Cow<'a, str>>,
    ) -> Result<Self, InvalidUncheckedDeclaration> {
        let (property, value) = (property.into(), value.into());
        validate(&property, &value)?;
        Ok(Self::normalized(
            Cow::Owned(property.into_owned()),
            Cow::Owned(value.into_owned()),
        ))
    }

    /// Parse one declaration, such as `color: red`. A trailing `;` is allowed.
    ///
    /// # Errors
    ///
    /// Returns an error if the input has no `:`, or if the property name or the value breaks one
    /// of the rules in the type documentation.
    pub fn parse(declaration: &str) -> Result<Self, InvalidUncheckedDeclaration> {
        let parsed = || {
            let (property, value) = declaration
                .split_once(':')
                .ok_or(InvalidUncheckedReason::MissingSeparator)?;
            let value = value.trim_matches(is_whitespace);
            let value = value.strip_suffix(';').unwrap_or(value);
            check_property(property)?;
            check_value(value)?;
            Ok(Self::normalized(
                Cow::Owned(property.trim_matches(is_whitespace).to_owned()),
                Cow::Owned(value.trim_matches(is_whitespace).to_owned()),
            ))
        };
        parsed().map_err(|reason| InvalidUncheckedDeclaration::new(declaration, reason))
    }

    /// Parse a `;`-separated declaration list and skip empty declarations.
    ///
    /// A `;` inside a string, `url()`, or a block does not split declarations.
    pub(crate) fn parse_list(css: &str) -> Result<Vec<Self>, InvalidUncheckedDeclaration> {
        let mut boundaries = Vec::new();
        let scanned = scan(css, |index, token| {
            if token == ';' {
                boundaries.push(index);
            }
            Ok(())
        });
        if let Err(reason) = scanned {
            // The scan stops inside the declaration that follows the last `;` that it found.
            let start = boundaries.last().map_or(0, |end| end + 1);
            return Err(InvalidUncheckedDeclaration::new(&css[start..], reason));
        }

        let mut declarations = Vec::new();
        let mut start = 0;
        for end in boundaries.into_iter().chain(std::iter::once(css.len())) {
            let piece = &css[start..end];
            if !piece.trim_matches(is_whitespace).is_empty() {
                declarations.push(Self::parse(piece)?);
            }
            start = end + 1;
        }
        Ok(declarations)
    }

    /// Return the property name, trimmed and, unless it is a custom property, lowercased.
    #[must_use]
    pub fn property_name(&self) -> &str {
        &self.property
    }

    /// Return the trimmed value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Create a declaration from a property name and a value that passed [`validate`].
    fn normalized(property: Cow<'static, str>, value: Cow<'static, str>) -> Self {
        let property = trim(property);
        let property =
            if property.starts_with("--") || !property.bytes().any(|b| b.is_ascii_uppercase()) {
                property
            } else {
                Cow::Owned(property.to_ascii_lowercase())
            };
        Self {
            property,
            value: trim(value),
        }
    }
}

impl fmt::Display for UncheckedDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{};", self.property, self.value)
    }
}

impl FromStr for UncheckedDeclaration {
    type Err = InvalidUncheckedDeclaration;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

/// Check a property name and a value, and report a failure as the declaration `property:value`.
fn validate(property: &str, value: &str) -> Result<(), InvalidUncheckedDeclaration> {
    check_property(property)
        .and_then(|()| check_value(value))
        .map_err(|reason| {
            let declaration = format!(
                "{}:{}",
                property.trim_matches(is_whitespace),
                value.trim_matches(is_whitespace)
            );
            InvalidUncheckedDeclaration::new(&declaration, reason)
        })
}

/// Remove CSS whitespace from both ends, without copying a borrowed text.
fn trim(text: Cow<'static, str>) -> Cow<'static, str> {
    match text {
        Cow::Borrowed(text) => Cow::Borrowed(text.trim_matches(is_whitespace)),
        Cow::Owned(text) => {
            let trimmed = text.trim_matches(is_whitespace);
            if trimmed.len() == text.len() {
                Cow::Owned(text)
            } else {
                Cow::Owned(trimmed.to_owned())
            }
        }
    }
}

/// Return whether `c` continues a name. CSS counts every non-ASCII character as a name character, but
/// this crate rejects control characters everywhere.
fn is_name_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '-' || c == '_' || (!c.is_ascii() && !c.is_control())
}

fn check_property(property: &str) -> Result<(), InvalidUncheckedReason> {
    let property = property.trim_matches(is_whitespace);
    if let Some(name) = property.strip_prefix("--") {
        return if name.is_empty() || !name.chars().all(is_name_char) {
            Err(InvalidUncheckedReason::InvalidCustomPropertyName)
        } else {
            Ok(())
        };
    }
    let body = property.strip_prefix('-').unwrap_or(property);
    let mut chars = body.chars();
    let starts_like_identifier = chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_');
    if starts_like_identifier && chars.all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') {
        Ok(())
    } else {
        Err(InvalidUncheckedReason::InvalidPropertyName)
    }
}

fn check_value(value: &str) -> Result<(), InvalidUncheckedReason> {
    let value = value.trim_matches(is_whitespace);
    if value.is_empty() {
        return Err(InvalidUncheckedReason::MissingValue);
    }
    scan(value, |_, token| match token {
        ';' => Err(InvalidUncheckedReason::TopLevelSemicolon),
        _ => Err(InvalidUncheckedReason::TopLevelExclamationMark),
    })
}

/// Scan a declaration value or list the way the CSS tokenizer groups it.
///
/// The scan reads the input forwards with the token rules of CSS Syntax Level 3, so it groups
/// strings, `url()` tokens, and blocks exactly like a browser. `on_top_level` receives each `;`
/// and `!` outside of them and decides whether to accept it. The scan rejects comments, escapes
/// outside of strings and `url()`, and control characters, so that it never has to model them.
///
/// [CSS Syntax Level 3 tokenization](https://www.w3.org/TR/css-syntax-3/#tokenization)
fn scan(
    input: &str,
    mut on_top_level: impl FnMut(usize, char) -> Result<(), InvalidUncheckedReason>,
) -> Result<(), InvalidUncheckedReason> {
    let mut tokenizer = Tokenizer {
        chars: input.char_indices().collect(),
        position: 0,
    };
    let mut closers = Vec::new();

    while let Some((index, c)) = tokenizer.next() {
        match c {
            '"' | '\'' => tokenizer.consume_string(c)?,
            '#' if tokenizer.peek(0).is_some_and(is_name_char) => tokenizer.consume_name()?,
            '@' if tokenizer.starts_ident(0) => tokenizer.consume_name()?,
            '<' if tokenizer.peek_str("!--") => tokenizer.position += 3,
            '-' if tokenizer.peek_str("->") => tokenizer.position += 2,
            _ if tokenizer.starts_number(-1) => {
                tokenizer.position -= 1;
                tokenizer.consume_numeric()?;
            }
            _ if tokenizer.starts_ident(-1) => {
                tokenizer.position -= 1;
                if tokenizer.consume_ident_like()? == IdentLike::Function {
                    closers.push(')');
                }
            }
            '(' => closers.push(')'),
            '[' => closers.push(']'),
            // CSS Syntax drafts parse a declaration that fails because of a top-level `{}` block
            // again as a nested rule, which could turn the rest of the text into declarations.
            '{' if closers.is_empty() => return Err(InvalidUncheckedReason::TopLevelBrace),
            '{' => closers.push('}'),
            ')' | ']' | '}' => {
                if closers.pop() != Some(c) {
                    return Err(InvalidUncheckedReason::Unbalanced);
                }
            }
            '\\' => return Err(InvalidUncheckedReason::Escape),
            '/' if tokenizer.peek(0) == Some('*') => return Err(InvalidUncheckedReason::Comment),
            ';' | '!' if closers.is_empty() => on_top_level(index, c)?,
            c if is_forbidden_control(c) => return Err(InvalidUncheckedReason::ControlCharacter),
            _ => {}
        }
    }

    if closers.is_empty() {
        Ok(())
    } else {
        Err(InvalidUncheckedReason::Unbalanced)
    }
}

/// What an ident-like token turned out to be.
#[derive(PartialEq, Eq)]
enum IdentLike {
    /// An identifier, or an unquoted `url()` token, which contains no block.
    Ident,
    /// A function token, which opens a block that a `)` closes.
    Function,
}

/// A cursor over the characters of the input.
struct Tokenizer {
    chars: Vec<(usize, char)>,
    position: usize,
}

impl Tokenizer {
    fn next(&mut self) -> Option<(usize, char)> {
        let next = self.chars.get(self.position).copied();
        self.position += 1;
        next
    }

    /// Return the character `offset` positions after the cursor.
    fn peek(&self, offset: isize) -> Option<char> {
        self.position
            .checked_add_signed(offset)
            .and_then(|index| self.chars.get(index))
            .map(|&(_, c)| c)
    }

    fn peek_str(&self, expected: &str) -> bool {
        expected
            .chars()
            .enumerate()
            .all(|(offset, c)| self.peek(offset.cast_signed()) == Some(c))
    }

    /// Return whether the characters from `offset` start an ident sequence.
    fn starts_ident(&self, offset: isize) -> bool {
        match self.peek(offset) {
            Some('-') => self
                .peek(offset + 1)
                .is_some_and(|c| is_name_start_char(c) || c == '-' || c == '\\'),
            Some('\\') => true,
            Some(c) => is_name_start_char(c),
            None => false,
        }
    }

    /// Return whether the characters from `offset` start a number.
    fn starts_number(&self, offset: isize) -> bool {
        let is_digit = |offset| self.peek(offset).is_some_and(|c: char| c.is_ascii_digit());
        match self.peek(offset) {
            Some('+' | '-') => {
                is_digit(offset + 1) || (self.peek(offset + 1) == Some('.') && is_digit(offset + 2))
            }
            Some('.') => is_digit(offset + 1),
            Some(c) => c.is_ascii_digit(),
            None => false,
        }
    }

    fn consume_while(&mut self, mut condition: impl FnMut(char) -> bool) {
        while self.peek(0).is_some_and(&mut condition) {
            self.position += 1;
        }
    }

    /// Consume a name. An escape, which would continue the name, is rejected.
    ///
    /// CSS Syntax drafts disagree on whether every non-ASCII character continues a name. The
    /// difference only matters directly before `url(`, where it decides whether a `url()` token
    /// starts, so such a name is rejected.
    fn consume_name(&mut self) -> Result<(), InvalidUncheckedReason> {
        let start = self.position;
        self.consume_while(is_name_char);
        if self.peek(0) == Some('\\') {
            return Err(InvalidUncheckedReason::Escape);
        }
        let name = &self.chars[start..self.position];
        if self.peek(0) == Some('(')
            && name.len() > 3
            && !name[name.len() - 4].1.is_ascii()
            && is_url(&name[name.len() - 3..])
        {
            return Err(InvalidUncheckedReason::NonAsciiBeforeUrl);
        }
        Ok(())
    }

    fn consume_numeric(&mut self) -> Result<(), InvalidUncheckedReason> {
        if matches!(self.peek(0), Some('+' | '-')) {
            self.position += 1;
        }
        self.consume_while(|c| c.is_ascii_digit());
        if self.peek(0) == Some('.') && self.peek(1).is_some_and(|c| c.is_ascii_digit()) {
            self.position += 1;
            self.consume_while(|c| c.is_ascii_digit());
        }
        if matches!(self.peek(0), Some('e' | 'E')) {
            let digits_at = if matches!(self.peek(1), Some('+' | '-')) {
                2
            } else {
                1
            };
            if self.peek(digits_at).is_some_and(|c| c.is_ascii_digit()) {
                self.position += digits_at.cast_unsigned();
                self.consume_while(|c| c.is_ascii_digit());
            }
        }
        if self.starts_ident(0) {
            self.consume_name()?;
        } else if self.peek(0) == Some('%') {
            self.position += 1;
        }
        Ok(())
    }

    fn consume_ident_like(&mut self) -> Result<IdentLike, InvalidUncheckedReason> {
        let start = self.position;
        self.consume_name()?;
        if self.peek(0) != Some('(') {
            return Ok(IdentLike::Ident);
        }
        let is_url = is_url(&self.chars[start..self.position]);
        self.position += 1;
        if !is_url {
            return Ok(IdentLike::Function);
        }
        let mut lookahead = 0;
        while self.peek(lookahead).is_some_and(is_whitespace) {
            lookahead += 1;
        }
        if matches!(self.peek(lookahead), Some('"' | '\'')) {
            return Ok(IdentLike::Function);
        }
        self.consume_url()?;
        Ok(IdentLike::Ident)
    }

    fn consume_url(&mut self) -> Result<(), InvalidUncheckedReason> {
        self.consume_while(is_whitespace);
        loop {
            match self.next().map(|(_, c)| c) {
                None => return Err(InvalidUncheckedReason::UnterminatedUrl),
                Some(')') => return Ok(()),
                Some(c) if is_whitespace(c) => {
                    self.consume_while(is_whitespace);
                    return match self.next() {
                        Some((_, ')')) => Ok(()),
                        _ => Err(InvalidUncheckedReason::WhitespaceInUrl),
                    };
                }
                Some('\\') => match self.next() {
                    None | Some((_, '\n' | '\r' | '\x0c')) => {
                        return Err(InvalidUncheckedReason::InvalidEscapeInUrl);
                    }
                    Some(_) => {}
                },
                Some('"' | '\'' | '(') => {
                    return Err(InvalidUncheckedReason::InvalidCharacterInUrl);
                }
                Some(c) if c.is_control() => return Err(InvalidUncheckedReason::ControlCharacter),
                Some(_) => {}
            }
        }
    }

    fn consume_string(&mut self, quote: char) -> Result<(), InvalidUncheckedReason> {
        loop {
            match self.next().map(|(_, c)| c) {
                None => return Err(InvalidUncheckedReason::UnterminatedString),
                Some('\\') => {
                    if self.next().is_none() {
                        return Err(InvalidUncheckedReason::UnterminatedString);
                    }
                }
                Some('\n' | '\r' | '\x0c') => return Err(InvalidUncheckedReason::NewlineInString),
                Some(c) if c == quote => return Ok(()),
                Some(c) if is_forbidden_control(c) => {
                    return Err(InvalidUncheckedReason::ControlCharacter);
                }
                Some(_) => {}
            }
        }
    }
}

/// Return whether `name` is `url`, in any case.
fn is_url(name: &[(usize, char)]) -> bool {
    name.iter()
        .map(|&(_, c)| c.to_ascii_lowercase())
        .eq("url".chars())
}

const fn is_whitespace(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r' | '\x0c')
}

fn is_forbidden_control(c: char) -> bool {
    c.is_control() && !is_whitespace(c)
}

fn is_name_start_char(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_' || (!c.is_ascii() && !c.is_control())
}

#[cfg(test)]
mod tests {
    use assertr::prelude::*;

    use super::*;

    #[test]
    fn standard_property_names_are_trimmed_and_lowercased() {
        assert_that!(UncheckedDeclaration::new(" Touch-Action ", "none").property_name())
            .is_equal_to("touch-action");
        assert_that!(UncheckedDeclaration::new("-webkit-line-clamp", "2").property_name())
            .is_equal_to("-webkit-line-clamp");
    }

    #[test]
    fn custom_property_names_keep_their_case() {
        assert_that!(UncheckedDeclaration::new(" --ThemeAccent ", "tomato").property_name())
            .is_equal_to("--ThemeAccent");
    }

    #[test]
    fn malformed_property_names_are_rejected() {
        for property in [
            "",
            "--",
            "1color",
            "color:red",
            "co lor",
            "color;",
            "--a b",
            "--a\u{85}",
        ] {
            assert_that!(UncheckedDeclaration::try_new(property, "red")).is_err();
        }
    }

    #[test]
    fn structured_values_are_accepted() {
        for value in [
            "grid",
            "\"Open Sans\", system-ui",
            "url(data:image/png;base64,AAAA)",
            "url( \"a;b.png\" )",
            "calc(100% - var(--gap, 4px))",
            "attr(data-label string, \"a!\")",
            "[full-start] 1fr [full-end]",
            "'semi;colon'",
            "#url(a)",
            "1url(a)",
            "-url(a;b)",
            "<!-- red -->",
            "1e-3px",
        ] {
            assert_that!(UncheckedDeclaration::try_new("content", value)).is_ok();
        }
    }

    #[test]
    fn values_that_could_inject_declarations_are_rejected() {
        let cases = [
            (
                "red; position: fixed",
                InvalidUncheckedReason::TopLevelSemicolon,
            ),
            (
                "red !important",
                InvalidUncheckedReason::TopLevelExclamationMark,
            ),
            ("\"unterminated", InvalidUncheckedReason::UnterminatedString),
            ("calc(1px", InvalidUncheckedReason::Unbalanced),
            ("1px)", InvalidUncheckedReason::Unbalanced),
            (
                "url(a(b);color:red)",
                InvalidUncheckedReason::InvalidCharacterInUrl,
            ),
            ("u\\rl(a(b);color:red)", InvalidUncheckedReason::Escape),
            ("red /* ; */", InvalidUncheckedReason::Comment),
            ("url(a b)", InvalidUncheckedReason::WhitespaceInUrl),
            ("a\u{0}b", InvalidUncheckedReason::ControlCharacter),
            ("x{} position:fixed", InvalidUncheckedReason::TopLevelBrace),
            ("a\u{85}b", InvalidUncheckedReason::ControlCharacter),
            // A hash, an at-keyword, or `<!--` before `url(` keeps the browser from reading a
            // `url()` token, so the `/*` starts a comment that hides the `)`.
            (
                "#url(/*)'*/);position:fixed;--x:'",
                InvalidUncheckedReason::Comment,
            ),
            (
                "@url(/*)'*/);position:fixed;--x:'",
                InvalidUncheckedReason::Comment,
            ),
            (
                "(<!--url(a')) ;position:fixed;--x:'))",
                InvalidUncheckedReason::InvalidCharacterInUrl,
            ),
            (
                "\u{a0}url(/*)'*/);position:fixed;--x:'",
                InvalidUncheckedReason::NonAsciiBeforeUrl,
            ),
            ("   ", InvalidUncheckedReason::MissingValue),
        ];
        for (value, reason) in cases {
            assert_that!(
                UncheckedDeclaration::try_new("color", value)
                    .unwrap_err()
                    .reason()
            )
            .is_equal_to(reason);
        }
    }

    #[test]
    #[should_panic(expected = "invalid CSS declaration \"color:red;\"")]
    fn new_panics_on_invalid_text() {
        let _ = UncheckedDeclaration::new("color", "red;");
    }

    #[test]
    fn try_new_accepts_borrowed_text() {
        let (property, value) = (String::from(" Gap "), String::from("4px"));
        let declaration = UncheckedDeclaration::try_new(&property, &value).unwrap();
        assert_that!(declaration.to_string()).is_equal_to("gap:4px;");
    }

    #[test]
    fn errors_report_the_rejected_declaration() {
        let rejected = |result: Result<UncheckedDeclaration, InvalidUncheckedDeclaration>| {
            result.unwrap_err().declaration().to_owned()
        };
        assert_that!(rejected(UncheckedDeclaration::try_new(
            " color ",
            " red !important "
        )))
        .is_equal_to("color:red !important");
        assert_that!(rejected(UncheckedDeclaration::try_new("1color", "red")))
            .is_equal_to("1color:red");
        assert_that!(rejected(UncheckedDeclaration::parse(" color : red ! ; ")))
            .is_equal_to("color : red ! ;");
        for (css, declaration) in [
            ("color: red; padding: 'x", "padding: 'x"),
            (
                "color: red; width: calc(1px; gap: 0",
                "width: calc(1px; gap: 0",
            ),
            (
                "color: red; margin: 0 !important; gap: 0",
                "margin: 0 !important",
            ),
            ("color: red; 1color: red", "1color: red"),
            ("color red; gap: 0", "color red"),
        ] {
            let error = UncheckedDeclaration::parse_list(css).unwrap_err();
            assert_that!(error.declaration()).is_equal_to(declaration);
        }
    }

    #[test]
    fn parse_recovers_property_value_pairs() {
        let declaration = UncheckedDeclaration::parse(" margin : 5px; ").unwrap();
        assert_that!(declaration.property_name()).is_equal_to("margin");
        assert_that!(declaration.value()).is_equal_to("5px");
        assert_that!(
            UncheckedDeclaration::parse("margin 5px")
                .unwrap_err()
                .reason()
        )
        .is_equal_to(InvalidUncheckedReason::MissingSeparator);
        assert_that!(UncheckedDeclaration::parse(": 5px").unwrap_err().reason())
            .is_equal_to(InvalidUncheckedReason::InvalidPropertyName);
    }

    #[test]
    fn parse_list_splits_only_on_top_level_semicolons() {
        let declarations = UncheckedDeclaration::parse_list(
            ";;color: red; background: url(data:x;y) no-repeat;;font-family: 'a;b'",
        )
        .unwrap();
        let rendered: Vec<String> = declarations
            .iter()
            .map(|d| format!("{}:{}", d.property_name(), d.value()))
            .collect();
        assert_that!(rendered).is_equal_to(vec![
            "color:red".to_string(),
            "background:url(data:x;y) no-repeat".to_string(),
            "font-family:'a;b'".to_string(),
        ]);
    }
}
