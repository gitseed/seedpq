// #![no_implicit_prelude]

use ::seedpq::QueryResult;

#[derive(QueryResult, Debug)]
#[allow(dead_code)]
struct PostgresVersion {
    version: ::std::string::String,
}

fn main() {}
