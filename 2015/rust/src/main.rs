#[path = "_2015/_1/run.rs"]
pub mod _2015_one;

#[path = "_2015/_2/run.rs"]
pub mod _2015_two;

fn main() {
    _2015_one::run();
    _2015_two::run();
}
