# Smiling Associates: A Very Normal Friendship Agency

A mini-project series that follows *The Rust Book* chapter by chapter. Each project introduces the new concept from that chapter while building directly on top of the previous project's code.

The theme: you run a deeply under-resourced friendship agency in a world where reality is optional and HR has given up. Employees get dispatched to resolve clients' problems, which range from "mildly sad" to "cursed by a sentient vending machine" to "technically already dead but in denial about it." Nobody at this company questions anything anymore. Every Rust concept maps onto some piece of agency chaos — employees, clients, case files, the increasingly haunted office.

> Original characters/world, inspired by the unhinged, pastel-nightmare "friendship agency" tone of shows like *Smiling Friends* — no copyrighted names, characters, or assets are used. Tone is absurdist/cartoon-violence-adjacent, not genuinely dark — think "getting flattened by an anvil and being fine" not anything real-world graphic.

---

## Chapter 2 — "Guess the Client's Mood"
**Book topic:** Programming a Guessing Game

**The bit:** A client legally changed his name to "Sandwich" and is convinced everyone's jealous of it. Nobody knows what mood he'll be in today — could be anywhere from "at peace with it" to "feral."

**Your mission:** Build a program that picks a secret mood number and lets the player keep guessing until they land on it, giving a hint each time they're off. **Twist:** Sandwich is impatient — the longer it takes to guess, the worse his mood gets. Every so often (you decide how often), the secret number itself creeps upward, so a guess that would've been right two turns ago might be wrong now.

**Think about:**
- How do you get a random number, and keep the program running until the guess is right?
- What should happen if someone types something that isn't a number?
- Where in your loop does it make sense to check "has it been long enough to bump the mood?" — and what should change when it does?

**Hints if you're stuck:**
- The `rand` crate (add it to `Cargo.toml`) has a way to generate a number within a range — look at `rand::thread_rng()` and `.gen_range()`.
- Reading input is `std::io::stdin().read_line(&mut some_string)` — note it takes a mutable reference.
- "Keep running until X happens" is what `loop` + a `break` is for.
- Input comes in as text, so you'll need to convert it to a number — look up `.parse()` and what it returns (hint: it can fail).
- `match` pairs nicely with comparing two numbers — look at `std::cmp::Ordering`.
- For the twist, your secret number needs to be declared as `mut` so it can change. A simple guess counter (increment it once per loop) is enough to decide "every N guesses, bump the mood" — no need for anything fancier like timers.
- Consider printing something when the mood shifts, so the player actually notices the ground moved under them.

*This one maps closely onto the book's own example, so work through that version first, then swap in the theme and layer the twist on top.*

---

## Setup

```bash
cd ch02-guess-the-mood
cargo run
```

Requires a recent stable Rust toolchain (`rustup update stable`).

---

## Progress Tracker

- [ ] Ch 2 — Guess the Client's Mood
