# uuid-generator
Command-line UUID v4 and v7 generator with multiple output formats

## usage

```
cargo run -- --help
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running `target\debug\uuid-generator.exe --help`
Command-line UUID v4 and v7 generator with multiple output formats

Usage: uuid-generator.exe [OPTIONS]

Options:
  -t, --type <TYPE>
          UUID type to generate: v4 or v7

          [default: v4]
          [possible values: v4, v7]

  -c, --count <COUNT>
          Number of UUIDs to generate

          [default: 1]

  -f, --format <FORMAT>
          Output format

          Possible values:
          - lower-dash:    lowercase with dashes
          - upper-dash:    UPPERCASE with dashes
          - lower-no-dash: lowercase without dashes
          - upper-no-dash: UPPERCASE without dashes
          - all:           print all formats

          [default: all]

  -q, --quiet
          Quiet mode: suppress header and line labels, print only the raw UUID value(s)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
