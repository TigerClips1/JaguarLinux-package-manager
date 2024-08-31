/**************************************************************************/
/* conflict.rs                                                            */
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

use std::path::Path;
use crate::ps4::database::ps4dbmain::get_conflicts;
use crate::ps4::database::ps4db::InstalledPS4Packages;

pub struct ConflictingFiles {
    pub is_conflict: bool,
    pub files: Vec<String>,
}

pub struct ConflictingPackages {
    pub is_conflict: bool,
    pub packages: Vec<InstalledPS4Packages>,
}

pub fn run_conflict_check(files: &Vec<String>, is_installed: bool, root: String) -> ConflictingFiles {
    let mut conflicting_struct = ConflictingFiles {
        is_conflict: false,
        files: vec![]
    };

    for i in files {
        if !is_installed && Path::new(format!("{}{}", &root, &i).as_str()).exists() {
            conflicting_struct.is_conflict = true;
            conflicting_struct.files.push(format!("{}{}", root.clone(), i.clone()));
        }
    }

    return conflicting_struct;
}

pub fn run_conflict_package_check(package: &String) -> ConflictingPackages {
    let mut conflicting_struct = ConflictingPackages {
        is_conflict: false,
        packages: vec![]
    };

    for i in get_conflicts(package) {
        if &i.name == package {
            // Whoops we found a conflict with ourselves, lets skip this one
            continue;
        }

        conflicting_struct.is_conflict = true;
        conflicting_struct.packages.push(i);
    }

    return conflicting_struct;
}
