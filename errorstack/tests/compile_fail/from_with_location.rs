// Checks that combining thiserror's `#[from]` with `#[location]` produces a
// compile error, since `#[from]` generates a `From` impl that cannot capture
// the caller location.
use errorstack::ErrorStack;

#[derive(thiserror::Error, ErrorStack, Debug)]
enum Bad {
    #[error("oops")]
    Oops {
        #[from]
        source: std::io::Error,
        #[location]
        location: &'static std::panic::Location<'static>,
    },
}

fn main() {}
