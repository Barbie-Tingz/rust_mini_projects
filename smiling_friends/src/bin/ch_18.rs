/* Advanced Case Matching

**The bit:** Some cases only make sense to escalate under very specific, oddly
particular conditions — a simple if/else won't cut it anymore.

**Your mission:** Use more advanced pattern matching to handle combinations of data
at once.*/

// 1. Combine two pieces of data about a case (e.g. its CaseType and a weirdness number)
//    into a tuple or struct.
// 2. Write a match expression that uses a match guard (an `if` condition on the arm) to
//    handle one specific combination, e.g. ExistentialCrisis with weirdness > 80.
// 3. Use an `|` pattern to group two enum variants that should be handled the same way.
// 4. Use a range pattern (e.g. 90..=100) to match a numeric field falling within a
//    specific band.
// 5. Use an `@` binding to capture a value while also matching a range on it, and print
//    both the captured value and the result.
// 6. Add a catch-all `_` arm and confirm your match is exhaustive.