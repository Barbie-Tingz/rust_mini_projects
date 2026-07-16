/* Anyone Can Take a Case

**The bit:** Doesn't matter if you're a seasoned employee or a brand-new intern — anyone
can be sent out to attempt a cheer-up, as long as they're capable of it.

**Your mission:** Define a shared trait, implement it for more than one type, and write a
generic function that works with anything implementing it.*/

// 1. Define a trait named CanSmile with one required method:
//    fn attempt_cheer_up(&self) -> String.
// 2. Implement CanSmile for your Employee struct, returning a message that includes its name.
// 3. Define a new struct named Intern with a single field, name (String).
// 4. Implement CanSmile for Intern, returning a different-flavored message that also
//    includes its name.
// 5. Write a generic function named dispatch that takes any type implementing CanSmile
//    (using `T: CanSmile` as a bound) and calls attempt_cheer_up on it, printing the result.
// 6. Call dispatch once with an Employee instance and once with an Intern instance.
// 7. (Optional) Write a function named longer_case that takes two &str case descriptions
//    with an explicit lifetime annotation and returns whichever one is longer.