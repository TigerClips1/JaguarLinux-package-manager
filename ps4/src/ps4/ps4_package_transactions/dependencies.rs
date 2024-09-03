/**************************************************************************/
/* dependencies.rs                                                        */
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
use crate::ps4::database::ps4dbmain::{get_dependencies, get_installed_package, get_provides, get_remote_package, search_for_package};
use crate::ps4::ps4_lock_package::remove_lock;
use crate::ps4::ps4_package_progess_bar::string_to_vec;
use crate::ps4::packaging::ps4_packageing_setup::PS4Package;


/// Returns a list of all dependencies for a given package.
pub fn run_depend_resolve(package: PS4Package, dependencies: &mut HashSet<String>) {
    if package.depends.is_empty() {
        return;
    }

    for dep in string_to_vec(package.depends) {
        // Insert top level dependency into set
        dependencies.insert(dep.clone());

        let deeper_dep = get_dependencies(dep.clone());

        if deeper_dep.is_err() {
            eprintln!("FATAL ERROR Could not resolve dependency {} for {}", dep, package.name);

            remove_lock().expect("Could not remove lock file?");
            std::process::exit(1);
        }

        // Check database to get dependencies of dependency
        for depr_dep in deeper_dep.unwrap() {
            if dependencies.contains(&depr_dep.name) {
                // Circular dependency detected, let's not loop forever thanks
                continue;
            }

            // Insert depend we're looking into in case it doesn't have any dependencies
            dependencies.insert(depr_dep.name.clone());

            run_depend_resolve(depr_dep,  dependencies)
        }
    }
}

/// Check to see if provided dependencies are installed.
pub fn run_depend_check(packages: HashSet<String>) -> HashMap<String, bool> {
    let mut dependencies = HashMap::new();

    for package in packages.iter() {
        let remote_package_repo = search_for_package(&package);

        if remote_package_repo.is_ok() {
            let repo = remote_package_repo.unwrap();
            let remote_package = get_remote_package(&package, &repo);

            for provides_package in get_provides(&repo, &remote_package.unwrap().name)  {
                if get_installed_package(&provides_package.name).is_ok() {
                    dependencies.insert(package.clone(), true);
                    break;
                } else {
                    dependencies.insert(package.clone(), false);
                }
            }
        } else {
            if get_installed_package(&package).is_ok() {
                dependencies.insert(package.clone(), true);
            } else {
                dependencies.insert(package.clone(), false);
            }
        }
    }

    return dependencies;
}
