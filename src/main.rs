#[macro_use] extern crate rocket;
use std::{fs, io::{BufRead, BufReader}, sync::OnceLock};
use rand::{rngs::StdRng, Rng, SeedableRng};

const PADDING: [char; 11] = ['!', '@', '#', '$', '%', '^', '&', '*', '-', '=', '+'];
const PADDING_DEFAULT: &str = "wwdd";
const PADDING_MAX: usize= 8;

const WORD_LENGTH_DEFAULT: i8 = 5;
const WORD_LENGTH_MAX: i8 = 10;
const WORD_LENGTH_MIN: i8 = 3;

const WORD_COUNT_DEFAULT: i8 = 3;
const WORD_COUNT_MAX: i8 = 6;
const WORD_COUNT_MIN: i8 = 3;

const CAPITALIZATIONS: [&str; 4] = ["upper", "lower", "title", "random"];
const CAPITALIZATION_DEFAULT: &str = "random";
// Special thanks to SSL Sheep (@use_libre_software) for the suggestion of a OnceLock
static DICTIONARY: OnceLock<Vec<String>> = OnceLock::new();

#[get("/?<padding>&<wordlength>&<wordcount>&<capitalization>", rank = 1)]
fn pass_generator_normal(padding: Option<&str>, wordlength: Option<i8>, wordcount: Option<i8>, capitalization: Option<&str>) -> String {

    let (padding, wordlength, wordcount, capitalization): (&str, i8, i8, &str) = 
        query_validator(padding, wordlength, wordcount, capitalization);

    let passphrase: String = password_generator(padding, &wordlength, &wordcount, capitalization);

    return passphrase;
}

fn password_generator(pd: &str, wl: &i8, wc: &i8, cp: &str) -> String{
         
    let prefix: String = generate_prefix(pd).clone();
    let phrase: String = generate_phrase(wl, wc, cp);
    let suffix: String = prefix.chars().rev().collect::<String>();

    let passphrase: String = prefix + &phrase.clone() + &suffix.clone();
    
    return passphrase;
}

fn generate_prefix(pd: &str) -> String{
    let mut prefix: String = String::with_capacity(10);
    let mut prefix_stack: String = String::with_capacity(10);

    for i in pd.chars() {
        if prefix_stack.len() == 0 || prefix_stack.len() > 0 && prefix_stack.ends_with(i){
            prefix_stack.push(i);
        } else {
            if prefix_stack.ends_with('w'){
                prefix = prefix + &generate_prefix_char(&prefix_stack);

                prefix_stack.clear();
                prefix_stack.push(i);
            }else if prefix_stack.ends_with('d'){
                prefix = prefix + &generate_prefix_digit(&prefix_stack);

                prefix_stack.clear();
                prefix_stack.push(i);
            }
        }
    }

    if !prefix_stack.is_empty(){
        if pd.ends_with('w'){
            prefix = prefix + &generate_prefix_char(&prefix_stack);
        }else if pd.ends_with('d'){
            prefix = prefix + &generate_prefix_digit(&prefix_stack);
        }
    }

    prefix_stack.clear();
    return prefix;

}

fn generate_phrase(wl: &i8, wc: &i8, cp: &str) -> String{

    let padding_index: usize = StdRng::from_entropy().gen_range(0..PADDING.len().try_into().unwrap());
    let padding_char: char = PADDING[padding_index];

    let mut passphrase: String = padding_char.to_string();

    let mut word_list: Vec<String> = Vec::new();

    for word in DICTIONARY.get().unwrap(){
        if !word.starts_with('#'){
            if word.len() == usize::try_from(*wl).unwrap_or(5){
                let formatted_word: String = capitalizer(word, cp.to_string());
                word_list.push(formatted_word);
            }            
        }
    }

    let mut word_index: usize;

    for _ in 1..=*wc {
        word_index = StdRng::from_entropy().gen_range(0..word_list.len());

        passphrase = passphrase + &word_list[word_index];
        passphrase = passphrase + &padding_char.to_string();
    }

    return passphrase
    
}

fn generate_prefix_char(prefix_stack: &String) -> String {
    let prefix_index: usize = StdRng::from_entropy().gen_range(0..PADDING.len().try_into().unwrap());
    let prefix_char: char = PADDING[prefix_index];

    return prefix_char.to_string().repeat(prefix_stack.len());
}

fn generate_prefix_digit(prefix_stack: &String) -> String {
    let mut prefix: String = String::with_capacity(10);

    for _d in prefix_stack.chars(){
        prefix = prefix + &StdRng::from_entropy().gen_range(0..9).to_string();
    }

    return prefix;
}

