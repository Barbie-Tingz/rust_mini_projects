/* Sort by Chaos Level

**The bit:** Management wants a weekly "Chaos Leaderboard" for the company newsletter,
ranking every open client by weirdness rating, most unhinged first.

**Your mission:** Use iterators and closures to filter, sort, and summarize your
roster or case list.*/

// 1. Take your Vec<Employee> or Vec<Client> (with a weirdness/chaos rating field) from Chapter 8.
// 2. Use .iter().filter(...) with a closure to get only the entries above a chaos threshold
//    you choose.
// 3. Use .sort_by(...) or .sort_by_key(...) with a closure to sort the collection by chaos
//    rating, descending.
// 4. Use .map(...) to transform the filtered/sorted collection into just names or short
//    summary strings.
// 5. Chain filter, sort, and map together into one pipeline, and print the final result.
// 6. (Optional) Use .fold() to compute a total or average chaos rating across the whole collection.