# Sorting algorithms in Rust for learning purposes

Right now there's is five implemented:

    1. Bubble sort
    2. Selection sort
    3. Insertion sort
    4. Merge sort
    5. Quick sort
    6. Radix sort

The idea is to implement the following:

    1. Random quick sort
    2. Counting sort

Accept an optional positional parameter with the number of randomly generated numbers for sorting:

```
cargo run -- 10000
```

```
cargo run --release -- 10000
```

And output some similiar to this:

```
Length of v: 10000

Bubble sort:
        Swap counter: 24655297
        Comp counter: 96710328
        Total ops:    121365625
        Duration:     250 ms

Selection sort:
        Swap counter: 9874
        Comp counter: 50004999
        Total ops:    50014873
        Duration:     74 ms

Insertion sort:
        Swap counter: 24655297
        Comp counter: 24665291
        Total ops:    49320588
        Duration:     36 ms

Merge sort:
        Swap counter: 253047
        Comp counter: 113040
        Total ops:    366087
        Duration:     3 ms

```

The "Merge sort" is god damn fast, isn't? :smile:
