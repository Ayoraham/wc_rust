#![allow(unused)]

use anyhow::{Result};
use clap::Parser;
use std::{
    fmt::format, fs, io::{self, BufRead, BufReader, Read}, ops::AddAssign,
};

type MyResult<T> = Result<T>;

#[derive(Default)]
struct FIleInfo{
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize
}

impl FIleInfo{
    fn get_file_info(&mut self, buf_reader: &mut Box<dyn BufRead>) -> Result<()>{
        let mut lines = 0_usize;
        let mut words = 0_usize;
        let mut chars = 0_usize;
        let mut bytes = 0_usize;
        loop{
            let mut reader = String::new();
            let byte_count = buf_reader.read_line(&mut reader);
            if byte_count? == 0 {break}
            if reader.ends_with("\n"){lines += 1};
            words += reader.split_whitespace().count();
            chars += reader.chars().count();
            bytes += reader.len();


        }
        self.lines = lines;
        self.words = words;
        self.chars =  chars;
        self.bytes = bytes;
        Ok(())
    }

}


fn wc_formatter(wc_args: &WCArgs, file_info: &FIleInfo) -> Result<String> {
    // lines
    let mut wc = String::new();
    if wc_args.lines {
        wc = format!("{}{:>8}", wc, file_info.lines)
    }
    if wc_args.words {
        wc = format!("{}{:>8}", wc, file_info.words)
    }
    if wc_args.chars {
        wc = format!("{}{:>8}", wc, file_info.chars)
    }
    if wc_args.bytes {
        wc = format!("{}{:>8}", wc, file_info.bytes)
    }
    Ok(wc)
}

impl AddAssign for FIleInfo{
    fn add_assign(&mut self, rhs: Self) {
        self.lines += rhs.lines;
        self.words += rhs.words;
        self.chars += rhs.chars;
        self.bytes += rhs.bytes
    }
}

pub fn run() -> MyResult<()> {
    let mut wc_args = WCArgs::parse();
    if !wc_args.lines && !wc_args.words && !wc_args.bytes && !wc_args.chars {      
        (wc_args.lines, wc_args.words, wc_args.bytes, wc_args.chars) = (true, true, true, false);       
    };
    let mut aggr_file_info= FIleInfo::default();
    let line = String::new();
    for file in &wc_args.files {
        let mut buf_reader = match open(&file) {
            Ok(val) => val,
            Err(e) => {
                eprintln!("{}: {e}", &file);
                continue
            }
        };
        let mut file_info = FIleInfo::default();
        file_info.get_file_info(&mut buf_reader)?;
        let mut wc = wc_formatter(&wc_args, &file_info)?;
        if file != "-" {
        wc.push_str(&format!(" {file}"));
        }
        aggr_file_info += file_info;
        println!("{}", wc)
    }
    if wc_args.files.len() > 1{
        let wc = wc_formatter(&wc_args, &aggr_file_info)?;
        println!("{wc} total")
    }
    Ok(())
}

fn line_counter(buf_reader: &mut Box<dyn BufRead>) -> Result<usize> {
    let mut lines = 0_usize;
    for _ in buf_reader.by_ref().lines() {
        lines += 1;
    }
    Ok(lines)
}

fn word_counter(buf_reader: &mut Box<dyn BufRead>) -> Result<usize> {
    let mut words = 0_usize;
    for line in buf_reader.by_ref().lines() {
        let line = line?;
        words += line.trim().split(" ").count();
    }
    Ok(words)
}

fn char_counter(buf_reader: &mut Box<dyn BufRead>) -> Result<usize> {
    let mut chars = 0_usize;
    for line in buf_reader.by_ref().lines() {
        let line = line?;
        chars += line.chars().count()
    }
    Ok(chars)
}

fn byte_counter(buf_reader: &mut Box<dyn BufRead>) -> Result<usize> {
    let mut bytes = 0_usize;
    for line in buf_reader.by_ref().lines() {
        let line = line?;
        bytes += line.len()
    }
    Ok(bytes)
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct WCArgs {
    /// File(s) you want to operate on
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,

    /// print the newline counts
    #[arg(long = "lines", short = 'l', value_name = "LINES")]
    lines: bool,

    /// print the word counts
    #[arg(long = "words", short = 'w', value_name = "WORDS")]
    words: bool,

    /// print character count
    #[arg(long = "chars", short = 'm', value_name = "CHARACTERS")]
    chars: bool,

    ///  print byte count
    #[arg(
        long = "bytes",
        short = 'c',
        value_name = "BYTES",
        conflicts_with = "chars"
    )]
    bytes: bool,
}

fn open(file: &str) -> Result<Box<dyn BufRead>> {
    match file {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(fs::File::open(file)?))),
    }
}
