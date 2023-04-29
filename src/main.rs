use better_panic;

pub mod cmd;

fn main() {
    better_panic::install();
    cmd::init();
}
