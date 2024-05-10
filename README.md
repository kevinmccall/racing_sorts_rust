# Racing Sorts

A program to visualize different sorting algorithims concurrently. This program
works on systems that use termcap/terminfo databases for console.

## How to use

Run the command

```shell
cargo run --release -- -f <file_to_sort> <sorting_algorithim> [<sorting_algorithm> ...]
```

NOTE: You may not have more sorting algorithms than the number of rows on your screen.

## Terminfo

The Terminfo package helps me interact with the terminfo/termcap database and to
make commands. It is a wrapper around the database so that I won't have to interact
with unsafe code.

## Credit

I borrowed and modified Kirill Vasiltsov's (jlkiri) quick sort implementation for this project.
Here is his (Github)[https://github.com/jlkiri] and (Website)[https://www.kirillvasiltsov.com/]
