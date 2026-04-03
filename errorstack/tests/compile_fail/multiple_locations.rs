use errorstack::ErrorStack;

#[derive(ErrorStack)]
enum Bad {
    Oops {
        #[location]
        loc1: &'static std::panic::Location<'static>,
        #[location]
        loc2: &'static std::panic::Location<'static>,
    },
}

fn main() {}
