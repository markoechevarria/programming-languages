use crate::garden::vegetables::Asparagus;
use rand::Rng;
use std::{cmp::Ordering, io};

/*
* use std::io;
* use std::io::Write;
*/

use std::io::{self, Write};

use std::collections::*;

pub mod garden;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
}
