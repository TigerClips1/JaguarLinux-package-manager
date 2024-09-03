/**************************************************************************/
/* help.rs                                                                */
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

pub fn help() {
    println!("ps4 - the JaguarLinux package  manager - v{}", crate::get_version());
    println!("usage: ps4 <command> [...]");
    println!("commands:");
    println!("\t ps4 {{-h --help}}");
    println!("\t\t - List all commands for ps4 (this view)");
    println!("\t ps4 {{-s --sync}}");
    println!("\t\t - Synchronizes package databases with remotes");
    println!("\t ps4 {{-u --upgrade}}");
    println!("\t\t - Check for (and then install) package updates");
    println!("\t ps4 {{-i --install}} <package(s)>");
    println!("\t ps4 {{-gi --groupinstall}} <groups(s)>");
    println!("\t\t - Install a specified package");
    println!("\t  ps4 {{-U --local}} <path(s)>");
    println!("\t\t - Install a package from a local archive");
    println!("\t  ps4 {{-remove --uninstall}} <package(s)>");
    println!("\t\t - Uninstall a specified package");
    //println!("\t ps4 info <package>"); TODO
    //println!("\t ps4 search <package>"); TODO
    println!("\t ps4 list");
    println!("\t\t - List all installed packages with their version and source");
}
