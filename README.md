<h1 align="center">
  <img src="https://socialify.git.ci/luisbernardinello/competitive-rust/image?font=Raleway&language=1&name=1&owner=1&pattern=Solid&theme=Auto" alt="competitive-rust" width="650" height="220" />
</h1>

<p align="center">
  <img src="https://img.shields.io/badge/Code%20License-MIT-blue.svg" alt="License" />
  <img src="https://img.shields.io/badge/Language-Rust-dea584?logo=rust" alt="Rust" />
  <img src="https://img.shields.io/badge/Status-Active-brightgreen" alt="Status" />
</p>

_In this repository you will see a collection of my dsa and cp solutions implemented in Rust :crab:_

## :balloon: Online Judges :balloon:

- #### [BeeCrowd](https://www.beecrowd.com/)
- #### [CodeWars](https://codewars.com/)
- #### [LeetCode](https://leetcode.com/)

### :wrench: Problem Creation Script :wrench:

This repository includes a shell script `new_problem.sh` to easily create new problem files and add them to Cargo.toml:

```bash
./new_problem.sh <platform> <number> <name>
```

Example:

```bash
./new_problem.sh leetcode 000 "problem name"
```

When cloning this repository:

- Linux/macOS users can use the script directly
- Windows users should run the script through WSL2

The script will:

1. Create a new Rust file with a basic template
2. Add the appropriate entry to Cargo.toml
3. Set up the directory structure if needed

---

### References

- [Competitive Programming 4](https://cpbook.net/)
- [Introduction to Algorithms](https://mitpress.mit.edu/books/introduction-algorithms-third-edition)
- [Algorithms by Robert Sedgewick](https://algs4.cs.princeton.edu/home/)
