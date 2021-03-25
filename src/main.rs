use rand::seq::SliceRandom;
use std::fs::File;
use std::io;
use std::io::{stdin, stdout, Read, Write};

const SHIFT_KEY: u8 = 13;

fn main() {
    println!("Game Start");
    main_func();
    pause();
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"\nPress Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read_line(&mut String::new()).unwrap();
}

fn main_func() {
    let servant = Kana {
        sent: get_servant_name(),
    };
    let roman = servant.to_roman();
    let caesar = roman.encryption_caesar(SHIFT_KEY);
    let morse = caesar.to_morse();
    let fgo = morse.to_fgo();

    let mut hint: u8 = 0;
    const MORSE: u8 = 1;
    const CAESAR: u8 = 2;
    const ROMAN: u8 = 3;
    const SERVANT: u8 = 4;

    loop {
        println!("\nDecrypt the following text");
        match hint {
            MORSE => println!("Encrypted text: {}", morse.sent),
            CAESAR => println!("Encrypted text: {}", caesar.sent),
            ROMAN => println!("Encrypted text: {}", roman.sent),
            SERVANT => println!("Correct answer: {}", servant.sent),
            _ => println!("Encrypted text: {}", fgo.sent),
        }
        println!("Input the decrypted text");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line"); // 行の読み込みに失敗しました

        let input: String = input.trim().to_string();

        if input.eq(&servant.sent) {
            println!("Congratulations");
            break;
        } else if input.eq("fgo") {
            hint = 0;
        } else if input.eq("morse") {
            hint = MORSE;
        } else if input.eq("caesar") {
            hint = CAESAR;
        } else if input.eq("roman") {
            hint = ROMAN;
        } else if input.eq("servant") {
            hint = SERVANT;
        } else if input.eq("debug") {
            print_debug_msg(&servant.sent);
        } else if is_exit(&input) {
            break;
        } else {
            println!("Incorrect");
        }
    }
}

fn is_exit(input: &str) -> bool {
    match input {
        "exit" => true,
        "quit" => true,
        "q" => true,
        _ => false,
    }
}

fn print_debug_msg(name: &str) {
    println!("\nencryption");
    let servant = Kana {
        sent: name.to_string(),
    };
    let roman = servant.to_roman();
    let caesar = roman.encryption_caesar(SHIFT_KEY);
    let morse = caesar.to_morse();
    let fgo = morse.to_fgo();
    println!("servant: {}", servant.sent);
    println!("roman: {}", roman.sent);
    println!("caesar: {}", caesar.sent);
    println!("morse: {}", morse.sent);
    println!("fgo: {}", fgo.sent);

    println!("\ndecryption");
    let morse = fgo.to_morse();
    let caesar = morse.to_roman();
    let roman = caesar.encryption_caesar(26 - SHIFT_KEY);
    let servant = roman.to_kana();
    println!("fgo: {}", fgo.sent);
    println!("morse: {}", morse.sent);
    println!("caesar: {}", caesar.sent);
    println!("roman: {}", roman.sent);
    println!("servant: {}", servant.sent);
}

