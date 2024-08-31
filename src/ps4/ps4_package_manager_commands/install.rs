/**************************************************************************/
/* install.rs                                                             */
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

use std::fs;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

use isahc::http::StatusCode;
use isahc::ReadResponseExt;

use crate::ps4::database::ps4dbmain::{get_remote_package, search_for_package};
use crate::ps4::database::ps4db::Source;
use crate::ps4::ps4_lock_package::{create_lock, lock_exists, remove_lock};
use crate::ps4::ps4_package_progess_bar::{continue_prompt, display_installing_packages, get, get_root};
use crate::ps4::ps4mirror::load_mirrors;
use crate::ps4::packaging::ps4_packageing_main::run_remove;
use crate::ps4::packaging::ps4_packageing_setup::PS4Package;
use crate::ps4::ps4_package_transactions::conflict::run_conflict_package_check;
use crate::ps4::ps4_package_transactions::dependencies::{run_depend_check, run_depend_resolve};
use crate::ps4::ps4_package_transactions::install::{InstallTransaction, run_install};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Packages {
    name: String,
    repo: String
}

pub fn install(args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Please provide a package to install. (Check ps4 --help for usage)");

        std::process::exit(1);
    }

    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    lock_exists();

    create_lock().expect("Failed to create lock file. (Does /tmp/ps4.lock already exist?)");

    let requested_packages: Vec<String> = args.clone().drain(2..).collect();
    let mut packages: HashSet<Packages> = HashSet::new();

    println!(" Resolving packages and dependencies...");
    for i in &requested_packages {
        let repo = search_for_package(&i);

        if repo.is_err() {
            eprintln!("ERROR {} was not found! Aborting...", i);
    
            remove_lock().expect("Failed to remove lock?");
    
            std::process::exit(1);
        }

        let repo_unwrap = repo.unwrap();

        let remote_package = get_remote_package(&i, &repo_unwrap);

        if remote_package.is_err() {
            eprintln!("WARNING {} was not found!", &i);

            continue;
        }

        packages.insert(Packages {
            name: i.clone(),
            repo: repo_unwrap.clone()
        });

        let remote_package = remote_package.unwrap();

        if remote_package.depends.is_empty() {
            // Let's not check for depends as there is none
            continue;
        }

        let mut dependencies: HashSet<String> = HashSet::new();

        run_depend_resolve(
            get_remote_package(&i, &repo_unwrap).expect("Failed to get remote package."),
            &mut dependencies
        );

        let checked_deps = run_depend_check(dependencies);

        for x in checked_deps.iter() {
            if !x.1 {
                let repo = search_for_package(&x.0);

                if repo.is_err() {
                    eprintln!("ERROR {} was not found! Aborting...", i);

                    remove_lock().expect("Failed to remove lock?");

                    std::process::exit(1);
                }

                packages.insert(Packages {
                    name: x.0.clone(),
                    repo: repo.unwrap()
                });
            }
        }
    }

    if packages.is_empty() {
        eprintln!("ERROR No packages in queue! Aborting...");

        remove_lock().expect("Failed to remove lock?");

        std::process::exit(1);
    }

    println!(" Checking for already installed packages...");

    println!(" Looking for package conflicts...");
    let mut conflict = false;

    let mut possible_replace = false;
    let mut replace: HashMap<String, String> = HashMap::new();

    for i in packages.clone() {
        let conflict_pkg = run_conflict_package_check(&i.name);

        if conflict_pkg.is_conflict {
            conflict = true;
            println!("ERROR {} conflicts with:", &i.name);

            for x in conflict_pkg.packages {
                println!("\t{} {}-{}", x.name, x.version, x.upstream);

                if get_remote_package(&i.name, &i.repo).unwrap().provides.contains(&x.name) {
                    possible_replace = true;
                    replace.insert(i.name.clone(), x.name);
                }
            }
        }
    }

    if conflict && !possible_replace {
        println!("ERROR Package conflicts detected. Aborting...");

        remove_lock().expect("Failed to remove lock?");
        std::process::exit(1);
    }

    if possible_replace {
        for (i, x) in replace {
            println!("\n {} can be replaced with {}", x, i);

            if continue_prompt() {
                println!("=> Removing {}...", &x);
                run_remove(&x);
            } else {
                println!("errror Package conflicts detected. Aborting...");

                remove_lock().expect("Failed to remove lock?");
                std::process::exit(1);
            }
        }

        println!();
    }

    println!("  installing the  queue...");
    let mut queue: HashMap<PS4Package, String> = HashMap::new();
    for i in packages.clone() {
        queue.insert(
            get_remote_package(&i.name, &i.repo).expect("Failed to get remote package."),
            i.repo.clone()
        );
    }

    println!("\nPackages to install [{}]: {}\n", queue.len(), display_installing_packages(queue.clone()));

    if !(continue_prompt()) {
        println!("Abandoning install!");

        remove_lock().expect("Failed to remove lock?");
        std::process::exit(1);
    }

    println!("\n installing packages...");

    let mut filequeue: HashMap<InstallTransaction, File> = HashMap::new();

    for i in queue.clone() {
        println!(" installing {} v{}-{}...", &i.0.name, &i.0.version, &i.0.upstream);

        for x in load_mirrors() {
            let url = format!("{}/{}-{}-{}.tar.gz", x.replace("$repo", &*i.1),
                              &i.0.name, &i.0.version, &i.0.upstream);

            let mut downloaded_package = get(&url).expect("Failed to get package.");

            if downloaded_package.status() != StatusCode::OK  {
                println!("Failed to get {}. Status: {}", &url, downloaded_package.status());
                continue;
            }

            File::create(format!("{}/tmp/{}-{}-{}.tar.gz", get_root(),
                                 &i.0.name, &i.0.version, &i.0.upstream))
                .expect("Failed to create temporary file!")
                .write_all(downloaded_package.bytes().expect("Failed to get bytes.").as_slice())
                .expect("Failed to write to temporary file!");

            let file = File::open(format!("{}/tmp/{}-{}-{}.tar.gz", get_root(),
                                              &i.0.name, &i.0.version, &i.0.upstream))
                .expect("Failed to open temporary file!");

            filequeue.insert(InstallTransaction {
                package: i.0.clone(),
                source: Source { name: i.1, url: Some(url) }
            }, file);

            break;
        }
    }

    println!("\n Installing packages...");

    for i in filequeue {
        println!(" Installing {} v{}-{}...", &i.0.package.name, &i.0.package.version, &i.0.package.upstream);

        run_install(i.0, i.1);
    }

    println!("\n Cleaning up...");

    for i in queue {
        fs::remove_dir_all(format!("{}/tmp/ps4/{}", get_root(), &i.0.name))
            .expect("Failed to delete temp path!");

        fs::remove_file(format!("{}/tmp/{}-{}-{}.tar.gz", get_root(),
                                &i.0.name, &i.0.version, &i.0.upstream))
            .expect("Failed to remove temporary file!");
    }

    println!("\n Complete! :)");

    remove_lock().expect("Failed to remove lock?");
}