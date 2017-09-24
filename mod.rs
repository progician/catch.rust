pub mod stack;

#[macro_use]
mod catch;

mod stack_test;

fn main() {
    stack_test::test_case_fn();
}