fn get_servant_name() -> String {
    let mut f = File::open("src/data/servant.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let mut v: Vec<String> = Vec::new();
    for content in contents.split_terminator(',') {
        v.push(content.to_string());
    }

    let mut rng = rand::thread_rng();
    v.shuffle(&mut rng);

    match v.get(0) {
        Some(result) => result.trim().to_string(),
        None => "ましゅ".to_string(),
    }
}

struct Kana {
    sent: String,
}
impl Kana {
    fn to_roman(&self) -> Roman {
        let mut tmp = String::new();
        for i in self.sent.chars() {
            if i == ' ' {
                tmp.push(' ');
            } else if i == '・' {
                tmp.push('・');
            } else if i == '＝' {
                tmp.push('＝');
            } else if i == 'ー' {
                tmp.push('ー');
            } else if i == '゛' {
                tmp.push('゛');
            } else {
                tmp.push_str(match_kana_to_roman(i));
            }
        }
        Roman { sent: tmp }
    }
}
fn match_kana_to_roman<'life>(kana: char) -> &'life str {
    match kana {
        'ぁ' => "LA",
        'あ' => "A",
        'ぃ' => "LI",
        'い' => "I",
        'ぅ' => "LU",
        'う' => "U",
        'ぇ' => "LE",
        'え' => "E",
        'ぉ' => "LO",
        'お' => "O",
        'か' => "KA",
        'が' => "GA",
        'き' => "KI",
        'ぎ' => "GI",
        'く' => "KU",
        'ぐ' => "GU",
        'け' => "KE",
        'げ' => "GE",
        'こ' => "KO",
        'ご' => "GO",
        'さ' => "SA",
        'ざ' => "ZA",
        'し' => "SI",
        'じ' => "ZI",
        'す' => "SU",
        'ず' => "ZU",
        'せ' => "SE",
        'ぜ' => "ZE",
        'そ' => "SO",
        'ぞ' => "ZO",
        'た' => "TA",
        'だ' => "DA",
        'ち' => "TI",
        'ぢ' => "DI",
        'っ' => "LTU",
        'つ' => "TU",
        'づ' => "DU",
        'て' => "TE",
        'で' => "DE",
        'と' => "TO",
        'ど' => "DO",
        'な' => "NA",
        'に' => "NI",
        'ぬ' => "NU",
        'ね' => "NE",
        'の' => "NO",
        'は' => "HA",
        'ば' => "BA",
        'ぱ' => "PA",
        'ひ' => "HI",
        'び' => "BI",
        'ぴ' => "PI",
        'ふ' => "HU",
        'ぶ' => "BU",
        'ぷ' => "PU",
        'へ' => "HE",
        'べ' => "BE",
        'ぺ' => "PE",
        'ほ' => "HO",
        'ぼ' => "BO",
        'ぽ' => "PO",
        'ま' => "MA",
        'み' => "MI",
        'む' => "MU",
        'め' => "ME",
        'も' => "MO",
        'ゃ' => "LYA",
        'や' => "YA",
        'ゅ' => "LYU",
        'ゆ' => "YU",
        'ょ' => "LYO",
        'よ' => "YO",
        'ら' => "RA",
        'り' => "RI",
        'る' => "RU",
        'れ' => "RE",
        'ろ' => "RO",
        'ゎ' => "LWA",
        'わ' => "WA",
        'を' => "WO",
        'ん' => "NN",
        _ => "XX",
    }
}

struct Roman {
    sent: String,
}

impl Roman {
    fn encryption_caesar(&self, key: u8) -> Roman {
        let mut tmp = String::new();
        for i in self.sent.chars() {
            if i == ' ' {
                tmp.push(' ');
            } else if i == '・' {
                tmp.push('・');
            } else if i == '＝' {
                tmp.push('＝');
            } else if i == 'ー' {
                tmp.push('ー');
            } else if i == '゛' {
                tmp.push('゛');
            } else {
                let num = match_alphabet_to_num(i);
                let encrypted_alphabet = match_num_to_alphabet((num + key) % 26);
                tmp.push(encrypted_alphabet);
            }
        }
        Roman { sent: tmp }
    }

    fn to_morse(&self) -> Morse {
        let mut tmp = String::new();
        for i in self.sent.chars() {
            tmp.push_str(match_alphabet_to_morse(i));
            tmp.push(' ');
        }
        Morse { sent: tmp }
    }

    fn to_kana(&self) -> Kana {
        let mut tmp = String::new();
        let mut check = String::new();
        for i in self.sent.chars() {
            if i == ' ' {
                tmp.push(' ');
            } else if i == '・' {
                tmp.push('・');
            } else if i == '＝' {
                tmp.push('＝');
            } else if i == 'ー' {
                tmp.push('ー');
            } else if i == '゛' {
                tmp.push('゛');
            } else {
                check.push(i);
                let aaa = match_roman_to_kana(&check);
                if aaa != '@' {
                    check = "".to_string();
                    tmp.push(aaa);
                }
            }
        }
        Kana { sent: tmp }
    }
}

fn match_alphabet_to_num(alphabet: char) -> u8 {
    match alphabet {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        'E' => 4,
        'F' => 5,
        'G' => 6,
        'H' => 7,
        'I' => 8,
        'J' => 9,
        'K' => 10,
        'L' => 11,
        'M' => 12,
        'N' => 13,
        'O' => 14,
        'P' => 15,
        'Q' => 16,
        'R' => 17,
        'S' => 18,
        'T' => 19,
        'U' => 20,
        'V' => 21,
        'W' => 22,
        'X' => 23,
        'Y' => 24,
        'Z' => 25,
        _ => 128,
    }
}

fn match_num_to_alphabet(num: u8) -> char {
    match num {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        4 => 'E',
        5 => 'F',
        6 => 'G',
        7 => 'H',
        8 => 'I',
        9 => 'J',
        10 => 'K',
        11 => 'L',
        12 => 'M',
        13 => 'N',
        14 => 'O',
        15 => 'P',
        16 => 'Q',
        17 => 'R',
        18 => 'S',
        19 => 'T',
        20 => 'U',
        21 => 'V',
        22 => 'W',
        23 => 'X',
        24 => 'Y',
        25 => 'Z',
        _ => '_',
    }
}

fn match_alphabet_to_morse<'life>(alphabet: char) -> &'life str {
    match alphabet {
        'A' => "・－",
        'B' => "－・・・",
        'C' => "－・－・",
        'D' => "－・・",
        'E' => "・",
        'F' => "・・－・",
        'G' => "－－・",
        'H' => "・・・・",
        'I' => "・・",
        'J' => "・－－－",
        'K' => "－・－",
        'L' => "・－・・",
        'M' => "－－",
        'N' => "－・",
        'O' => "－－－",
        'P' => "・－－・",
        'Q' => "－－・－",
        'R' => "・－・",
        'S' => "・・・",
        'T' => "－",
        'U' => "・・－",
        'V' => "・・・－",
        'W' => "・－－",
        'X' => "－・・－",
        'Y' => "－・－－",
        'Z' => "－－・・",
        ' ' => "・・－－・－",
        '・' => "－－－・・・",
        '＝' => "－・・・・－",
        'ー' => "・－－・－",
        '゛' => "・－・・－・",
        _ => "・・－－・・",
    }
}

fn match_roman_to_kana(roman: &str) -> char {
    match roman {
        "LA" => 'ぁ',
        "A" => 'あ',
        "LI" => 'ぃ',
        "I" => 'い',
        "LU" => 'ぅ',
        "U" => 'う',
        "LE" => 'ぇ',
        "E" => 'え',
        "LO" => 'ぉ',
        "O" => 'お',
        "KA" => 'か',
        "GA" => 'が',
        "KI" => 'き',
        "GI" => 'ぎ',
        "KU" => 'く',
        "GU" => 'ぐ',
        "KE" => 'け',
        "GE" => 'げ',
        "KO" => 'こ',
        "GO" => 'ご',
        "SA" => 'さ',
        "ZA" => 'ざ',
        "SI" => 'し',
        "ZI" => 'じ',
        "SU" => 'す',
        "ZU" => 'ず',
        "SE" => 'せ',
        "ZE" => 'ぜ',
        "SO" => 'そ',
        "ZO" => 'ぞ',
        "TA" => 'た',
        "DA" => 'だ',
        "TI" => 'ち',
        "DI" => 'ぢ',
        "LTU" => 'っ',
        "TU" => 'つ',
        "DU" => 'づ',
        "TE" => 'て',
        "DE" => 'で',
        "TO" => 'と',
        "DO" => 'ど',
        "NA" => 'な',
        "NI" => 'に',
        "NU" => 'ぬ',
        "NE" => 'ね',
        "NO" => 'の',
        "HA" => 'は',
        "BA" => 'ば',
        "PA" => 'ぱ',
        "HI" => 'ひ',
        "BI" => 'び',
        "PI" => 'ぴ',
        "HU" => 'ふ',
        "BU" => 'ぶ',
        "PU" => 'ぷ',
        "HE" => 'へ',
        "BE" => 'べ',
        "PE" => 'ぺ',
        "HO" => 'ほ',
        "BO" => 'ぼ',
        "PO" => 'ぽ',
        "MA" => 'ま',
        "MI" => 'み',
        "MU" => 'む',
        "ME" => 'め',
        "MO" => 'も',
        "LYA" => 'ゃ',
        "YA" => 'や',
        "LYU" => 'ゅ',
        "YU" => 'ゆ',
        "LYO" => 'ょ',
        "YO" => 'よ',
        "RA" => 'ら',
        "RI" => 'り',
        "RU" => 'る',
        "RE" => 'れ',
        "RO" => 'ろ',
        "LWA" => 'ゎ',
        "WA" => 'わ',
        "WO" => 'を',
        "NN" => 'ん',
        "XX" => '_',
        _ => '@',
    }
}

struct Morse {
    sent: String,
}

impl Morse {
    fn to_fgo(&self) -> Fgo {
        let mut tmp = String::new();
        for i in self.sent.chars() {
            tmp.push_str(match i {
                '・' => "F",
                '－' => "G",
                ' ' => "O",
                _ => "@",
            });
        }
        Fgo { sent: tmp }
    }

    fn to_roman(&self) -> Roman {
        let mut tmp = String::new();
        for i in self.sent.split_whitespace() {
            tmp.push(match_morse_to_alphabet(i));
        }
        Roman { sent: tmp }
    }
}
fn match_morse_to_alphabet(morse: &str) -> char {
    match morse {
        "・－" => 'A',
        "－・・・" => 'B',
        "－・－・" => 'C',
        "－・・" => 'D',
        "・" => 'E',
        "・・－・" => 'F',
        "－－・" => 'G',
        "・・・・" => 'H',
        "・・" => 'I',
        "・－－－" => 'J',
        "－・－" => 'K',
        "・－・・" => 'L',
        "－－" => 'M',
        "－・" => 'N',
        "－－－" => 'O',
        "・－－・" => 'P',
        "－－・－" => 'Q',
        "・－・" => 'R',
        "・・・" => 'S',
        "－" => 'T',
        "・・－" => 'U',
        "・・・－" => 'V',
        "・－－" => 'W',
        "－・・－" => 'X',
        "－・－－" => 'Y',
        "－－・・" => 'Z',
        "・・－－・－" => ' ',
        "－－－・・・" => '・',
        "－・・・・－" => '＝',
        "・－－・－" => 'ー',
        "・－・・－・" => '゛',
        _ => '@',
    }
}

struct Fgo {
    sent: String,
}

impl Fgo {
    fn to_morse(&self) -> Morse {
        let mut tmp = String::new();
        for i in self.sent.chars() {
            tmp.push_str(match i {
                'F' => "・",
                'G' => "－",
                'O' => " ",
                _ => "@",
            });
        }
        Morse { sent: tmp }
    }
}

#[cfg(test)]
mod tests {
    use super::Kana;
    use super::SHIFT_KEY;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_all() {
        let mut f = File::open("src/data/servant.txt").expect("file not found");
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        for content in contents.split_terminator(',') {
            let name = content.trim();
            let kana = Kana {
                sent: name.to_string(),
            };
            let roman = kana.to_roman();
            let caesar = roman.encryption_caesar(SHIFT_KEY);
            let morse = caesar.to_morse();

            let fgo = morse.to_fgo();

            let dec_morse = fgo.to_morse();
            let dec_caesar = dec_morse.to_roman();
            let dec_roman = dec_caesar.encryption_caesar(26 - SHIFT_KEY);
            let dec_kana = dec_roman.to_kana();

            assert_eq!(kana.sent, dec_kana.sent);
            assert_eq!(roman.sent, dec_roman.sent);
            assert_eq!(caesar.sent, dec_caesar.sent);
            assert_eq!(morse.sent, dec_morse.sent);
        }
    }
}
