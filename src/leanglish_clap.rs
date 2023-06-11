pub mod leanglish_clap {

    use clap::{
        crate_version, 
        crate_description,
        crate_name,crate_authors,
        Command,
        Arg
    };

    pub fn get_leanglish_clap() -> Command {
        Command::new(crate_name!())
            .author(crate_authors!())
            .version(crate_version!())
            .about(crate_description!())
            .subcommand(
                Command::new("connection")
                    .arg(Arg::new("new"))
                    .arg(Arg::new("is_connected"))
            )
            .subcommand(
                Command::new("dict")
                    .arg(Arg::new("list"))
                    .arg(Arg::new("today"))

            )
            .arg(Arg::new("learn"))
    }
}