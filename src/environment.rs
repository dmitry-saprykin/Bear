// Copyright (c) 2017 László Nagy
//
// Licensed under the MIT license <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use std::env;
use serde_json;

const ENV_KEY: &'static str = "__BEAR";

#[derive(Serialize, Deserialize, Debug)]
pub struct Environment {
    pub cc: String,
    pub cxx: String,
}

pub fn read() -> Option<Environment> {
    let value = env::var(ENV_KEY).unwrap();
    let result: Environment = serde_json::from_str(value.as_str()).unwrap();
    Some(result)
}
