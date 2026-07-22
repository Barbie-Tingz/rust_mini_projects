/* Case Types & Emotional States

**The bit:** The office fern has started whispering stock tips to anyone who waters it —
filed under CursedObject, naturally. Meanwhile Debra's mood has fully tipped into Feral
after being assigned a second ExistentialCrisis case in the same week.

**Your mission:** Add enums for case types and moods, and use pattern matching to respond
differently depending on which variant you're dealing with.*/

// 1. Define a CaseType enum with exactly these three variants: MinorSadness,
//    ExistentialCrisis, CursedObject.
// 2. Define a Mood enum with exactly these three variants: Calm, Anxious, Feral.
// 3. Add a case_type field (CaseType) to your Client struct from Chapter 5, and a mood
//    field (Mood) to your Employee struct.
// 4. Write a function named recommend_action that takes a CaseType and uses match to
//    return a &str: MinorSadness -> "Send an intern", ExistentialCrisis -> "Escalate to
//    a senior caseworker", CursedObject -> "Do not touch it".
// 5. Write a function named describe_mood that takes a Mood and uses match to return a
//    &str description for each variant.
// 6. Update your struct instances in main to include a case_type and a mood.
// 7. Call both functions from main with your sample data and print the results.