use errorstack::ErrorStack;

#[derive(ErrorStack)]
enum Bad {
    Oops(String),
}

fn main() {}
