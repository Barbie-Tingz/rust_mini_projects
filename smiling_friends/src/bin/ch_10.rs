/* Anyone Can Take a Case

**The bit:** Doesn't matter if you're a seasoned employee or a brand-new intern — anyone
can be sent out to attempt a cheer-up, as long as they're capable of it.

**Your mission:** Define a shared trait, implement it for more than one type, and write a
generic function that works with anything implementing it.*/

// 1. Define a trait (e.g. CanSmile) with one method, such as attempt_cheer_up(&self) -> String.
// 2. Implement that trait for your Employee struct.
// 3. Define a second type (e.g. a new Intern struct) and implement the same trait for it.
// 4. Write a generic function that takes any type implementing your trait and calls its
//    method on it.
// 5. Call that generic function once with an Employee and once with your second type.
// 6. (Optional) Write a function that takes two string references with an explicit lifetime
//    and returns whichever one is longer.
// 7. Note where the compiler required a lifetime annotation, and why.