fn capitalizer(word: &String, mut format: String) -> String{
    // Including Title case in this makes remembering way more complicated, leave it two Upper/Lower
    if format.to_ascii_lowercase() ==  "random"{
        match &StdRng::from_entropy().gen_range(0..=1){
            0 => format = "lower".to_string(),
            1 => format = "upper".to_string(),
            _ => format = "lower".to_string()
        }
    }

    let mut fomatted_word: String = word.clone();
    
    if fomatted_word.is_ascii() && !fomatted_word.is_empty(){
        match format.as_ref(){
            "lower" => fomatted_word.make_ascii_lowercase(),
            "upper" => fomatted_word.make_ascii_uppercase(),
            "title" => fomatted_word[0..1].make_ascii_uppercase(),
            _ => fomatted_word.make_ascii_lowercase()
        }
    }

    return fomatted_word;
}

fn query_validator<'a>(padding: Option<&'a str>, wordlength: Option<i8>, wordcount: Option<i8>, capitalization: Option<&'a str>) -> (&'a str, i8, i8, &'a str){
    let mut v_padding: &str = padding.unwrap_or_else(|| PADDING_DEFAULT.into());

    if !v_padding.is_ascii() || !v_padding.chars().all(|c|c == 'w' || c == 'd') {
        v_padding = PADDING_DEFAULT;
    }

    if v_padding.len() > PADDING_MAX {
        v_padding = &v_padding[0..PADDING_MAX];
    }

    // If this value gets too high you get a long password, but much more easily brute forced if you know the parameters. 10 is a good compromise.
    let v_wordlength: i8 = wordlength.unwrap_or(WORD_LENGTH_DEFAULT);
    let v_wordlength: i8 = if v_wordlength > WORD_LENGTH_MAX || v_wordlength < WORD_LENGTH_MIN { WORD_LENGTH_DEFAULT } else { v_wordlength };

    let v_wordcount: i8 = wordcount.unwrap_or(WORD_COUNT_DEFAULT);
    let v_wordcount: i8 = if v_wordcount > WORD_COUNT_MAX || v_wordcount < WORD_COUNT_MIN { WORD_COUNT_DEFAULT } else { v_wordcount };

    let v_capitalization: &str = capitalization.unwrap_or_else(|| CAPITALIZATION_DEFAULT.into());
    let v_capitalization: &str = if CAPITALIZATIONS.contains(&v_capitalization) { v_capitalization } else { CAPITALIZATION_DEFAULT };

    return (v_padding, v_wordlength, v_wordcount, v_capitalization);
}

#[get("/help")]
fn help() -> String {
    let result: String = ("XKPASS.IO(1)\n\n".to_owned() + 
                        "NAME\n\txkpass.io - xkcd-inspired passphrase via curl\n\n" +
                        "SYNOPSIS\n\tcurl https://xkpass.io [/? OPTION]\n\n") +
                        "\tDESCRIPTION\n\tWeb-based XKCD passphrase generator, based off the well-known 'correct horse battery staple' comic (#936).\n\n" +
                        "\tService is run stateless across TLS as a web-endpoint for generating passwords with automation.\n\n" +
                        "\tWithout additional parameters, the service will spit out a password consisting of the following:\n" +
                        "\t\t- Two special characters, two digits as a prefix\n" +
                        "\t\t- Three words, a minimum of five characters, separated by a random special character\n" +
                        "\t\t- Two digits, two special characters as a suffix\n\n" +
                        "\tOptional parameters are as follows:\n\n" +
                        "\t\tpadding=\tspecify prefix/suffix padding format, uses \"w\" to specify a special character spot and \"d\" to specify a digit spot\n\n\t" +
                        &format!("\twordlength=\tspecify word length, takes an integer between {} and {}\n\n\t", WORD_LENGTH_MIN, WORD_LENGTH_MAX) +
                        &format!("\twordcount\tspecify word count, takes an integer between {} and {}\n\n\t", WORD_COUNT_MIN, WORD_COUNT_MAX) +
                        &format!("\tcapitalization\tspecify capitalization, takes one of the following options: {}\n\n\t", CAPITALIZATIONS.join(" "))+
                        "Without specified parameters, the default query will look like: https://xkpass.io/?pd=wwdd&wl=5&wc=3&cp=random";

    return result;
}

#[launch]
fn rocket() -> _ {
    // Read dict_en.txt into memory on program load
    let buf = BufReader::new(fs::File::open("./share/dict_en.txt").expect("dict_en.txt not found!"));
    DICTIONARY.get_or_init(|| {
        buf.lines()
        .map(|l| l.expect("Unable to parse line in dict_en.txt!"))
        .collect()});

    rocket::build().mount("/", routes![pass_generator_normal,help])
}