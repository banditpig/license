use clap::{arg_enum, App, Arg, SubCommand};

#[derive(Debug)]
pub struct Args {
    pub features_file: String,
    pub expires: String,
    pub owner_name: String,
    pub owner_email: String,
    pub public_key_file: String,
}
arg_enum! {
    #[derive(Debug)]
    enum Algorithm {
        SHA1,
        SHA256,
        Argon2
    }
}
fn main() {
    let matches = App::new("24daysofrust")
        // ...
        .subcommand(
            SubCommand::with_name("analyse")
                .about("Analyses the data from file")
                .arg(
                    Arg::with_name("input-file")
                        .short("i")
                        .default_value("default.csv")
                        .value_name("FILE"),
                ),
        )
        .subcommand(
            SubCommand::with_name("verify")
                .about("Verifies the data")
                .arg(
                    Arg::with_name("algorithm")
                        .short("a")
                        .help("Hash algorithm to use")
                        .possible_values(&Algorithm::variants())
                        .required(true)
                        .value_name("ALGORITHM"),
                ),
        )
        // ...
        .get_matches();

    println!("{:?}", matches);
}
