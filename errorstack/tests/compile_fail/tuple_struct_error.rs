use errorstack::ErrorStack;

#[derive(ErrorStack)]
struct Bad(String);

fn main() {}
