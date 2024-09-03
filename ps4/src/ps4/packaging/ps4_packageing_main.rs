/**************************************************************************/
/* ps4_packageing_main.rs                                                  */
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

use std::fs::{self, File};
use std::path::Path;

use tar::Archive;
use flate2::read::GzDecoder;

use crate::ps4::{database::ps4dbmain::{remove_package_from_installed, return_owned_files}, packaging::ps4_packageing_setup::PS4Package};

pub fn decompress_gz(compressed_tar: File) -> Archive<GzDecoder<File>> {
    return Archive::new(GzDecoder::new(compressed_tar));
}

pub fn decode_pkg_file(pkg: File) -> PS4Package {
    let v: PS4Package = serde_json::from_reader(pkg).unwrap();

    return v;
}

pub fn check_if_package(mut gztar: Archive<GzDecoder<File>>) -> bool {    
    // Look for PKG file
    for file in gztar.entries().unwrap() {
        if file.unwrap().header().path().unwrap() == Path::new("PS4PKG") {
            // If a PKG file is found then this is a valid package
            return true;
        }                
    }

    return false;
}

pub fn run_remove(package: &String) {
    for x in return_owned_files(package).expect("Failed to get owned files!") {
        if Path::new(&x).exists() {
            fs::remove_file(x).expect("Failed to delete file!")
        }
    }

    remove_package_from_installed(package).expect("Failed to remove package from database.");
}
