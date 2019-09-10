# Song with Number

This tool searches lyrics of songs provided as a CSV file for the specified
number.

## Data

This tool is written for the data from
https://www.kaggle.com/gyani95/380000-lyrics-from-metrolyrics, but can also be
used for the data from https://www.kaggle.com/rakannimer/billboard-lyrics, by
adapting the headers in the CSV file.

_NOTE_: the genre filter doesn't work with the second data set.

## Usage

Download the pre-compiled binaries from the release page or compile the program
yourself.

```
$ song-with-number -h  # or cargo run -- -h
Song finder

USAGE:
    song-with-number [OPTIONS] --file <FILE> --number <N>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <FILE>         the input file
        --genre <GENRE>       only search songs with the specified genre
        --newer <MIN_YEAR>    only search songs newer than the specified year
    -n, --number <N>          the number to find in the lyrics
        --older <MAX_YEAR>    only search songs older than the specified year
```

## License

This project is licensed under the terms of both the MIT license and the Apache
License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

