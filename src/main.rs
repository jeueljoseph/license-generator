extern crate chrono;

use chrono::Datelike;
use chrono::Local;
use structopt::StructOpt;

use license_generator::create_license;
use license_generator::write_license;
use std::env;
use std::process;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(name = "INPUT")]
    input: Vec<String>,
    #[structopt(short = "a", long = "author")]
    author: String,
    #[structopt(short = "p", long = "project")]
    project: Option<String>,
    #[structopt(short = "y", long = "year")]
    year: Option<u32>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let dt = Local::now();
    let current_year = dt.year();
    let year = opt.year.unwrap_or_else(|| current_year as u32);
    let author = opt.author.as_str();
    // TODO: want to remove clone
    let project = opt.project.clone().unwrap_or_else(|| {
        env::current_dir()
            .expect("use --project: Failed to retrieve current directory")
            .file_name()
            .expect("use --project: Failed to retrieve current directory name")
            .to_os_string()
            .into_string()
            .expect("use --project: Failed to unwrap os_string")
    });

    for license_name in &opt.input {
        let license = create_license(&license_name);

        let license = license.unwrap_or_else(|| {
            eprintln!("License \"{}\" is not recognised.", &license_name);
            process::exit(1);
        });

        let file_name = if &opt.input.len() == &1 {
            "LICENSE".to_owned()
        } else {
            "LICENSE_".to_owned() + &license_name.to_uppercase()
        };

        let license_text = license.notice(year, &author, &project);
        write_license(&license_text, &file_name).unwrap_or_else(|error| {
            eprintln!("Error creating license file {}: {}", &file_name, error);
            process::exit(1);
        });
    }

    Ok(())
}
