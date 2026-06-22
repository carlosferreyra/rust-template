use super::check::run;
use super::test::test;
use crate::flags::Test;

pub fn build() {
    test(Test { filter: None });
    run(&["build", "--workspace", "--release"]);
}
