/**************************************************************************/
/* remove.rs                                                              */
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

use crate::ps4::{ps4_lock_package::{create_lock, lock_exists, remove_lock}, packaging::ps4_packageing_main::run_remove};
use crate::ps4::database::ps4dbmain::{get_depended_on, get_installed_package};
use crate::ps4::database::ps4db::InstalledPS4Packages;
use crate::ps4::ps4_package_progess_bar::{continue_prompt, display_removing_packages};

pub fn remove(args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Please provide a path to a package to remove. (Check ps4 --help for usage)");

        std::process::exit(1);
    }

    sudo::escalate_if_needed().expect("Failed to escalate to root.");
    lock_exists();
    create_lock().expect("Failed to create lock file. (Does /tmp/ps4.lock already exist?)");

    println!(" Getting  ps4 packages...");
    let raw_packages: Vec<String> = args.clone().drain(2..).collect();
    let mut packages: HashSet<InstalledPS4Packages> = HashSet::new();

    for i in raw_packages {
        let package = get_installed_package(&i);

        if package.is_ok() {
            packages.insert(package.unwrap());
        } else {
            println!("WARNING Package {} not found.", i);
        }
    }

    if packages.is_empty() {
        println!("ERROR No valid packages specified!");

        remove_lock().expect("Failed to remove lock file.");
        std::process::exit(1);
    }

    println!(" Checking dependencies...");
    let mut abort = false;
    let mut abort_map: HashMap<InstalledPS4Packages, Vec<InstalledPS4Packages>> = HashMap::new();
    for i in packages.clone() {
        let mut abort_vec: Vec<InstalledPS4Packages> = Vec::new();

        for x in get_depended_on(&i.name) {
            abort = true;
            abort_vec.push(x);
        }

        abort_map.insert(i, abort_vec);
    }

    if abort {
        println!("ERROR The following packages are depended on by other packages:");
        for (i, v) in abort_map.iter() {
            println!("{} {}-{} is required by:", i.name, i.version, i.upstream);
            for x in v {
                println!("\t{} {}-{}", x.name, x.version, x.upstream);
            }
        }

        println!("ERROR Please remove the above packages before continuing.");
        remove_lock().expect("Failed to remove lock file.");
        std::process::exit(1);
    }

    println!("\nPackages to remove [{}]: {}\n", packages.len(), display_removing_packages(packages.clone()));

    if !continue_prompt() {
        println!("\n==> Aborting!");

        remove_lock().expect("Failed to remove lock file.");
        std::process::exit(0);
    }

    println!("\n Removing packages...");

    for i in packages {
        println!("=> Removing {} {}-{}...", &i.name, &i.version, &i.upstream);
        run_remove(&i.name);
    }

    println!("\n Complete!");

    remove_lock().expect("Failed to remove lock");
}
