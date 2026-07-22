/* Case File Search CLI

**The bit:** The swamp landlord's lawyer is calling back in ten minutes and someone needs
every case file that mentions "swamp" before he does.

**Your mission:** Build a small command-line tool that searches a text file for a
keyword, similar to a mini `grep`.*/

// 1. Create a text file named case_files.txt in your project root with exactly these
//    five lines:
//    "Case 001: Client convinced he is a sandwich"
//    "Case 002: Pigeon filed a noise complaint"
//    "Case 003: Cursed vending machine on floor 2"
//    "Case 004: Landlord who is also a swamp"
//    "Case 005: Client legally changed name to Sandwich"
// 2. Read command-line arguments using std::env::args() to accept a search keyword as
//    the first argument (e.g. running `cargo run --bin ch_12 -- Sandwich`).
// 3. Read the contents of case_files.txt using std::fs::read_to_string.
// 4. Loop over the lines of the file and print any line containing the search keyword.
// 5. Move the search logic into a function named search that takes the keyword and the
//    file's contents, and returns a Vec of matching lines.
// 6. Handle the case where case_files.txt doesn't exist, using Result and a friendly
//    error message instead of a panic.
// 7. (Optional) Add a case-insensitive search mode using .to_lowercase() on both the
//    keyword and each line before comparing.