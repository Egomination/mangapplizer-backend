use super::{
    ConnectionManager,
    Pool,
    PoolError,
};

fn init_pool(database_url: &str) -> Result<Pool, PoolError> {}

pub(crate) fn establish_connection(opt: crate::cli_args::Opt) -> Pool {
    init_pool(&opt.database_url)
        .except("Failed to create database connection pool!")
}
