/* Quality Assurance (Sort Of)

**The bit:** Corporate mandates a full audit of the cheer-up formula after the pigeon and
Sandwich cases got accidentally double-booked into the same appointment slot last week.

**Your mission:** Write unit tests for logic from earlier chapters.*/

// 1. Pick 2-3 functions from earlier chapters (e.g. your score calculator from Chapter 3,
//    or your trait method from Chapter 10) to test.
// 2. Add a `#[cfg(test)] mod tests { ... }` block at the bottom of your file.
// 3. Inside it, write a `#[test]` function that calls one of your functions with known
//    inputs and checks the result with `assert_eq!`.
// 4. Write a second test that checks an edge case (e.g. a score of exactly 0, or an
//    empty collection).
// 5. Run `cargo test` and confirm all tests pass.
// 6. Intentionally break one function (change a constant or a condition) and confirm the
//    matching test fails — then fix it back.