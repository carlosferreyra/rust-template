//! Dev automation for the `{{project-name}}` workspace.
#![allow(clippy::print_stdout, clippy::print_stderr)]

mod flags;
mod tasks;

fn main() {
    let flags = flags::Xtask::from_env_or_exit();
    match flags.subcommand {
        flags::XtaskCmd::Check(_) => tasks::check(),
        flags::XtaskCmd::Test(cmd) => tasks::test(cmd),
        flags::XtaskCmd::Build(_) => tasks::build(),
        flags::XtaskCmd::Add(cmd) => tasks::add(cmd),
        flags::XtaskCmd::Coverage(_) => tasks::coverage(),
        flags::XtaskCmd::Publish(cmd) => tasks::publish(cmd),
    }
}
