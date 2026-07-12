/* Guess the Client's Mood 

**The bit:** A client legally changed his name to "Sandwich" and is convinced everyone's jealous of it. 
Nobody knows what mood he'll be in today — could be anywhere from "at peace with it" to "feral."

**Your mission:** Build a program that picks a secret mood number and lets the player keep guessing until they land on it, 
giving a hint each time they're off. **Twist:** Sandwich is impatient — the longer it takes to guess, the worse his mood gets. 
Every so often (you decide how often), the secret number itself creeps upward, so a guess that would've been right two turns ago might be wrong now.

**Hints if you're stuck:**
- The `rand` crate (add it to `Cargo.toml`) has a way to generate a number within a range — look at `rand::thread_rng()` and `.gen_range()`.
- Reading input is `std::io::stdin().read_line(&mut some_string)` — note it takes a mutable reference.
- "Keep running until X happens" is what `loop` + a `break` is for.
- Input comes in as text, so you'll need to convert it to a number — look up `.parse()` and what it returns (hint: it can fail).
- `match` pairs nicely with comparing two numbers — look at `std::cmp::Ordering`.
- For the twist, your secret number needs to be declared as `mut` so it can change. A simple guess counter (increment it once per loop)
 is enough to decide "every N guesses, bump the mood" — no need for anything fancier like timers. 
- Consider printing something when the mood shifts, so the player actually notices the ground moved under them. */