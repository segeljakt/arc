/// Converts an item into something which can be sent across threads.
pub trait IntoSendable {
    type T: Send;
    fn into_sendable(self) -> Self::T;
}

/// Converts an item into something which can be shared within a thread.
pub trait IntoSharable {
    type T;
    fn into_sharable(self) -> Self::T;
}
