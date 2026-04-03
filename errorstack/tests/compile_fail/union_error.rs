use errorstack::ErrorStack;

#[derive(ErrorStack)]
union Bad {
    a: u32,
    b: f32,
}

fn main() {}
