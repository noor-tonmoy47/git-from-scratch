# git-from-scratch

A learning project to build a **basic version of Git from scratch** in Rust.  
The goal is to understand **Git internals** by re-implementing core commands step by step.

---

## 📦 Installation

Clone the repository and build it:

```bash
git clone https://github.com/noor-tonmoy47/git-from-scratch.git
cd git-from-scratch
cargo build --release
```

This will create a binary under `target/release/git-from-scratch`.

---

## 🚀 Usage

Currently, only the `init` command is implemented.

```bash
./target/release/git-from-scratch init
```

This will create a new `.git` directory in the current working directory with the following structure:

```
.git/
├── objects/
├── refs/
└── HEAD
```

- `objects/` → stores Git objects (blobs, trees, commits)  
- `refs/` → stores references (branches, tags)  
- `HEAD` → a special file that points to the current branch (e.g., `refs/heads/master`)  

---
## 📅 Commands Built So far
- [x] `init` – initialize a new repository
---
## 🛠 How the `init` command was built

The `init` command mimics `git init` by:

1. **Creating a `.git` directory**  
   - This directory acts as the "database" for the repository.  
   - All commits, objects, and references are stored here.  

2. **Setting up the `objects` directory**  
   - Git stores all data as **objects** (blobs, trees, commits, tags).  
   - At this stage, the directory is empty, but later we’ll populate it.  

3. **Setting up the `refs` directory**  
   - This will contain references to commits, such as branches (`refs/heads/master`).  

4. **Creating the `HEAD` file**  
   - This file tells Git which branch you are currently on.  
   - By default, it points to `refs/heads/master`.  

- [Pro Git, Chapter 10: Git Internals](https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain)  
### Thought process

- The starting point of Git is its `.git` folder.  
- To build Git step by step, we first need a minimal repository layout that can later be expanded.  
- By implementing `init`, we establish the foundation for adding commands like `add`, `commit`, and `log`.  
