use getopts::Options;
use std::env;
use std::fs::File;
use std::io::BufReader;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("", "in", "input string", "INPUT");
    opts.optopt("", "model", "path of model file", "MODEL");
    opts.optflag("h", "help", "print this help menu");

    let matches = opts.parse(&args[1..]).unwrap_or_else(|f| {
        print_usage(&program, &opts);
        panic!("{}", f.to_string());
    });

    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
    }

    let input = matches.opt_str("in").unwrap_or_default();

    let words = match matches.opt_str("model") {
        Some(x) => {
            // Load model from json file and split sentences using the loaded model.
            let file = File::open(x).unwrap();
            let reader = BufReader::new(file);
            let model: budoux::Model = serde_json::from_reader(reader).unwrap();
            budoux::parse(&model, &input)
        }
        None => {
            // Split sentences with internal model.
            let model = budoux::models::default_japanese_model();
            budoux::parse(model, &input)
        }
    };

    for word in words {
        println!("{}", word);
    }
}
