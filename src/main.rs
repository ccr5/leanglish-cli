mod leanglish_clap;

use leanglish_clap::leanglish_clap::get_leanglish_clap;

fn main() {
    let args = get_leanglish_clap().get_matches();
    match args.subcommand() {
        Some(("connection",  sub_m)) => { println!("{:?}", sub_m) }, // clone was used
        Some(("dict",   sub_m)) => { println!("{:?}", sub_m) }, // push was used
        _                       => { println!("Hello World") }, // Either no subcommand or one not tested 
    }
}
