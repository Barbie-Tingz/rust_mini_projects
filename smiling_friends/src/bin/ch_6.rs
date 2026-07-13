/* Case Types & Emotional States

**The bit:** Not every case is the same, and not every employee is in the same headspace —
time to give the agency some categories to work with.

**Your mission:** Add enums for case types and moods, and use pattern matching to respond
differently depending on which variant you're dealing with.*/

// 1. Define a CaseType enum with a few variants (e.g. MinorSadness, ExistentialCrisis, CursedObject).
// 2. Define a Mood enum with a few variants (e.g. Calm, Anxious, Feral).
// 3. Add a case_type field (CaseType) to your Client struct from Chapter 5, and a mood field
//    (Mood) to your Employee struct.
// 4. Write a function that takes a CaseType and uses match to return a recommended action
//    as a string for each variant.
// 5. Write a function that takes a Mood and uses match to return a short description.
// 6. Call both functions from main with example data and print the results.
// 7. (Optional) Give one variant a piece of data it carries along, e.g. CursedObject(String)
//    naming the specific object, and match on that data too.