// Exclude the CLI crate when the user opts out.
let include_cli = variable::get("include_cli");
if !include_cli {
    ignore::push("crates/{{project-name}}-cli");
}
