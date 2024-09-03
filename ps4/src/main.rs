/**************************************************************************/
/* main.rs                                                                */
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

#![allow(clippy::all)]


use std::env;


mod ps4;

/// Get a static string of the current ps4 version
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    // Check if any command was supplied
    if args.len() < 2 {
        ps4::ps4_package_manager_commands::help::help();
        std::process::exit(0);
    }

    let command: String = args[1].to_lowercase();

    match &command[..] {
        // Help commands    
        "-h" => ps4::ps4_package_manager_commands::help::help(),
        "--help" => ps4::ps4_package_manager_commands::help::help(),

        // Sync commands
        "s" => ps4::ps4_package_manager_commands::sync::sync(),
        "--sync" => ps4::ps4_package_manager_commands::sync::sync(),

        // Upgrade commands
        "u" => ps4::ps4_package_manager_commands::upgrade::upgrade(),
        "--upgrade" => ps4::ps4_package_manager_commands::upgrade::upgrade(),


        // Install commands
        "i" => ps4::ps4_package_manager_commands::install::install(args),
        "--install" => ps4::ps4_package_manager_commands::install::install(args),
        "-U" => ps4::ps4_package_manager_commands::localinstall::local_install(args),
        "--local" => ps4::ps4_package_manager_commands::localinstall::local_install(args),
        "gi" => ps4::ps4_package_manager_commands::groupinstall::group_install(args),
        "--groupinstall" => ps4::ps4_package_manager_commands::groupinstall::group_install(args),

        // Remove commands
        "remove" => ps4::ps4_package_manager_commands::remove::remove(args),
        "--uninstall" => ps4::ps4_package_manager_commands::remove::remove(args),

        // Info commands

        // List commands
        "--list" => ps4::ps4_package_manager_commands::list::list(),

        // Specify that command is invalid and show help command
        _ => {
            println!("ps4: Invalid command \"{}\", use {{-h --help}} for valid commands.", command);
        }
    }
}

