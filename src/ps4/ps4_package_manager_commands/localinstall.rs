/**************************************************************************/
/* localinstall.rs                                                        */
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


use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::Path;

use crate::ps4::database::ps4db::Source;
use crate::ps4::ps4_lock_package::{create_lock, lock_exists, remove_lock};
use crate::ps4::ps4_package_progess_bar::{continue_prompt, get_root};
use crate::ps4::packaging::ps4_packageing_main::{check_if_package, decode_pkg_file, decompress_gz};
use crate::ps4::ps4_package_transactions::install::{InstallTransaction, run_install};

pub fn local_install(args: Vec<String>) {
    if args.len() < 3 {
        eprintln!("Please provide a path to a package to install. (Check ps4 --help for usage)");

        std::process::exit(1);
    }

    sudo::escalate_if_needed().expect("Failed to escalate to root.");
    lock_exists();
    create_lock().expect("Failed to create lock file. (Does /tmp/ps4.lock already exist?)");

    let packages: Vec<String> = args.clone().drain(2..).collect();

    println!(" Resolving packages...");
    let mut package_queue: HashMap<InstallTransaction, File> = HashMap::new();
    for i in &packages {
        // Check if i is a valid path and assume it's a file we want to install if it is
        if Path::new(i).exists() {
            if !check_if_package(decompress_gz(fs::File::open(i).expect("Failed to read package!"))) {
                println!("Warning {} is not a valid package!", i);
            }

            let mut package_tar = decompress_gz(fs::File::open(i).expect("Failed to read package!"));
            package_tar.unpack(format!("{}/tmp/ps4/{}", get_root(), &i)).unwrap();

            let package = decode_pkg_file(fs::File::open(format!("{}/tmp/ps4/{}/PS4PKG", get_root(), &i))
                .expect("Failed to open PS4PKG file!"));

            package_queue.insert(InstallTransaction {
                package: package,
                source: Source{ name: "local".to_string(), url: None }
            }, fs::File::open(i).expect("Failed to read package!"));
        } else {
            println!("WARNING {} is not a valid package!", i);
        }
    }

    let mut temp_string = String::new();

    if package_queue.is_empty() {
        println!("ERROR_OwO No packages to install!");

        remove_lock().expect("Failed to remove lock file.");
        std::process::exit(1);
    }

    for (i, _f) in &package_queue {
        temp_string.push_str(&*i.package.name);
        temp_string.push_str("->");
        temp_string.push_str(&*i.package.version);
        temp_string.push_str("->");
        temp_string.push_str(&*i.package.upstream.to_string());
        temp_string.push_str(" ");
    }

    println!("\nPackages to install [{}]: {}\n", &package_queue.len(), temp_string);

    if !(continue_prompt()) {
        println!("Abandoning install!");

        remove_lock().expect("Failed to remove lock?");
        std::process::exit(1);
    }

    println!("\n Installing packages...");
    let mut clean_up_list: Vec<String> = Vec::new();
    for (i, f) in package_queue {
        println!(" Installing {} v{}-{}...", &i.package.name, &i.package.version, &i.package.upstream);

        run_install(i.clone(), f);

        clean_up_list.push(i.package.name.clone());
    }

    println!("\n Cleaning up...");

    for i in &packages {
        fs::remove_dir_all(format!("{}/tmp/ps4/{}", get_root(), &i))
            .expect("Failed to delete temp path!");
    }

    for i in clean_up_list {
        fs::remove_dir_all(format!("{}/tmp/ps4/{}", get_root(), &i))
            .expect("Failed to delete temp path!");
    }

    println!("\n Complete! :)");

    remove_lock().expect("Failed to remove lock?");
}
