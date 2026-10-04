// Use bundled SPDX license texts so generation works offline on every platform.
// This hook operates on cargo-generate's temporary template copy.
variable::set("copyright_year", system::date().year.to_string());
let license_expression = variable::get("license");
file::delete("LICENSE");
file::rename(`hooks/licenses/${license_expression}.txt`, "LICENSE");
file::delete("hooks/licenses");
