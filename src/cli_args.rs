use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone)]
pub(crate) struct Opt {
    /// Port to listen
    #[structopt(short, long, env = "PORT", default_value = "9092")]
    pub port: u16,

    /// Domain address such as localhost.
    #[structopt(long, env = "DOMAIN", default_value = "localhost")]
    pub domain: String,

    /// Database URL
    #[structopt(long, env = "DATABASE_URL")]
    pub database_url: String,
}
