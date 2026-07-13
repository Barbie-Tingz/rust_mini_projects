/* Multiple Branches, Multiple Crises

**The bit:** Several agency branches are all handling their own disasters at the same
time, and none of them are waiting around for the others.

**Your mission:** Simulate multiple branches working concurrently using threads, with
some shared state between them.*/

// 1. Write a function representing one branch handling a case (e.g. it sleeps briefly,
//    then prints a message).
// 2. Spawn a thread using std::thread::spawn that runs this function, representing one branch.
// 3. Spawn 2-3 more threads representing other branches, each with slightly different
//    behavior or timing.
// 4. Store the JoinHandles in a Vec, and call .join() on each so main waits for every
//    branch to finish.
// 5. Share a piece of state (e.g. a shared counter of resolved cases) across threads using
//    Arc<Mutex<...>>.
// 6. Have each thread lock the mutex and increment the shared counter.
// 7. Print the final counter value after all threads finish, to confirm it reflects every
//    branch's work.