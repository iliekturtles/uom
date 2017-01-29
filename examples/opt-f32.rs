#[inline(never)]
#[no_mangle]
fn print(velocity: f32) {
    println!("{:?}", velocity);
}

#[inline(never)]
#[no_mangle]
fn calc() -> f32 {
    let length = 100.0_f32 * 0.9144;
    let time = 1.0_f32 * 60.0;

    length / time
}

fn main() {
    print(calc());
}
