# riops

**R**ust **I**nput/**O**utput **P**arallel **S**earching — a fast command-line search tool that uses multi-threading to grep for matches across files.

## Features

- 🔍 Search a single file or recursively across a directory
- ⚡ Parallel directory search powered by [Rayon](https://github.com/rayon-rs/rayon)
- 🔠 Case-insensitive matching (`--ignore-case`)
- 🔤 Whole-word matching (`--whole-word`)
- 📄 Summary output mode (`--simple-search`)

## Installation

Requires [Rust](https://www.rust-lang.org/tools/install) (edition 2024).

```bash
git clone https://github.com/your-username/riops
cd riops
cargo build --release
```

The compiled binary will be at `target/release/riops`.

## Usage

```
rps [OPTIONS] --query <QUERY>
```

### Options

| Flag                      | Short | Description                                             |
| ------------------------- | ----- | ------------------------------------------------------- |
| `--query <QUERY>`         | `-q`  | The search term                                         |
| `--file-path <FILE_PATH>` | `-f`  | Path to a single file to search                         |
| `--directory [DIR]`       | `-d`  | Recursively search a directory (default: `.`)           |
| `--ignore-case`           | `-i`  | Case-insensitive matching                               |
| `--whole-word`            | `-w`  | Match whole words only                                  |
| `--simple-search`         | `-s`  | Print a one-line summary per file instead of each match |
| `--help`                  | `-h`  | Print help                                              |

`--file-path` and `--directory` are mutually exclusive.

### Examples

```bash
# Search a single file
rps --query "hello" --file-path ./notes.txt

# Recursively search the current directory
rps --query "hello" --directory

# Recursively search a specific directory
rps --query "hello" --directory ./docs

# Case-insensitive search
rps -q "hello" -f ./notes.txt --ignore-case

# Whole-word match across a directory
rps -q "log" -d ./logs --whole-word

# Summary output (occurrence counts only)
rps -q "error" -d ./logs --simple-search
```

### Output

**Normal mode** — prints every matching line with its file and line number:

```
File: ./docs/intro.txt
Line 4: hello world
Line 9: say hello again
```

**Simple mode** (`--simple-search`) — prints a one-line count per file:

```
hello in ./docs/intro.txt: 2 occurrences(s)
```

## Dependencies

| Crate     | Version | Purpose                       |
| --------- | ------- | ----------------------------- |
| `clap`    | 4.6.1   | CLI argument parsing          |
| `rayon`   | 1.12.0  | Data parallelism / threading  |
| `walkdir` | 2.5.0   | Recursive directory traversal |

## License

MIT
