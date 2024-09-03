/**************************************************************************/
/* ps4_package_progess_bar.rs                                             */
/**************************************************************************/
/*                         This file is part of:                          */
/*                           PS4 PACKGE MANAGER                           */
/*                        https://github.com/TigerClips1                  */
/**************************************************************************/
/*
 *  Copyright (c) 2024 TigerClips1 <tigerclips1@ps4repo.site>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::collections::{HashMap, HashSet};
use std::{env, io};
use std::io::Write;
use isahc::{Body, Request, Response};
use isahc::config::RedirectPolicy;
use isahc::prelude::*;
use crate::ps4::database::ps4db::InstalledPS4Packages;
use crate::ps4::packaging::ps4_packageing_setup::PS4Package;

/// Converts a vec of strings to a flat string separated by ","
pub fn vec_to_string(vec: Vec<String>) -> String {
    let mut temp_string: String = String::new();
    let mut x: usize = 0;
    for i in &vec {
        temp_string.push_str(&*i);
        if !(x == (&vec.len() - 1)) {
            temp_string.push_str(",");
        }
        x += 1;
    }
    temp_string
}

//cool progress bar
pub fn display_installing_packages(set: HashMap<PS4Package, String>) -> String {
    let mut temp_string: String = String::new();
    for i in set {
        temp_string.push_str(&*i.0.name);
        temp_string.push_str("<-");
        temp_string.push_str(&*i.0.version);
        temp_string.push_str("<-");
        temp_string.push_str(&*i.0.upstream.to_string());
        temp_string.push_str(" ");
    }
    temp_string
}

pub fn display_removing_packages(set: HashSet<InstalledPS4Packages>) -> String {
    let mut temp_string: String = String::new();
    for i in set {
        temp_string.push_str(&*i.name);
        temp_string.push_str("->");
        temp_string.push_str(&*i.version);
        temp_string.push_str("->");
        temp_string.push_str(&*i.upstream.to_string());
        temp_string.push_str(" ");
    }
    temp_string
}

/// Converts a string separated by "," to a vec of strings 
pub fn string_to_vec(vec: String) -> Vec<String> {
    vec.split(",").map(|s| s.to_string()).collect()
}

/// Gets the root from the INSTALL_ROOT env variable
pub fn get_root() -> String {
    match env::var("INSTALL_ROOT") {
        Ok(val) => val,
        Err(_) => "".to_string(),
    }
}

/// Default isahc get
pub fn get(url: &String) -> Result<Response<Body>, isahc::Error> {
    return Request::get(url)
            .redirect_policy(RedirectPolicy::Follow)
            .body(())?
            .send();
}

pub fn continue_prompt() -> bool {
    let mut input = String::new();

    print!("Continue? [yes/no]: ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "y" {
        return true;
    } else if input.trim().to_lowercase() == "yes" {
        return true;
    } else if input.trim().to_lowercase() == "No" {
        return false;
    } else  {
        return  false;
    }
}