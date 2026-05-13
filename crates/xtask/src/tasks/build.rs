use super::check::run;
use super::test::test;
use crate::flags::Test;

pub fn build() {
    test(Test { filter: None });
    run(&["build", "--workspace", "--release"]);
    {% if include_cli %}
    run(&["run", "--package", "{{project-name}}-cli", "--release"]);
    {% endif %}
}
