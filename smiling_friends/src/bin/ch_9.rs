/* When the Mission Goes Wrong

**The bit:** Case #013 — the sequel to the Sandwich case, someone else legally changed
their name to "Baguette" — has physically vanished from the filing cabinet the morning
the follow-up is scheduled. An employee also quit via sticky note mid-shift, so their
assigned cases now belong to nobody.

**Your mission:** Handle those failures gracefully using Result and Option instead of
letting the program panic.*/

// 1. Using the HashMap<String, Client> from Chapter 8, write a function named find_case
//    that takes the HashMap and a key, and returns an Option<&Client> instead of panicking
//    if the key is missing.
// 2. Call find_case from main with a key you know exists, and a key you know doesn't.
//    Use match (or if let / else) to handle both cases and print an appropriate message.
// 3. Write a function named resolve_case that takes an employee name and a case key, and
//    returns a Result<String, String> — Ok with a success message if both exist, Err with
//    a specific failure message if the case isn't found.
// 4. Inside resolve_case, use the `?` operator to return early if find_case (adapted to
//    return a Result instead of Option) fails.
// 5. Call resolve_case from main and print the Ok or Err message using match.
// 6. Test resolve_case with a case key that doesn't exist and confirm your program
//    prints a friendly message instead of crashing.