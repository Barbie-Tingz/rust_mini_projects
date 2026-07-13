# Smiling Associates: A Very Normal Friendship Agency

A mini-project series that follows *The Rust Book* chapter by chapter. Each project introduces the new concept from that chapter while building directly on top of the previous project's code.

The theme: you run a deeply under-resourced friendship agency in a world where reality is optional and HR has given up. Employees get dispatched to resolve clients' problems, which range from "mildly sad" to "cursed by a sentient vending machine" to "technically already dead but in denial about it." Nobody at this company questions anything anymore. Every Rust concept maps onto some piece of agency chaos — employees, clients, case files, the increasingly haunted office.

> Original characters/world, inspired by the unhinged, pastel-nightmare "friendship agency" tone of shows like *Smiling Friends* — no copyrighted names, characters, or assets are used. Tone is absurdist/cartoon-violence-adjacent, not genuinely dark — think "getting flattened by an anvil and being fine" not anything real-world graphic.

---

## Chapter 2 — "Guess the Client's Mood"
**Book topic:** Programming a Guessing Game

**The bit:** A client legally changed his name to "Sandwich" and is convinced everyone's jealous of it. Nobody knows what mood he'll be in today — could be anywhere from "at peace with it" to "feral."

**Your mission:** Build a program that picks a secret mood number and lets the player keep guessing until they land on it, giving a hint each time they're off. **Twist:** Sandwich is impatient — the longer it takes to guess, the worse his mood gets. Every so often (you decide how often), the secret number itself creeps upward, so a guess that would've been right two turns ago might be wrong now.

*Numbered step comments for this one live in `ch_2.rs` in your repo. This one maps closely onto the book's own example, so work through that version first, then swap in the theme and layer the twist on top.*

---

## Chapter 3 — "Cheer-Up Score Calculator"
**Book topic:** Common Programming Concepts

**The bit:** A pigeon filed a formal complaint against the agency, and a caseworker was sent to personally apologize to it.

