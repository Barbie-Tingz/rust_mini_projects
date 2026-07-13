/* Custom Chaos Macros

**The bit:** Some things need to be announced with maximum drama, repeatedly, and
typing it out every time is getting old.

**Your mission:** Write a small custom macro, and take a careful, contained look at
an unsafe block.*/

// 1. Write a simple declarative macro using macro_rules! that prints a themed message
//    (e.g. pandemonium!() prints a chaos announcement).
// 2. Extend the macro to accept an argument, e.g. pandemonium!("the printer is possessed"),
//    and include it in the printed message.
// 3. Call your macro from main a couple of times with different arguments.
// 4. Write a small unsafe block that does something simple and clearly contained, e.g.
//    dereferencing a raw pointer to a number you created yourself.
// 5. Add a comment explaining exactly why the unsafe block is safe in this specific case.
// 6. (Optional) Look at a simple trait with an associated type, just to see the syntax.