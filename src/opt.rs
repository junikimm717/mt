use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "mt", version = "0.2.0")]
pub struct Opt {
    #[structopt(short, long, help="Check the validity of the configuration file")]
    pub check: bool,
    #[structopt(short = "f", long, help="Write a default config file")]
    pub configure: bool,
    #[structopt(short, long, help="Edit the config file")]
    pub edit: bool,
    #[structopt(help="Alias (mt will auto-determine meetings if not given)")]
    pub alias: Option<String>,
}
