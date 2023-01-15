# Minigrep cli
![Tests](https://github.com/amasotti/learning-rust-minigrep/actions/workflows/test.yml/badge.svg?branch=main)


This is a simple cli tool to search for a string in a file. 
It is a project from the book "[The Rust Programming Language](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)", chap. 12 - *An I/O Project*.

## Structure

The project is structured as follows:

~~~
.
├── Cargo.lock
├── Cargo.toml
├── data
│   └── poem
├── README.md
├── src
│   ├── lib.rs
│   └── main.rs
~~~


## Usage



```bash
$ minigrep <search_string> <file_name>
```

The search is per default *case sensitive*. But you can export an environment variable to make it *case insensitive*:

```bash
$ export MINIGREP_IGNORE_CASE=1
$ minigrep <search_string> <file_name>
```

or simply

```bash
$ MINIGREP_IGNORE_CASE=1 minigrep <search_string> <file_name>
```

