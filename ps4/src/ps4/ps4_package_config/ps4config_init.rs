/**************************************************************************/
/* ps4config_init.rs                                                      */
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

use std::fmt::{self};
use serde::Deserialize;

/// Custom error type for config related errors.
pub struct ConfigError;

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Requested config entry not found!")
    }
}

impl fmt::Debug for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

/// All possible config entries.
pub enum ConfigEntries {
    Disname,
    Codename,
    Version,
    Architecture,
    _Colour,
    _Progressbar,
    _Repos
}

/// All possible repo config entries.
pub enum RepoEntries {
    _Name,
    _Active,
    _Url
}

/// Struct form of Bulge's config file.
#[derive(Deserialize)]
pub(super) struct Config {
    pub(super) architecture: String,
    pub(super) version: String,
    pub(super) codename: String,
    pub(super) disname: String,
    pub(super) colour: bool,
    pub(super) progressbar: bool,
    pub(super) repos: Vec<RepoNode>
}

/// Struct form of repo config.
#[derive(Deserialize)]
pub(super) struct RepoNode {
    pub(super) name: String,
    pub(super) active: bool,
    pub(super) url: Option<String> 
}
