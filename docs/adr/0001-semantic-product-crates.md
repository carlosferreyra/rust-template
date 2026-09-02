# Semantic product crates

Generated workspaces reserve `crates/` for product packages named with the
project prefix and a concrete ownership noun. Repository automation remains the
private `tools/xtask` package behind `cargo xtask`, so template infrastructure
does not masquerade as product architecture; new product crates are introduced
only when a coherent responsibility earns a real seam.
