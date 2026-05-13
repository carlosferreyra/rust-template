use super::check::{check, run};
use crate::flags::Test;

pub fn test(cmd: Test) {
    check();
    let mut args = vec!["test", "--workspace"];
    let filter;
    if let Some(ref f) = cmd.filter {
        filter = f.clone();
        args.push(&filter);
    }
    run(&args);
}
