/**************************************************************************/
/* ps4mirror.rs                                                           */
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
use crate::ps4::ps4_package_config::ps4_mirror_config_main::get_config_entry;
use crate::ps4::ps4_package_config::ps4config_init::ConfigEntries;
use crate::ps4::ps4_package_progess_bar::get_root;

/// Load mirrors for repos from mirror list
pub fn load_mirrors() -> Vec<String> {
    let mut mirrors: Vec<String> = vec![];

    let arch = get_config_entry(ConfigEntries::Architecture, None, None).expect("Failed to get config architecture.");
    let dis_name = get_config_entry(ConfigEntries::Disname, None, None).expect("Failed to get config dis_name.");
    let codename = get_config_entry(ConfigEntries::Codename, None, None).expect("Failed to get config codename.");
    let version = get_config_entry(ConfigEntries::Version, None, None).expect("Failed to get config version.");

    let mut raw_mirrors = String::new();

    File::open(get_root() + "/etc/ps4.d/mirrorlist")
        .expect("Failed to open mirror list, is another program using it?")
        .read_to_string(&mut raw_mirrors)
        .expect("Failed to convert file to string");

    for i in raw_mirrors.lines() {
        if !i.is_empty() && !i.starts_with("#") {
            mirrors.push(
                i.to_string()
                    .trim()
                    .replace("$arch", arch.trim_matches(|c| c == '\\' || c == '"'))
                    .replace("$dis_name", dis_name.trim_matches(|c| c == '\\' || c == '"'))
                    .replace("$codename", codename.trim_matches(|c| c == '\\' || c == '"'))
                    .replace("$version", version.trim_matches(|c| c == '\\' || c == '"'))
            );
        }
    }

    return mirrors;
}
