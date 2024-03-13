use std::fs;

fn main() {
    fs::copy(
        "./node_modules/htmx.org/dist/htmx.min.js",
        "./static/htmx.min.js",
    )
    .expect("Could not copy htmx.min.js from node modules to static");
}
