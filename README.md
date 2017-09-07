# paco - The field pair counter
A PoC CLI app in Rust for KB Labs

## Building

```bash
$ cargo build --release
```

## Usage

`paco` takes as input two tsv files with two columns. The program then counts
the number of occurances of each pair in the files and prints out a sorted
list.

```bash
$ cat pairs.tsv
a   b
1   b
a   c
1   b
$ ./paco pairs.tsv pairs.tsv
4:      ["1-b"]
2:      ["a-b", "a-c"]
```

The equivalent if using `awk`, `sort`, and `uniq` would be as follows:

```bash
$ awk '{ print $1 "-" $2 }' pairs.tsv pairs.tsv | sort | uniq -c | sort -nr
```

## License

See [LICENSE](LICENSE).

