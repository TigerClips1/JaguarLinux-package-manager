/**************************************************************************/
/* ps4_lock_package.rs                                                    */
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

use std::{fs, fs::File};
use std::path::Path;
use crate::ps4::ps4_package_progess_bar::continue_prompt;

/// Creates a lock file indicating that bulge is open
pub fn create_lock() -> std::io::Result<()> {
    File::create("/tmp/ps4.lock")?;
    Ok(())
}

/// Deletes the lock file
pub fn remove_lock() -> std::io::Result<()>{
    fs::remove_file("/tmp/ps4.lock")?;
    Ok(())
}

/// Returns true if the lock file exists on the file system
pub fn check_lock() -> bool {
    Path::new("/tmp/ps4.lock").exists()
}

/// Check if a bulge instance is already running and give the option of removing the lock file
pub fn lock_exists() {
    if check_lock() {
        println!("An instance of ps4 is already running.");
        println!("Delete lock file? (Only do this when the other process is frozen)");
        if continue_prompt() {
            remove_lock().expect("Failed to delete lock file.");
        } else {
            std::process::exit(1);
        }
    }
}
