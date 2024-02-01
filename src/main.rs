use std::time::Duration;
use std::io::{Read, Write, stdout, StdoutLock};
use std::cmp::{max, min};

use ndarray::{prelude::*, ShapeError};
use termion::async_stdin;
use termion::raw::{IntoRawMode, RawTerminal};

const CONFIG_DIGITS: &[u8] = include_bytes!("../config/digits.txt");
const CONFIG_LETTERS_EN: &[u8] = include_bytes!("../config/letters_en.txt");
const CONFIG_LETTERS_RU: &[u8] = include_bytes!("../config/letters_ru.txt");

const SYM_H: usize = 5;
const SYM_W: usize = 6;

const DIGITS_BUF_LEN: usize = SYM_W * 10;
const LETTERS_EN_BUF_LEN: usize = SYM_W * 26;
const LETTERS_RU_BUF_LEN: usize = SYM_W * 32;

type ConsBuf = Array2<char>;

fn parse_config_bytes(bytes: &[u8], config_h: usize, config_w: usize) -> Result<ConsBuf, ShapeError> {
    Array::from_iter(
        bytes
        .iter()
        .filter_map(|&b| match b {
            b'#' => Some('#'),
            b' ' => Some(' '),
            _ => None,
        }))
    .into_shape((config_h, config_w))
}

fn display(out: &mut RawTerminal<StdoutLock>, buf: &ConsBuf, frame_start: usize, count: usize) {
    for line in buf.axis_iter(Axis(0)) {
        let s: String = 
            line
            .iter()
            .cycle()
            .skip(frame_start)
            .take(count * SYM_W)
            .collect();
        writeln!(out, "{}\r", s).unwrap();
    }
}

fn insert_symbol(display: &mut ConsBuf, sym_view: ArrayView2<char>, pos: usize) {
    let pos = (pos * SYM_W) as i32;
    display.slice_mut(s![0..SYM_H, (pos as i32)..(pos + SYM_W as i32)]).assign(&sym_view);
}

fn main() {
    let digits_buf = parse_config_bytes(CONFIG_DIGITS, SYM_H, DIGITS_BUF_LEN).unwrap();
    let letters_en_buf = parse_config_bytes(CONFIG_LETTERS_EN, SYM_H, LETTERS_EN_BUF_LEN).unwrap();
    let letters_ru_buf = parse_config_bytes(CONFIG_LETTERS_RU, SYM_H, LETTERS_RU_BUF_LEN).unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let line: Vec<char> = 
        input
        .to_uppercase()
        .chars()
        .into_iter()
        .filter(|ch| 
            ch.is_alphanumeric() 
            || ch.is_alphabetic() 
            || (ch.is_whitespace() && !ch.is_control()))
        .collect();

    let mut display_buf = ConsBuf::from_elem((SYM_H, line.len() * SYM_W), '_');
    
    for (i, ch) in line.iter().enumerate() {
        let buf: &ConsBuf;
        let diff_from: char;
        match ch {
            '0'..='9' => {
                diff_from = '0';
                buf = &digits_buf;
            }
            'A'..='Z' => {
                diff_from = 'A';
                buf = &letters_en_buf;
            }
            'А'..='Я' => {
                diff_from = 'А';
                buf = &letters_ru_buf;
            }
            ' ' => {
                let whsp = ConsBuf::from_elem((SYM_H, SYM_W), ' ');
                insert_symbol(&mut display_buf, whsp.view(), i);
                continue;
            }
            _ => continue
        }

        let mut chars_utf16: [u16;2] = [0;2];
        diff_from.encode_utf16(&mut chars_utf16);
        ch.encode_utf16(&mut chars_utf16[1..2]);
        let diff = (chars_utf16[1] - chars_utf16[0]) as i32;

        let pos_in_buf = SYM_W * diff as usize;
        let slice = buf.slice(s![0..SYM_H, (pos_in_buf as i32)..((pos_in_buf + SYM_W) as i32)]);
        insert_symbol(&mut display_buf, slice, i)
    }

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut a_stdin = async_stdin().bytes();

    let mut frame_start: usize = 0;
    let mut speed: usize = 2;
    loop {
        write!(stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1))
             .unwrap();

        display(&mut stdout, &display_buf, frame_start, 4);
        write!(stdout, "-----------------------\n\r").unwrap();
        write!(stdout, "Speed = {} sym/s\n\r", speed).unwrap();
        stdout.flush().unwrap();

        frame_start += 1;
        if frame_start >= display_buf.len_of(Axis(1)) {
            frame_start = 0;
        }

        match a_stdin.next() {
            Some(key) => {
                match key {
                    Ok(b'q') => break,
                    Ok(67) => speed = min(speed + 1, 10),
                    Ok(68) => speed = max(speed - 1, 1),
                    _ => {}
                }
            }
            None => {}
        }
        std::thread::sleep(Duration::from_millis((1000/6) / speed as u64));
    }
}
