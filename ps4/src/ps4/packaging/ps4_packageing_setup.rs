/**************************************************************************/
/* ps4_packageing_setup.rs                                                */
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

use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct PS4Package {
    pub name: String,
    pub version: String,
    pub upstream: i32,
    pub description: String,
    pub groups: String,
    pub url: String,
    pub license: String,
    pub depends: String,
    pub optional_depends: String,
    pub provides: String,
    pub conflicts: String,
    pub replaces: String,
    pub sha512sum: String
}

pub struct PS4NewPackage {
    pub name: String,
    pub groups: String,
    pub version: String,
    pub upstream: i32,
    pub installed_files: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub dependencies: Vec<String>
}

pub struct _PS4RequestPackage {
    pub name: String,
    pub version: String,
    pub upstream: i32
}
