# Visar - A Sorting Visualizer in Rust

![](https://github.com/FoundedNahte/visar/blob/main/example/selection.gif)

## Running
Installation:

The executable can be downloaded in the "Releases" tab.

Building Locally:

```bash
cd vis
cargo build
cd ./target/release
visar.exe --algo <ALGO> --size <SIZE>
```
Pressing **"Space"** will start the animation  

Pressing **"Space"** during the animation will **queue** another animation once the current one is done. 

There is no pausing feature yet, so the animation will run until either finished or the window is closed.

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