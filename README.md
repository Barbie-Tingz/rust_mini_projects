# Rust Mini Projects

A growing collection of themed Rust mini-project series, built alongside *The Rust Book* — one small, story-driven project per chapter, each building on the last. Made for the **Byte Me** YouTube channel, but built to be cloned and followed along with on your own machine.

Every series in this repo follows the same idea: instead of dry, disconnected exercises, each chapter's concept gets wrapped in a running story so the concepts actually stick — and so the projects are genuinely fun to build, not just technically correct.

---

## Series in This Repo

| Folder | Series | Status |
|---|---|---|
| `smiling_friends/` | **Smiling Associates: A Very Normal Friendship Agency** — a chaotic friendship agency, following all 20 chapters of The Rust Book | In progress |
| _more coming_ | More themed series will be added here over time | Planned |

Each series has its own README inside its folder with the full chapter-by-chapter breakdown (the bit, the mission, and the steps for that chapter).

---

## Getting Started

**1. Clone the repo:**

```bash
git clone https://github.com/Barbie-Tingz/rust_mini_projects.git
cd rust_mini_projects
```

Or click the green "Code" button on GitHub and choose "Download ZIP" if you'd rather not use git.

**2. Make sure you have Rust installed.** If you don't, install it from [rust-lang.org](https://www.rust-lang.org/tools/install), then confirm it worked:

```bash
rustc --version
cargo --version
```

**3. Pick a series folder** (e.g. `smiling_friends/`) and check its own README for that series' chapter list and setup.

**4. Run a specific chapter.** Each chapter lives as its own file in that series' `src/bin/`, named like `ch_2.rs`, `ch_3.rs`, etc. From inside that series' project folder:

```bash
cargo run --bin ch_2
```

Swap `ch_2` for whichever chapter you're working on.

---

## Why This Repo Exists

Learning Rust chapter-by-chapter can feel dry if you're just doing the book's exercises in isolation. This repo turns that same progression into small, connected, story-driven builds — so ownership, enums, error handling, concurrency, and everything else in the book gets tied to something memorable instead of a disconnected drill.

Follow along on YouTube: **Byte Me** — squashing bugs one byte at a time.

---

## Contributing / Following Along

This repo is primarily a learning-in-public project tied to the Byte Me channel, but feel free to fork it, try the chapters yourself, or open an issue if something in a series' instructions is unclear.
