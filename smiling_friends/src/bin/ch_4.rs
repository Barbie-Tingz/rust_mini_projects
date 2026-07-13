/* Who's Holding the Clipboard

**The bit:** The agency has exactly one clipboard, and whoever's holding it is legally
responsible if today's client (a landlord who is also, somehow, a swamp) decides to sue.

**Your mission:** Model the clipboard changing hands between two employees, in a way that
makes it obvious only one person can "have" it at a time. Then find a way for a manager to
look at the clipboard without taking responsibility for it.*/

// 1. Create your main function.
// 2. Create a String representing the clipboard (e.g. "Clipboard: Swamp Landlord Case").
// 3. Write a function that takes ownership of the clipboard (takes a String, returns a String)
//    to represent handing it off to a new employee.
// 4. Call that function, passing the clipboard, and store the returned value in a new variable.
// 5. Try using the original clipboard variable after handing it off. Let the compiler error
//    happen on purpose, read it, then comment that line back out.
// 6. Write a second function that takes a reference (&String) instead, so a manager can
//    "check" the clipboard without taking ownership of it.
// 7. Call that function and confirm you can still use the original variable afterward.
// 8. (Optional) Write a function that takes a mutable reference (&mut String) so the manager
//    could write a note on the clipboard without ever owning it.