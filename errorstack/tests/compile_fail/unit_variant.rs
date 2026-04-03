use errorstack::ErrorStack;

#[derive(ErrorStack)]
enum Bad {
    Oops,
}

fn main() {}
