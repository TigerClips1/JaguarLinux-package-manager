/**************************************************************************/
/* groupinstall.rs                                                        */
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

use std::collections::HashSet;
use crate::ps4::ps4_package_manager_commands::install::install;
use crate::ps4::database::ps4dbmain::{get_group, search_for_group};
use crate::ps4::ps4_lock_package::{create_lock, lock_exists, remove_lock};

pub fn group_install(args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Please provide a group to install. (Check ps4 --help for usage)");

        std::process::exit(1);
    }

    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    lock_exists();

    create_lock().expect("Failed to create lock file. (Does /tmp/ps4.lock already exist?)");

    let requested_groups: Vec<String> = args.clone().drain(2..).collect();
    let mut install_queue: HashSet<String> = HashSet::new();

    for i in requested_groups {
        println!(" Looking for packages in {}", &i);

        let group_repo = search_for_group(&i);

        if group_repo.is_err() {
            eprintln!(" Group {} not found!", &i);

            remove_lock().expect("Failed to remove lock?");
            std::process::exit(1);
        }

        let requested_group = get_group(&group_repo.unwrap(), &i);

        for x in requested_group {
            install_queue.insert(x.name);
        }
    }

    // Append padding to updates so install will accept it
    let mut padding: Vec<String> = vec!["0".to_string(), "1".to_string()];
    padding.append(&mut install_queue.into_iter().collect());

    // Remove the lock as the install will takeover
    remove_lock().expect("Failed to remove lock?");

    install(padding);

    // remove_lock is done by install
}
