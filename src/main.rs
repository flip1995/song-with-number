use clap::{App, Arg};
use csv;
use english_numbers::{convert, Formatting};
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
#[allow(dead_code)]
struct Song {
    index: usize,
    song: String,
    year: u16,
    artist: String,
    genre: String,
    lyrics: String,
}

impl std::fmt::Display for Song {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{} - {}", self.artist, self.song)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("Song finder")
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .required(true)
                .help("the input file"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .value_name("N")
                .required(true)
                .help("the number to find in the lyrics"),
        )
        .arg(
            Arg::with_name("newer")
                .long("newer")
                .value_name("MIN_YEAR")
                .help("only search songs newer than the specified year"),
        )
        .arg(
            Arg::with_name("older")
                .long("older")
                .value_name("MAX_YEAR")
                .help("only search songs older than the specified year"),
        )
        .arg(
            Arg::with_name("genre")
                .long("genre")
                .value_name("GENRE")
                .help("only search songs with the specified genre"),
        )
        .get_matches();

    let file = matches.value_of("file").expect("required argument");
    let mut rdr = csv::Reader::from_path(Path::new(file)).expect("couldn't open csv file");
    let csv_entries: csv::DeserializeRecordsIter<_, Song> = rdr.deserialize();

    let number = matches.value_of("number").expect("required argument");
    let number_usize = number.parse::<i64>().expect("NaN");
    let newer = matches
        .value_of("newer")
        .map(|n| n.parse::<u16>().expect("NaN"));
    let older = matches
        .value_of("older")
        .map(|n| n.parse::<u16>().expect("NaN"));
    let genre = matches.value_of("genre");

    let formatting = Formatting::none();
    let dash_formatting = add_dash_formatting(formatting);
    let space_formatting = add_space_formatting(formatting);
    let dash_space_formatting = add_space_formatting(dash_formatting);
    let conjunction_space_formatting = add_conjunction_formatting(space_formatting);
    let conjunction_space_dash_formatting = add_dash_formatting(conjunction_space_formatting);
    let comma_space_formatting = add_comma_formatting(space_formatting);
    let comma_space_dash_formatting = add_dash_formatting(comma_space_formatting);
    let all_lower_formatting = add_conjunction_formatting(comma_space_dash_formatting);

    let number_reprs = vec![
        number.to_string(),
        convert(number_usize, formatting),
        convert(number_usize, dash_formatting),
        convert(number_usize, space_formatting),
        convert(number_usize, dash_space_formatting),
        convert(number_usize, conjunction_space_formatting),
        convert(number_usize, conjunction_space_dash_formatting),
        convert(number_usize, comma_space_formatting),
        convert(number_usize, comma_space_dash_formatting),
        convert(number_usize, all_lower_formatting),
    ];

    let songs = csv_entries
        .flatten()
        .filter(|song| {
            if let Some(newer) = newer {
                song.year >= newer
            } else {
                true
            }
        })
        .filter(|song| {
            if let Some(older) = older {
                song.year < older
            } else {
                true
            }
        })
        .filter(|song| {
            if let Some(genre) = genre {
                song.genre.to_lowercase() == genre.to_lowercase()
            } else {
                true
            }
        })
        .filter(|song| {
            let lyrics = song.lyrics.to_lowercase();
            number_reprs.iter().any(|repr| lyrics.contains(repr))
        })
        .collect::<Vec<Song>>();

    for song in &songs {
        println!("{}", song);
    }

    let amount_songs = songs.len();
    println!(
        "{} entr{} found",
        amount_songs,
        if amount_songs == 1 { "y" } else { "ies" },
    );

    Ok(())
}

fn add_dash_formatting(f: Formatting) -> Formatting {
    let mut formatting = f;
    formatting.dashes = true;
    formatting
}

fn add_space_formatting(f: Formatting) -> Formatting {
    let mut formatting = f;
    formatting.spaces = true;
    formatting
}

fn add_comma_formatting(f: Formatting) -> Formatting {
    let mut formatting = f;
    formatting.commas = true;
    formatting
}

fn add_conjunction_formatting(f: Formatting) -> Formatting {
    let mut formatting = f;
    formatting.conjunctions = true;
    formatting
}
