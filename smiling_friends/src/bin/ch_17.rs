/* The Full Roster

**The bit:** The agency's staff is a genuinely mixed bag at this point — full-timers,
interns, whatever else has wandered in — and they all need to be dispatched the same way.

**Your mission:** Use trait objects to store and use a mix of different types through
one shared interface.*/

// 1. Make sure you have at least two different types implementing the trait from
//    Chapter 10 (e.g. Employee and Intern).
// 2. Create a Vec<Box<dyn YourTrait>> and push an instance of each type into it.
// 3. Loop over the Vec and call your trait method on each entry, regardless of its
//    concrete type.
// 4. Add a new method to the trait with a default implementation, then override it for
//    just one of your types.
// 5. (Optional) Add a third type implementing the trait, and confirm it slots into the
//    same Vec and loop without any changes to the loop code.