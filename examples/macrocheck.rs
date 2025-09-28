#![no_implicit_prelude]

// This purpose of this example is to test that my macro code is using the least ambiguous paths.

use ::seedpq::QueryResult;

#[derive(QueryResult, Debug)]
#[allow(dead_code)]
struct PostgresVersion {
    version: ::std::string::String,
}

fn main() {}
