# Visar - A Sorting Visualizer in Rust

![](https://github.com/FoundedNahte/visar/blob/main/example/selection.gif)

## Running

Running Locally:

```bash
cd visar
cargo build --release
cd ./target/release
vis.exe --algo <ALGO> --size <SIZE>
```

## Flags
Algorithm Keywords (REQUIRED):
* Selection Sort [selection]
* Insertion Sort [insertion]
* Bubble Sort [bubble]
* Shell Sort [shell]
* Radix Sort [radix]
* Odd Even Sort [oddeven]

Size (OPTIONAL):
* Default = 100

Takes a 16-bit unsigned integer.

## License
[MIT](https://choosealicense.com/licenses/mit/)