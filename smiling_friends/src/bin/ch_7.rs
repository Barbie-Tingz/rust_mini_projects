/* Organize the Agency

**The bit:** The health inspector is coming Thursday, and it's just been discovered the
entire office "system" is one unlabeled folder. Everything gets sorted into modules
overnight in a panic.

**Your mission:** Split your existing code into modules so related things live together.*/

// 1. Create a module named `employees` containing the Employee struct and its related functions.
// 2. Create a module named `clients` containing the Client struct, CaseType, Mood, and related functions.
// 3. Create a module named `office` for anything agency-wide (e.g. a function announcing today's chaos).
// 4. Move the relevant structs/functions out of main.rs and into their new modules.
// 5. In main.rs, bring each module in with `mod` and `use` statements.
// 6. Update any struct/function calls in main to use their new module paths.
// 7. Run `cargo build` and fix any privacy errors by adding `pub` to items that need to be
//    reached from outside their module.