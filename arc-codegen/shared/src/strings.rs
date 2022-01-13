use crate::conversions::IntoSendable;
use crate::conversions::IntoSharable;

pub mod sharable_string {
    use derive_more::Deref;
    use derive_more::From;

    #[derive(From, Deref)]
    #[from(forward)]
    pub struct String(pub std::rc::Rc<std::string::String>);
}

mod sendable_string {
    use derive_more::Deref;
    use derive_more::From;

    #[derive(From, Deref)]
    #[from(forward)]
    pub struct String(pub std::string::String);
}

impl IntoSendable for sharable_string::String {
    type T = sendable_string::String;
    fn into_sendable(self) -> Self::T {
        self.0.as_ref().into()
    }
}

impl IntoSharable for sendable_string::String {
    type T = sharable_string::String;
    fn into_sharable(self) -> Self::T {
        self.0.into()
    }
}

use sharable_string::String;

impl String {
    /// Concatenates `self` with `other`.
    pub fn concat(self, other: Self) -> Self {
        vec![self.as_str(), other.as_str()].join("").into()
    }
    /// Appends `ch` to `self`.
    pub fn append(self, ch: char) -> Self {
        let mut new = self.as_str().to_string();
        new.push(ch);
        new.into()
    }
    /// Returns `true` if `self` contains `other` substring, else `false`.
    pub fn contains(self, other: Self) -> bool {
        self.0.contains(other.as_str())
    }
    /// Returns `true` if `self` contains `other` substring, else `false`.
    pub fn truncate(self, new_len: usize) -> String {
        let mut new = self.as_str().to_string();
        new.truncate(new_len);
        new.into()
    }
}
