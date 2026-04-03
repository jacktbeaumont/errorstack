pub use errorstack_derive::ErrorStack;

/// An error within a typed error stack, preserving full error context as
/// errors propagate up the call stack.
///
/// Each error may carry the source-code [`location`] where it was constructed
/// and a reference to the next error in the stack via [`stack_source`].
///
/// Typically derived via `#[derive(ErrorStack)]` rather than implemented by
/// hand.
///
/// [`stack_source`]: ErrorStack::stack_source
/// [`location`]: ErrorStack::location
pub trait ErrorStack: std::error::Error + Send + Sync + 'static {
    /// Returns the source code location where this error was constructed,
    /// or [`None`] if location tracking is not available for this error.
    fn location(&self) -> Option<&'static std::panic::Location<'static>>;

    /// Returns the next error in the chain, or [`None`] if this is the root
    /// cause.
    fn stack_source(&self) -> Option<&dyn ErrorStack> {
        None
    }
}

impl std::error::Error for Box<dyn ErrorStack + Send + Sync> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        (**self).source()
    }
}

impl ErrorStack for Box<dyn ErrorStack + Send + Sync> {
    fn location(&self) -> Option<&'static std::panic::Location<'static>> {
        (**self).location()
    }

    fn stack_source(&self) -> Option<&dyn ErrorStack> {
        (**self).stack_source()
    }
}
