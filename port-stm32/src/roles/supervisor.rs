use proto::CoreLink;

/// Start all control stuff. This function HAS to return, as its supposed to only spawn
/// tasks.
pub fn start<T: CoreLink>(_link: &T) {}
pub fn init() {}
