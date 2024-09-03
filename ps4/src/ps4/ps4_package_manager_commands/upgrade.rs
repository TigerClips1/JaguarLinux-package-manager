/**************************************************************************/
/* upgrade.rs                                                             */
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

use version_compare::Version;
use crate::ps4::ps4_package_manager_commands::install::install;
use crate::ps4::database::ps4dbmain::{get_all_installed, get_remote_package};
use crate::ps4::ps4_lock_package::{create_lock, lock_exists, remove_lock};

pub fn upgrade() {
    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    // Ensure databases are synced
    crate::ps4::ps4_package_manager_commands::sync::sync();

    lock_exists();

    create_lock().expect("Failed to create lock file. (Does /tmp/ps4.lock already exist?)");

    println!(" Checking for updates...");

    let installed_packages = get_all_installed();
    let mut updates: Vec<String> = Vec::new();

    for i in installed_packages {
        let source = i.clone().source;
        let source_name = source.split(",").collect::<Vec<&str>>()[0];

        if source_name == "local" {
            continue;
        }

        let remote_package = get_remote_package(&i.name, &source_name.to_string());

        if remote_package.is_err() {
            continue;
        }

        let remote_package = remote_package.unwrap();

        // Always force upgrade if the epoch is higher
        if &remote_package.upstream > &i.upstream {
            updates.push(i.name.clone());
            continue;
        }

        if Version::from(&*remote_package.version) > Version::from(&*i.version) {
            updates.push(i.name.clone());
        }
    }

    match updates.len() {
        0 => {
            println!(" No updates found.");

            remove_lock().expect("Failed to remove lock file.");
            std::process::exit(0);
        },
        1 => {
            println!(" Updating {} package...", updates.len());
        },
        _ => {
            println!(" Updating {} packages...", updates.len());
        }
    }

    // Append padding to updates so install will accept it
    let mut padding: Vec<String> = vec!["0".to_string(), "1".to_string()];
    padding.append(&mut updates);

    // Remove the lock as the install will takeover
    remove_lock().expect("Failed to remove lock?");

    install(padding);

    // remove_lock is done by install
}
