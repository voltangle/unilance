use proto::CoreLink;

/// Start all control stuff. This function HAS to return, as its supposed to only spawn
/// tasks.
#[allow(unused)]
pub fn start<T: CoreLink>(_link: &T) { }
