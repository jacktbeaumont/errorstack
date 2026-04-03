use errorstack::ErrorStack;

#[derive(ErrorStack)]
enum Bad {
    Oops {
        #[stack_source]
        not_a_source: String,
    },
}

fn main() {}
