/**************************************************************************/
/* ps4db.rs                                                                */
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

pub struct _PS4pkgstructer {
    pub name: String,
    pub version: String,
    pub upstream: i32,
    pub description: String,
    pub groups: Vec<String>,
    pub url: String,
    pub license: Vec<String>,
    pub depends: Vec<String>,
    pub optional_depends: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub replaces: Vec<String>,
    pub sha512sum: String
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct InstalledPS4Packages {
    pub name: String,
    pub groups: Vec<String>,
    pub source: String,
    pub version: String,
    pub upstream: i32,
    pub installed_files: Vec<String>,
    pub provides: Vec<String>,
    pub conflicts: Vec<String>,
    pub dependencies: Vec<String>
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Source {
    pub name: String,
    pub url: Option<String>
}