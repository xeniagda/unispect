use std::io::BufRead;

use clap::{Parser, ArgAction};

use unic_ucd::name::Name;
use unic_ucd::block::Block;
use unic_ucd::case;
use unic_ucd::category::GeneralCategory;
use unic_ucd::age::Age;
use unic_char_property::EnumeratedCharProperty;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(about = "Displays information about each Unicode character passed", long_about="hello")]
struct Options {
    #[arg(
        short = 'f',
        long,
        default_value_t = {"pecb".to_string()},
        action = ArgAction::Append, help = "What attributes to include, in order. Key:
p: (Code)point (eg. U+e621)
e: UTF-8 Encoding (eg. <e621>)
a: Age/version added (eg. 1.1.0)
c: Category (eg. Ll)
b: Block (eg. [Basic Latin])
C: Case
    i: ignorable case
    c: cased
    f: changes when casefolded
    m: changes when casemapped
    l: changes when lowercased
    u: changes when uppercased
    t: changes when titlecased
    L: lowercase
    U: uppercase"
    )]
    format: String,

    #[arg(short = 'q', long, action = ArgAction::SetTrue, help = "Do not display the name of the character")]
    no_name: bool,

    #[arg(short = 't', long, action = ArgAction::SetTrue, help = "Don't pad each attribute to align them")]
    tight: bool,
}

#[derive(Clone, Copy, Debug)]
enum Display {
    CodePoint,
    Encoding,
    Age,
    Category,
    Block,
    Case,
}

impl TryFrom<char> for Display {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'p' => Ok(Display::CodePoint),
            'e' => Ok(Display::Encoding),
            'a' => Ok(Display::Age),
            'c' => Ok(Display::Category),
            'b' => Ok(Display::Block),
            'C' => Ok(Display::Case),
            x => Err(x)
        }
    }
}

impl Display {
    fn pad_to(&self) -> Option<usize> {
        match self {
            Display::CodePoint => Some(8), // 2 (U+) + 6 (longest codepoint)
            Display::Encoding => Some(10), // 10 = 2 (hex pair width) * 4 (max bytes for utf-8) + 2 (brackets)
            Display::Age => Some(6),
            Display::Category => Some(2),
            Display::Block => Some(25),
            Display::Case => None,
        }
    }

    fn info_for(&self, c: char) -> String {
        match self {
            Display::CodePoint => {
                // U+E621
                let n = c as u32;
                let mut hex = format!("{:X}", n);
                while hex.len() < 4 || hex.len() % 2 == 1 {
                    hex.insert(0, '0');
                }
                // 6
                format!("U+{}", hex)
            }
            Display::Encoding => {
                // <ee98a1>
                let utf_8_encoding = c.to_string().into_bytes();
                let hex_pairs = utf_8_encoding.into_iter().map(|x| format!("{:02x}", x)).collect::<String>();
                let disp = format!("<{}>", hex_pairs);
                disp
            },
            Display::Age => {
                // 1.2.3
                let disp = match Age::of(c).map(|x| x.actual()) {
                    Some(version) => format!("v{}.{}.{}", version.major, version.minor, version.micro),
                    None => "(unknown version)".to_string(),
                };
                disp
            }
            Display::Category => {
                // Category
                // Ll
                let cat = GeneralCategory::of(c);
                cat.abbr_name().to_string()
            },
            Display::Block => {
                // Block
                // [Private Use Area]
                match Block::of(c) {
                    Some(block) => format!("[{}]", block.name),
                    None => "(unknown block)".to_string(),
                }
            },
            Display::Case => {
                // ( c m utL )
                let mut case_str = String::new();

                let mut append_case = |test: fn(char) -> bool, ch: char| {
                    if test(c) {
                        case_str.push(ch);
                    } else {
                        case_str.push(' ');
                    }
                };
                append_case(case::case_ignorable::is_case_ignorable, 'i');
                append_case(case::cased::is_cased, 'c');
                append_case(case::changes_when_casefolded::changes_when_casefolded, 'f');
                append_case(case::changes_when_casemapped::changes_when_casemapped, 'm');
                append_case(case::changes_when_lowercased::changes_when_lowercased, 'l');
                append_case(case::changes_when_uppercased::changes_when_uppercased, 'u');
                append_case(case::changes_when_titlecased::changes_when_titlecased, 't');
                append_case(case::lowercase::is_lowercase, 'L');
                append_case(case::uppercase::is_uppercase, 'U');

                format!("({})", case_str)
            }
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Options::parse();

    let fmts = opts
        .format
        .chars()
        .map(|x| Display::try_from(x))
        .collect::<Result<Vec<_>, _>>();

    let fmts = match fmts {
        Ok(fmts) => fmts,
        Err(c) => {
            eprintln!("Unknown attribute {}", c);
            return Ok(());
        }
    };

    let stdin = std::io::stdin().lock();
    for line in stdin.lines() {
        let line = line?;
        for ch in line.chars() {
            print_unicode_info(ch, &fmts, opts.no_name, opts.tight);
        }
    }
    Ok(())
}

fn print_unicode_info(c: char, fmts: &[Display], no_name: bool, tight: bool) {
    for fmt in fmts {
        let info = fmt.info_for(c);
        match (fmt.pad_to(), tight) {
            (Some(width), false) => {
                print!("{:1$} ", info, width);
            }
            _ => {
                print!("{} ", info);
            }
        }
    }
    if no_name {
        println!();
    } else {
        match Name::of(c) {
            Some(name) => {
                println!("| {}", name);
            }
            None => {
                println!("| (unknown name)");
            }
        }
    }
}