**Your mission:** Write a function that takes a couple of numbers about the apology (your choice — how many breadcrumbs, how sincere it was, whatever's fun) and turns them into a score. Then print a different message depending on how the score comes out.

*Numbered step comments for this one live in `ch_3.rs` in your repo.*

---

## Chapter 4 — "Who's Holding the Clipboard"
**Book topic:** Understanding Ownership

**The bit:** The agency has exactly one clipboard, and whoever's holding it is legally responsible if today's client (a landlord who is also, somehow, a swamp) decides to sue.

**Your mission:** Model the clipboard changing hands between two employees, in a way that makes it obvious only one person can "have" it at a time. Then find a way for a manager to look at the clipboard without taking responsibility for it.

*Numbered step comments for this one live in `ch_4.rs` in your repo.*

---

## Chapter 5 — "Meet the Employees & Clients"
**Book topic:** Using Structs to Structure Related Data

**The bit:** The agency finally needs actual records instead of vibes — time to define who its employees and clients actually are.

**Your mission:** Define structs for your core character types, so later chapters have something to build on.

*Numbered step comments for this one live in `ch_5.rs` in your repo.*

---

## Chapter 6 — "Case Types & Emotional States"
**Book topic:** Enums and Pattern Matching

**The bit:** Not every case is the same, and not every employee is in the same headspace — time to give the agency some categories to work with.

**Your mission:** Add enums for case types and moods, and use pattern matching to respond differently depending on which variant you're dealing with.

*Numbered step comments for this one live in `ch_6.rs` in your repo.*

---

## Chapter 7 — "Organize the Agency"
**Book topic:** Managing Growing Projects with Packages, Crates, and Modules

**The bit:** The codebase has gotten as messy as the actual office — time to give everything a proper filing system.

**Your mission:** Split your existing code into modules so related things live together.

*Numbered step comments for this one live in `ch_7.rs` in your repo.*

---

## Chapter 8 — "The Case Files & Staff Roster"
**Book topic:** Common Collections

**The bit:** The agency needs an actual staff roster and a filing cabinet for open cases, instead of just remembering things.

**Your mission:** Store your employees in a Vec and your case files in a HashMap.

*Numbered step comments for this one live in `ch_8.rs` in your repo.*

---

## Chapter 9 — "When the Mission Goes Wrong"
**Book topic:** Error Handling

**The bit:** Missions go sideways constantly at this agency — case files go missing, employees quit mid-shift, clients vanish. The program shouldn't crash every time.

**Your mission:** Handle those failures gracefully using Result and Option instead of letting the program panic.

*Numbered step comments for this one live in `ch_9.rs` in your repo.*

---

## Chapter 10 — "Anyone Can Take a Case"
**Book topic:** Generic Types, Traits, and Lifetimes

**The bit:** Doesn't matter if you're a seasoned employee or a brand-new intern — anyone can be sent out to attempt a cheer-up, as long as they're capable of it.

**Your mission:** Define a shared trait, implement it for more than one type, and write a generic function that works with anything implementing it.

*Numbered step comments for this one live in `ch_10.rs` in your repo.*

---

## Chapter 11 — "Quality Assurance (Sort Of)"
**Book topic:** Writing Automated Tests

**The bit:** Someone should probably check that any of this actually works. Nobody's volunteered, so it falls to you.

**Your mission:** Write unit tests for logic from earlier chapters.

*Numbered step comments for this one live in `ch_11.rs` in your repo.*

---

## Chapter 12 — "Case File Search CLI"
**Book topic:** An I/O Project — Building a Command Line Program

**The bit:** Somewhere in a filing cabinet is the one case file everyone needs, and nobody can find it by hand anymore.

**Your mission:** Build a small command-line tool that searches a text file for a keyword, similar to a mini `grep`.

*Numbered step comments for this one live in `ch_12.rs` in your repo.*

---

## Chapter 13 — "Sort by Chaos Level"
**Book topic:** Functional Language Features — Iterators and Closures

**The bit:** Management wants to know which cases are the biggest disasters, ranked, without anyone having to read them all by hand.

**Your mission:** Use iterators and closures to filter, sort, and summarize your roster or case list.

*Numbered step comments for this one live in `ch_13.rs` in your repo.*

---

## Chapter 14 — "Package the Agency Toolkit"
**Book topic:** More about Cargo and Crates.io

**The bit:** Other branches want to use the same tools this branch built — time to package it up properly instead of copy-pasting files.

**Your mission:** Turn your reusable logic into a proper library crate.

*Numbered step comments for this one live in `ch_14.rs` in your repo.*

---

## Chapter 15 — "The Shared Cursed Object"
**Book topic:** Smart Pointers

**The bit:** Multiple open cases all reference the same cursed prop sitting in the break room, and everyone keeps touching it.

**Your mission:** Model a piece of shared, mutable state referenced from more than one place using `Rc<RefCell<>>`.

*Numbered step comments for this one live in `ch_15.rs` in your repo.*

---

## Chapter 16 — "Multiple Branches, Multiple Crises"
**Book topic:** Fearless Concurrency

**The bit:** Several agency branches are all handling their own disasters at the same time, and none of them are waiting around for the others.

**Your mission:** Simulate multiple branches working concurrently using threads, with some shared state between them.

*Numbered step comments for this one live in `ch_16.rs` in your repo.*

---

## Chapter 17 — "The Full Roster"
**Book topic:** Object Oriented Programming Features

**The bit:** The agency's staff is a genuinely mixed bag at this point — full-timers, interns, whatever else has wandered in — and they all need to be dispatched the same way.

**Your mission:** Use trait objects to store and use a mix of different types through one shared interface.

*Numbered step comments for this one live in `ch_17.rs` in your repo.*

---

## Chapter 18 — "Advanced Case Matching"
**Book topic:** Patterns and Matching

**The bit:** Some cases only make sense to escalate under very specific, oddly particular conditions — a simple if/else won't cut it anymore.

**Your mission:** Use more advanced pattern matching to handle combinations of data at once.

*Numbered step comments for this one live in `ch_18.rs` in your repo.*

---

## Chapter 19 — "Custom Chaos Macros"
**Book topic:** Advanced Features

**The bit:** Some things need to be announced with maximum drama, repeatedly, and typing it out every time is getting old.

**Your mission:** Write a small custom macro, and take a careful, contained look at an unsafe block.

*Numbered step comments for this one live in `ch_19.rs` in your repo.*

---

## Chapter 20 — "The Agency Server"
**Book topic:** Final Project — Building a Multithreaded Web Server

**The bit:** Every branch, every employee, every cursed object — it's all one system now, and clients are submitting requests over the internet like it's a real business.

**Your mission:** Build a small multithreaded web server tying the earlier systems together.

*Numbered step comments for this one live in `ch_20.rs` in your repo.*

---

## Getting Started (For Anyone Following Along)

This repo is meant to be cloned and run on your own machine, chapter by chapter, alongside *The Rust Book*.

**1. Clone the repo:**

```bash
git clone https://github.com/[your-username]/[repo-name].git
cd [repo-name]
```

Or click the green "Code" button on GitHub and choose "Download ZIP" if you'd rather not use git — just unzip it and open the folder afterward.

**2. Make sure you have Rust installed.** If you don't, install it from [rust-lang.org](https://www.rust-lang.org/tools/install), then confirm it worked:

```bash
rustc --version
cargo --version
```

**3. Run the chapter you're working on with Cargo.** Each chapter lives as its own file in `src/bin/`, named like `ch_2.rs`, `ch_3.rs`, etc. From the project root, run:

```bash
cargo run --bin ch_2
```

Swap `ch_2` for whichever chapter you're working on (e.g. `cargo run --bin ch_3`).

**4. Follow along in order.** Each chapter builds on ideas from the last one, and the README below tells you the goal for that chapter without handing you the full solution — try it yourself first, use the steps/hints if you get stuck, then compare notes.

---

## Setup

```bash
cargo run --bin ch_2
```

Swap `ch_2` for any other chapter's binary name (e.g. `ch_10`, `ch_20`) once you're ready for it.

Requires a recent stable Rust toolchain (`rustup update stable`).

---

## Progress Tracker

- [ ] Ch 2 — Guess the Client's Mood
- [ ] Ch 3 — Cheer-Up Score Calculator
- [ ] Ch 4 — Who's Holding the Clipboard
- [ ] Ch 5 — Meet the Employees & Clients
- [ ] Ch 6 — Case Types & Emotional States
- [ ] Ch 7 — Organize the Agency
- [ ] Ch 8 — The Case Files & Staff Roster
- [ ] Ch 9 — When the Mission Goes Wrong
- [ ] Ch 10 — Anyone Can Take a Case
- [ ] Ch 11 — Quality Assurance (Sort Of)
- [ ] Ch 12 — Case File Search CLI
- [ ] Ch 13 — Sort by Chaos Level
- [ ] Ch 14 — Package the Agency Toolkit
- [ ] Ch 15 — The Shared Cursed Object
- [ ] Ch 16 — Multiple Branches, Multiple Crises
- [ ] Ch 17 — The Full Roster
- [ ] Ch 18 — Advanced Case Matching
- [ ] Ch 19 — Custom Chaos Macros
- [ ] Ch 20 — The Agency Server