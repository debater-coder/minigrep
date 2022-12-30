# minigrep

## Description

Search for text in files

**Usage:** `minigrep <query> <file> [OPTIONS]`

**Options:**
| Option | Description |
| --- | --- |
| `--ignore-case` | Performs a case-insensitive search |
| `--case-sensitive` | Performs a case-sensitive search |

Case insensitivity can be enabled by setting the `IGNORE_CASE` environment variable. 
The case options passed to minigrep will override these.

---
This tool was made following the tutorial from the [Chapter 12 of The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) with some small extra features.

## Installation
Go to [Releases](https://github.com/debater-coder/minigrep/releases) and download the latest version for your OS. Add the executable to your path, and you are good to go!
