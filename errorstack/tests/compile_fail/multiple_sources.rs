use errorstack::ErrorStack;

#[derive(ErrorStack)]
enum Bad {
    Oops {
        source: std::io::Error,
        #[source]
        other: std::io::Error,
    },
}

fn main() {}
