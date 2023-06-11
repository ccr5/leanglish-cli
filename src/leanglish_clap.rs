use { 
    clap::{
        crate_version, 
        crate_description,
        crate_name,crate_authors,
        Command,
        Arg
    }
};

fn get_leanglish_clap() -> Command {
    let commands = Command::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .subcommand(
            Command::new("new")
                .arg(Arg::new("user"))
                .arg(Arg::new("dict"))   
        )
        .subcommand(
            Command::new("connection")
                .arg(Arg::new("new"))
                .arg(Arg::new)
        )

        .arg(Arg::new("new"))
        .arg(Arg::new("connection"))
        .arg(Arg::new("learn"));

    commands
        
}