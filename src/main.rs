use google_translator::{InputLang, OutputLang, translate, TranslateResult};
use rand::{thread_rng, Rng};
use console::style;
// use std::process::Command;
use std::env;
//mfw I have to update its length every time I update
static LANG: [(InputLang, OutputLang); 133] = [
    (InputLang::Galego,            OutputLang::Galego),
    (InputLang::Guarani,       OutputLang::Guarani),
    (InputLang::Gujarati,      OutputLang::Gujarati),
    (InputLang::Greek,         OutputLang::Greek),
    (InputLang::Dutch,         OutputLang::Dutch),
    (InputLang::Nepali,        OutputLang::Nepali),
    (InputLang::Norwegian,     OutputLang::Norwegian),
    (InputLang::Danish,        OutputLang::Danish),
    (InputLang::Dogri,         OutputLang::Dogri),
    (InputLang::German,        OutputLang::German),
    (InputLang::Dhivehi,       OutputLang::Dhivehi),
    (InputLang::Lao,           OutputLang::Lao),
    (InputLang::Latvian,       OutputLang::Latvian),
    (InputLang::Latin,         OutputLang::Latin),
    (InputLang::Russian,       OutputLang::Russian),
    (InputLang::Luganda,       OutputLang::Luganda),
    (InputLang::Romanian,      OutputLang::Romanian),
    (InputLang::Luxembourgish, OutputLang::Luxembourgish), 
    (InputLang::Lithuanian,    OutputLang::Lithuanian),
    (InputLang::Lingala,       OutputLang::Lingala),
    (InputLang::Marathi,       OutputLang::Marathi),
    (InputLang::Maori,         OutputLang::Maori),
    (InputLang::Maithili,      OutputLang::Maithili),
    (InputLang::Macedonian,    OutputLang::Macedonian),
    (InputLang::Malagasy,      OutputLang::Malagasy),
    (InputLang::Malayalam,     OutputLang::Malayalam),
    (InputLang::Malay,         OutputLang::Malay),
    (InputLang::Meithei,       OutputLang::Meithei),
    (InputLang::Malti,         OutputLang::Malti),
    (InputLang::Mongolian,     OutputLang::Mongolian),
    (InputLang::Hmong,         OutputLang::Hmong),
    (InputLang::Burmese,       OutputLang::Burmese),
    (InputLang::Mizo,          OutputLang::Mizo),
    (InputLang::Basque,        OutputLang::Basque),
    (InputLang::Bambara,       OutputLang::Bambara),
    (InputLang::Vietnamese,    OutputLang::Vietnamese),
    (InputLang::Belarusian,    OutputLang::Belarusian),
    (InputLang::Bengali,       OutputLang::Bengali),
    (InputLang::Bosnian,       OutputLang::Bosnian),
    (InputLang::Bhojpuri,      OutputLang::Bhojpuri),
    (InputLang::NSotho,        OutputLang::NSotho),
    (InputLang::Bulgarian,     OutputLang::Bulgarian),
    (InputLang::Samoan,        OutputLang::Samoan),
    (InputLang::Sanskrit,      OutputLang::Sanskrit),
    (InputLang::Serbian,       OutputLang::Serbian),
    (InputLang::Cebuano,       OutputLang::Cebuano),
    (InputLang::Sotho,         OutputLang::Sotho),
    (InputLang::Somali,        OutputLang::Somali),
    (InputLang::Shona,         OutputLang::Shona),
    (InputLang::Sundanese,     OutputLang::Sundanese),
    (InputLang::Swahili,       OutputLang::Swahili),
    (InputLang::Swedish,       OutputLang::Swedish),
    (InputLang::ScottishGaelic,OutputLang::ScottishGaelic),
    (InputLang::Spanish,       OutputLang::Spanish),
    (InputLang::Slovak,        OutputLang::Slovak),
    (InputLang::Slovene,       OutputLang::Slovene),
    (InputLang::Sindhi,        OutputLang::Sindhi),
    (InputLang::Sinhala,       OutputLang::Sinhala),
    (InputLang::Arabic,        OutputLang::Arabic),
    (InputLang::Armenian,      OutputLang::Armenian),
    (InputLang::Assamese,      OutputLang::Assamese),
    (InputLang::Aymara,        OutputLang::Aymara),
    (InputLang::Icelandic,     OutputLang::Icelandic),
    (InputLang::HaitianCreole, OutputLang::HaitianCreole),
    (InputLang::Irish,         OutputLang::Irish),
    (InputLang::Azerbaijani,   OutputLang::Azerbaijani),
    (InputLang::Afrikaans,     OutputLang::Afrikaans),
    (InputLang::Albanian,      OutputLang::Albanian),
    (InputLang::Amharic,       OutputLang::Amharic),
    (InputLang::Estonian,      OutputLang::Estonian),
    (InputLang::Esperanto,     OutputLang::Esperanto),
    (InputLang::Ewe,           OutputLang::Ewe),
    (InputLang::English,       OutputLang::English),
    (InputLang::Oromo,         OutputLang::Oromo),
    (InputLang::Odia,          OutputLang::Odia),
    (InputLang::Yoruba,        OutputLang::Yoruba),
    (InputLang::Urdu,          OutputLang::Urdu),
    (InputLang::Uzbek,         OutputLang::Uzbek),
    (InputLang::Ukrainian,     OutputLang::Ukrainian),
    (InputLang::Welsh,         OutputLang::Welsh),
    (InputLang::Uyghur,        OutputLang::Uyghur),
    (InputLang::Igbo,          OutputLang::Igbo),
    (InputLang::Yiddish,       OutputLang::Yiddish),
    (InputLang::Italian,       OutputLang::Italian),
    (InputLang::Indonesian,    OutputLang::Indonesian),
    (InputLang::Ilocano,       OutputLang::Ilocano),
    (InputLang::Japanese,      OutputLang::Japanese),
    (InputLang::Javanese,      OutputLang::Javanese),
    (InputLang::Georgian,      OutputLang::Georgian),
    (InputLang::Zulu,          OutputLang::Zulu),
    (InputLang::SimplifiedChinese, OutputLang::SimplifiedChinese),
    (InputLang::TraditionalChinese,OutputLang::TraditionalChinese) ,
    (InputLang::Chewa,          OutputLang::Chewa),
    (InputLang::Czech,          OutputLang::Czech),
    (InputLang::Tsonga,         OutputLang::Tsonga),
    (InputLang::Kazakh,         OutputLang::Kazakh),
    (InputLang::Catalan,        OutputLang::Catalan),
    (InputLang::Kannada,        OutputLang::Kannada),
    (InputLang::Quechuan,       OutputLang::Quechuan),
    (InputLang::Corsican,       OutputLang::Corsican),
    (InputLang::Xhosa,          OutputLang::Xhosa),
    (InputLang::Konkani,        OutputLang::Konkani),
    (InputLang::Sorani,         OutputLang::Sorani),
    (InputLang::Kurmanji,       OutputLang::Kurmanji),
    (InputLang::Croatian,       OutputLang::Croatian),
    (InputLang::Krio,           OutputLang::Krio),
    (InputLang::Khmer,          OutputLang::Khmer),
    (InputLang::Kinyarwanda,    OutputLang::Kinyarwanda),
    (InputLang::Kyrgyz,         OutputLang::Kyrgyz),
    (InputLang::Tamil,          OutputLang::Tamil),
    (InputLang::Tajik,          OutputLang::Tajik),
    (InputLang::Tatar,          OutputLang::Tatar),
    (InputLang::Thai,           OutputLang::Thai),
    (InputLang::Turkish,        OutputLang::Turkish),
    (InputLang::Telugu,         OutputLang::Telugu),
    (InputLang::Turkmen,        OutputLang::Turkmen),
    (InputLang::Akan,           OutputLang::Akan),
    (InputLang::Tigrinya,       OutputLang::Tigrinya),
    (InputLang::Pashto,         OutputLang::Pashto),
    (InputLang::Punjabi,        OutputLang::Punjabi),
    (InputLang::Persian,        OutputLang::Persian),
    (InputLang::Portuguese,     OutputLang::Portuguese),
    (InputLang::Polish,         OutputLang::Polish),
    (InputLang::French,         OutputLang::French),
    (InputLang::Frisian,        OutputLang::Frisian),
    (InputLang::Finnish,        OutputLang::Finnish),
    (InputLang::Filipino,       OutputLang::Filipino),
    (InputLang::Hawaiian,       OutputLang::Hawaiian),
    (InputLang::Hausa,          OutputLang::Hausa),
    (InputLang::Korean,         OutputLang::Korean),
    (InputLang::Hungarian,      OutputLang::Hungarian),
    (InputLang::Hebrew,            OutputLang::Hebrew),
    (InputLang::Hindi,             OutputLang::Hindi),

];
/// If any string has any of these punctuation characters then the API will trim
/// from the end of the string to where the first punctuation character occurs.
/// 
/// Any occurence of any one of these characters will the pattern for splitting
/// into a Vec<String>
const PUNCTUATION: [char; 5] = ['.','。',';','|', '!'];

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    // if args.is_empty() {
        
        // } else {
            
            // }
            
    // let term = Term::stdout();
    // term.clear_screen().expect("Uh oh");
    print!("{} \nby RadsammyT\n", 
        style("Translatify")
            .bold()
            .blue()
            .underlined()
    );

    
    let lang_count: u8;
    let mut input: String = String::new();
    if args.len() != 3 {
        println!("{}", style(
            format!("Invalid Argument Size. expected 2, got {}", args.len())
        )
        .red()
        .underlined()
        );

        println!("Args readout:");
        for i in args {
            println!("({i})");
        }

        panic!();
    } else {
        lang_count = args[1].as_str().parse().unwrap();
        input = args[2].to_owned();
    }


    println!("API Check... Please wait");
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

    in_vec =  doit(&in_vec, InputLang::Auto, LANG[lang_rand[0] as usize].1).await.0;
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
