# paco - The field pair counter
A PoC CLI app in Rust for KB Labs

## Building

```bash
$ cargo build --release
```

## Usage

`paco` takes as input a tsv file with two columns. The program then counts the
number of occurances of each pair in the file and prints out a sorted list.

```bash
$ cat pairs.tsv
a   b
1   b
a   c
1   b
$ ./paco pairs.tsv
2:      ["1-b"]
1:      ["a-b", "a-c"]
```

The equivalent if using `awk`, `sort`, and `uniq` would be as follows:

```bash
$ awk '{ print $1 "-" $2 }' pairs.tsv | sort | uniq -c | sort -nr
```

## License

See [LICENSE](LICENSE).

