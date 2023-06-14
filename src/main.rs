pub mod langs;

use google_translator::{InputLang, OutputLang, translate, TranslateResult};
use rand::{thread_rng, Rng};
use console::style;
// use std::process::Command;
use std::env;
use langs::LANG;
// use core::num::ParseIntError;
//mfw I have to update its length every time I update

/// If any string has any of these punctuation characters then the API (Not Eveheeero's fault, blame googles undocumented API) will trim
/// from the end of the string to where the first punctuation character occurs.
/// 
/// Any occurence of any one of these characters will be the pattern for splitting
/// into a Vec<String>
/// 
/// Google translate will take ANY punctuation mark of ANY language and have it cut off there.
/// Additional work may be required to add all known punctuation marks into this array
/// 
const PUNCTUATION: [char; 7] = ['.','。',';','|', '!', '?' ,'।'];

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    print!("{} \nby RadsammyT\n", 
        style("Translatify")
            .bold()
            .blue()
            .underlined()
    );

    
    let lang_count: u8;
    #[allow(unused_assignments)]
    let mut input: String = String::new();
    if args.len() != 3 {      
        if args.len() != 1 {    
            match args[1].as_str() {

                "help" => {
                    println!("Arguments: \n(number of languages) \"(input string)\"");
                }

                "test" => {
                    doit(&vec!["Test message".to_owned()], InputLang::Auto, OutputLang::Japanese).await;
                }

                _ => {
                    println!("{}", style(
                        format!("Invalid Argument Size. expected 3, got {}", args.len())
                    )
                    .red()
                    .underlined()
                    );
                
                    println!("Args readout:");
                    for i in args {
                        println!("({i})");
                    }
                }
            }
        } else {
            println!("Arguments: \n(number of languages) \"(input string)\"");
        }

        std::process::exit(0);
    } else {
        lang_count = args[1].as_str().parse().unwrap_or_else(|e| {
            println!("Invalid number. {e}");
            panic!("{e}");
        });
        input = args[2].to_owned();
    }

    let mut lang_rand: Vec<u16> = vec![];
    for _ in 0..lang_count {
        lang_rand.push(thread_rng().gen_range(0..LANG.len()) as u16);
    }

    // because windows puts a \r on entering the string.
    if cfg!(windows) {
        input = input.trim_end_matches("\r").to_string();
    }
    
    // let temp_1 = input.to_string().split('.');
    let temp: Vec<&str> = input.split(&PUNCTUATION[..]).collect();
    let mut in_vec: Vec<String> = vec![];
    for i in temp {
        in_vec.push(i.to_string());
    }
    
    dbg!(&in_vec);
    // dbg!(&lang_rand);

    in_vec = doit(&in_vec, InputLang::Auto, LANG[lang_rand[0] as usize].1).await.0;
    println!("ITER 0 (AUTO -> {:?}): {:?}", LANG[lang_rand[0] as usize].1, in_vec);
    for i in 0..lang_rand.len()-1 {
        let mut temp = String::new();
        for i in in_vec.to_owned() {
            temp.push_str(i.as_str());
        }
        let split = temp.split(&PUNCTUATION[..]).collect::<Vec<&str>>();
        in_vec.clear();
        for i in split {
            in_vec.push(i.to_owned());
        }

        //trans(input, langs[lang_rand[i]].0, LANG[lang_rand[i+1]].1)
        let res = doit(&in_vec, LANG[lang_rand[i] as usize].0, LANG[lang_rand[i+1] as usize].1).await;
        in_vec =  res.0;
        println!("ITER {} ({:?} -> {:?}): {:?}", i+1, LANG[lang_rand[i] as usize].0, LANG[lang_rand[i+1] as usize].1, in_vec);
        // in_vec = temp;
    }
    in_vec = doit(&in_vec, InputLang::Auto, OutputLang::English).await.0;
    println!("Final: {:?}", in_vec);
}

// if I put all the contents of this function into main() then translation wouldnt work
// because its still a future for whatever reason.
// putting the translate function in a separate async function works though
async fn doit(input: &Vec<String>, inlang: InputLang, outlang: OutputLang) -> (Vec<String>, TranslateResult) {
    let res = translate
    (
        input.to_vec(), 
        inlang, 
        outlang
    )
        .await
        .unwrap();
    // dbg!(&res.output_text);
    
    let mut ret: Vec<String> = vec![];
    for i in 0..res.output_text.len() {
        ret.push(res.output_text[i][0].to_owned());
    }
    return (ret.to_owned(), res);
}
