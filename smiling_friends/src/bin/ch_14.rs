/* Package the Agency Toolkit

**The bit:** A rival friendship agency across town heard about your cheer-up formula and
wants to license it — which means it actually has to work outside of one messy file.

**Your mission:** Turn your reusable logic into a proper library crate.*/

// 1. Create a new library crate (`cargo new --lib`) to hold shared logic like structs,
//    enums, and scoring functions.
// 2. Move reusable code from earlier chapters into the library's src/lib.rs, marking the
//    items that need to be public with `pub`.
// 3. Add doc comments (///) to at least two public functions explaining what they do.
// 4. In your existing binary project, add the library as a path dependency in Cargo.toml.
// 5. Import and use a function or struct from the library inside your binary's main.rs.
// 6. Run `cargo doc --open` to view your generated documentation.
// 7. (Optional) Add a short README to the library crate describing what it does.