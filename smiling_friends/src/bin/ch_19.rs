/* Custom Chaos Macros

**The bit:** After the fire alarm incident (still under investigation), the office needs
a proper chaos-announcement system — and somebody, against everyone's advice, insists on
touching the one raw pointer nobody asked them to touch.

**Your mission:** Write a small custom macro, and take a careful, contained look at
an unsafe block.*/

// 1. Write a declarative macro named pandemonium using macro_rules! that, when called
//    with no arguments as pandemonium!(), prints "CHAOS AT THE AGENCY!!!".
// 2. Extend pandemonium! to accept one string argument, so pandemonium!("the printer is
//    possessed") prints "CHAOS AT THE AGENCY: the printer is possessed!!!".
// 3. Call pandemonium!() and pandemonium!("...") from main with two different messages
//    of your own.
// 4. Write a small unsafe block that creates an i32, takes a raw pointer to it with `as
//    *const i32`, and dereferences that pointer to print its value.
// 5. Add a comment directly above the unsafe block explaining exactly why it's safe here
//    (the pointer points to a valid, still-alive local variable).