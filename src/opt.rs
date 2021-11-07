use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="mt", version="0.2.0")]
pub struct Opt {
    #[structopt(short, long)]
    pub check: bool,
    #[structopt(short="f", long)]
    pub configure: bool,
    #[structopt(short, long)]
    pub edit: bool,
    pub alias: Option<String>
}
