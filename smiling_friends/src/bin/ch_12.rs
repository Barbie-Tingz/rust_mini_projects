/* Case File Search CLI

**The bit:** Somewhere in a filing cabinet is the one case file everyone needs, and
nobody can find it by hand anymore.

**Your mission:** Build a small command-line tool that searches a text file for a
keyword, similar to a mini `grep`.*/

// 1. Create a small text file (e.g. case_files.txt) with several lines representing case entries.
// 2. Read command-line arguments using std::env::args() to accept a search keyword and a
//    file path.
// 3. Read the contents of the file using std::fs::read_to_string.
// 4. Loop over the lines of the file and print any line containing the search keyword.
// 5. Move the search logic into its own function that takes the keyword and the file's
//    contents, and returns the matching lines.
// 6. Handle the case where the file doesn't exist, using Result and a friendly error message.
// 7. (Optional) Add a case-insensitive search mode, controlled by an extra argument or
//    environment variable.