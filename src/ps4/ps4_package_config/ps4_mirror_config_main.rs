/**************************************************************************/
/* ps4_mirror_config_main.rs                                               */
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

use std::fs::File;
use std::io::prelude::*;
use crate::ps4::database::ps4db::Source;
use crate::ps4::ps4_package_config::ps4config_init::{ConfigEntries, ConfigError, Config, RepoEntries, RepoNode};
use crate::ps4::ps4_package_progess_bar::get_root;
/// Returns a string of the requested config entry, optionally returns a config entry within a repo.
///
/// See [ConfigEntries] and [RepoEntries].
pub fn get_config_entry(entry: ConfigEntries, repo: Option<String>, repo_entry: Option<RepoEntries>) -> Result<String, ConfigError> {
    // Load config file
    let mut x = String::new();
    
    File::open(get_root() + "/etc/ps4.d/mirrorlist")
        .expect("Failed to open config file, is another process accessing it?")
        .read_to_string(&mut x)
        .expect("Failed to convert file to string");
    let config: Config = serde_json::from_str(&x).expect("Failed to serialize data");
    match entry {
        ConfigEntries::Disname => Ok(config.disname),
        ConfigEntries::Codename => Ok(config.codename),
        ConfigEntries::Version => Ok(config.version),
        ConfigEntries::Architecture => Ok(config.architecture),
        ConfigEntries::_Colour => Ok(config.colour.to_string()),
        ConfigEntries::_Progressbar => Ok(config.progressbar.to_string()),
        ConfigEntries::_Repos => {
            // Check if a repo and a repo config entry were supplied
            if repo.is_none() && repo_entry.is_none() {
                for i in config.repos {
                    // Find the requested repo
                    if repo.clone().unwrap() == i.name {
                        // Return the requested repo config entry
                        match repo_entry.unwrap() {
                            RepoEntries::_Name => return Ok(i.name),
                            RepoEntries::_Active => return Ok(i.active.to_string()),
                            RepoEntries::_Url => {
                                if i.url.is_some() {
                                    return Ok(i.url.unwrap());
                                }
                                // If a url is not present, return an empty string
                                return Ok(String::new());
                            },
                        }
                    }
                }
            }
            return Err(ConfigError)
        },
    }
}
/// Returns a Vec containing the entire repos array from config.
///
/// Currently only used for [get_sources].
fn get_repo_vec() -> Vec<RepoNode> {
        // Load config file
        let mut x = String::new();
    
        File::open(get_root() + "/etc/ps4.d/mirrorlist")
            .expect("Failed to open config file, is another process accessing it?")
            .read_to_string(&mut x)
            .expect("Failed to convert file to string");
    
        let config: Config = serde_json::from_str(&x).expect("Failed to serialize data");
        return config.repos
}
/// Return sources in config.

///
/// See [Source].
pub fn get_sources() -> Vec<Source> {
    let mut sources: Vec<Source> = vec![];
    let repo_config_entry: Vec<RepoNode> = get_repo_vec();
    for i in repo_config_entry {
        if i.active == true {
            sources.push(Source{
                name: i.name,
                url: if i.url.is_some() { Option::from(i.url.unwrap()) } else { None }
            })
        }
    }
    return sources;
}
