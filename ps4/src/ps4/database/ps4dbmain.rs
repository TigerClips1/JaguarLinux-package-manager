/**************************************************************************/
/* ps4db.rs                                                               */
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
use rusqlite::{Connection, params};
use crate::ps4::{database::ps4db::Source, ps4_package_progess_bar::{string_to_vec, vec_to_string}, packaging::ps4_packageing_setup::{PS4NewPackage, PS4Package}};
use std::{time::{SystemTime, UNIX_EPOCH}, vec};
use crate::ps4::ps4_package_config::ps4_mirror_config_main::get_sources;
use std::{error::Error, fmt};
use crate::ps4::ps4_package_progess_bar::get_root;

use super::ps4db::InstalledPS4Packages;

#[derive(Debug)]
pub struct PackageDBError;

impl Error for PackageDBError {}

impl fmt::Display for PackageDBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error getting package from database!")
    }
}

/// Creates a database containing locally installed packages and various information
pub fn _init_database() {
    let conn = Connection::open(get_root() + "/etc/ps4/ps4.db").expect("Failed to create package database");

    conn.execute(
        "create table if not exists installed_packages
            (
                name text not null unique primary key,
                groups text,
                source text not null,
                version text not null,
                upstream integer not null,
                installed_files text,
                provides text,
                conflicts text,
                dependencies text
            )",
        [],
    ).expect("Failed to insert installed packages table");

    conn.execute(
        "create table if not exists repos
            (
                name text not null unique primary key,
                repo_hash text not null,
                last_updated text not null
            )",
        [],
    ).expect("Failed to insert repos table");

    add_package_to_installed(PS4NewPackage {
        name: "ps4".to_string(),
        groups: "core".to_string(),
        version: crate::get_version().to_string(),
        upstream: 0,
        installed_files: vec![],
        provides: vec!["ps4".to_string()],
        conflicts: vec![],
        dependencies: vec!["curl".to_string(), "sqlite".to_string()],
    }, Source{
        name: "core".to_string(),
        url: None
    });
}

/// Adds a package to the installed packages database
pub fn add_package_to_installed(package: PS4NewPackage, source: Source) {
    let conn = Connection::open(get_root() + "/etc/ps4/ps4.db").expect("Failed to create package database");

    // Convert installed files into a string
    let installed_files: String = vec_to_string(package.installed_files);

    // Convert source into a string
    let package_source: String;
    if source.url.is_none() {
        package_source = source.name
    } else {
        package_source = format!("{},{}", source.name, source.url.unwrap());
    }

    conn.execute("
        INSERT OR REPLACE INTO installed_packages (name, groups, source, version, upstream, installed_files, provides, conflicts, dependencies)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);",
        params![package.name,
        package.groups,
        package_source,
        package.version,
        package.upstream,
        installed_files,
        vec_to_string(package.provides),
        vec_to_string(package.conflicts),
        vec_to_string(package.dependencies)]
    ).expect("Failed to insert package into database!");
}

/// Returns files owned by a package
pub fn return_owned_files(package: &String) -> Result<Vec<String>, rusqlite::Error> {
    let conn = Connection::open(get_root() + "/etc/ps4/ps4.db")?;
    let mut files: Vec<String> = vec![];

    let mut statement = conn.prepare("SELECT * FROM installed_packages WHERE name = ?")?;

    let result = statement.query_map([package], | package | {
        return Ok(InstalledPS4Packages{
            name: package.get(0).unwrap(),
            groups: string_to_vec(package.get::<usize, String>(1).unwrap()),
            source: package.get(2).unwrap(),
            version: package.get(3).unwrap(),
            upstream: package.get(4).unwrap(),
            installed_files: package.get::<usize, String>(5).unwrap().split(",").map(|s| s.to_string()).collect(),
            provides: string_to_vec(package.get::<usize, String>(6).unwrap()),
            conflicts: string_to_vec(package.get::<usize, String>(7).unwrap()),
            dependencies: string_to_vec(package.get::<usize, String>(8).unwrap())
        });
    })?;

    for pkg in result {
        files = pkg?.installed_files.clone();
    }

    Ok(files)
}

/// Removes a package from the installed packages database
pub fn remove_package_from_installed(package: &String) -> Result<(), rusqlite::Error>{
    let conn = Connection::open(get_root() + "/etc/ps4/ps4.db")?;

    conn.execute("DELETE FROM installed_packages WHERE name = ?1",
    params![package])?;

    Ok(())
}

/// Look for a package in a repo and return the repo it is present in
pub fn search_for_package(package: &String) -> Result<String, PackageDBError> {
    let mut repo = String::new();

    for i in get_sources() {
        let conn = Connection::open(format!("{}/etc/ps4/{}.db", get_root(), i.name)).expect("Failed to create package database");

        // Fail silently and skip, this happens when the repo is empty
        if conn.prepare("SELECT * FROM packages WHERE name = ?").is_err() {
            println!("WARN> Repo {} is empty", i.name);
            continue;
        }

        let mut statement = conn.prepare("SELECT * FROM packages WHERE name = ?").expect("Failed to prepare statement");
        let mut rows = statement.query([package]).expect("Failed to query database");

        while let Some(_) = rows.next().expect("Failed to get next row") {
            repo = i.name.clone();
        }

        if !repo.is_empty() {
            return Ok(repo)
        }
    }

    return Ok(repo)
}

pub fn update_cached_repos(repo: &String, repo_hash: &String) {
    let conn = Connection::open(get_root() + "/etc/ps4/ps4.db").expect("Failed to create package database");

    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards?")
        .as_millis()
        .to_string();

    conn.execute("
        INSERT OR REPLACE INTO repos (name, repo_hash, last_updated)
        VALUES (?1, ?2, ?3);",
                 params![repo,
                 repo_hash,
                 current_time]
    ).expect("Failed to insert repo into database!");
}

pub fn get_installed_package(package: &String) -> Result<InstalledPS4Packages, PackageDBError> {
    let conn = Connection::open(get_root() + "/etc/ps4/ps4.db").expect("Failed to open database");

    let mut statement = conn.prepare("SELECT * FROM installed_packages WHERE name = ?").expect("Failed to prepare statement");

    let result = statement.query_map([package], | package | {
        return Ok(InstalledPS4Packages{
            name: package.get(0).unwrap(),
            groups: string_to_vec(package.get::<usize, String>(1).unwrap()),
            source: package.get(2).unwrap(),
            version: package.get(3).unwrap(),
            upstream: package.get(4).unwrap(),
            installed_files: package.get::<usize, String>(5).unwrap().split(",").map(|s| s.to_string()).collect(),
            provides: string_to_vec(package.get::<usize, String>(6).unwrap()),
            conflicts: string_to_vec(package.get::<usize, String>(7).unwrap()),
            dependencies: string_to_vec(package.get::<usize, String>(8).unwrap())
        });
    }).expect("DB Error!");

    for pkg in result {
        return Ok(pkg.unwrap());
    }

    return Err(PackageDBError);
}

pub fn get_remote_package(package: &String, repo: &String) -> Result<PS4Package, PackageDBError> {
    let conn = Connection::open(format!("{}/etc/ps4/{}.db", get_root(), repo)).expect("Failed to open database");

    let statement = conn.prepare("SELECT * FROM packages WHERE name = ?");

    if statement.is_err() {
        return Err(PackageDBError);
    }

    let mut unwrap_statement = statement.unwrap();

    let result = unwrap_statement.query_map([package], | package | {
        return Ok(PS4Package{
            name: package.get(0).unwrap(),
            version: package.get(1).unwrap(),
            upstream: package.get(2).unwrap(),
            description: package.get(3).unwrap(),
            groups: package.get(4).unwrap(),
            url: package.get(5).unwrap(),
            license: package.get(6).unwrap(),
            depends: package.get(7).unwrap(),
            optional_depends: package.get(8).unwrap(),
            provides: package.get(9).unwrap(),
            conflicts: package.get(10).unwrap(),
            replaces: package.get(11).unwrap(),
            sha512sum: package.get(12).unwrap()
        });
    }).expect("DB Error!");

    for pkg in result {
        return Ok(pkg.unwrap());
    }

    return Err(PackageDBError);
}


/// Get top-level dependencies for a package
pub fn get_dependencies(package_name: String) -> Result<Vec<PS4Package>, PackageDBError> {
    let mut dependencies: Vec<PS4Package> = Vec::new();

    let pkg_repo = search_for_package(&package_name)?;

    let pkg = get_remote_package(&package_name, &pkg_repo)?;

    if pkg.depends.is_empty() {
        return Ok(dependencies);
    }

    for dep in pkg.depends.split(",") {
        let dep_pkg = get_remote_package(&dep.to_string(), &pkg_repo)?;
        dependencies.push(dep_pkg);
    }

    return Ok(dependencies);
}

pub fn get_all_installed() -> Vec<InstalledPS4Packages> {
    let conn = Connection::open(format!("{}/etc/ps4/ps4.db", get_root())).expect("Failed to open package database");

    let mut statement = conn.prepare("SELECT * FROM installed_packages").expect("Failed to create statement");

    let result = statement.query_map([], | package | {
        return Ok(InstalledPS4Packages{
            name: package.get(0).unwrap(),
            groups: string_to_vec(package.get::<usize, String>(1).unwrap()),
            source: package.get(2).unwrap(),
            version: package.get(3).unwrap(),
            upstream: package.get(4).unwrap(),
            installed_files: package.get::<usize, String>(5).unwrap().split(",").map(|s| s.to_string()).collect(),
            provides: string_to_vec(package.get::<usize, String>(6).unwrap()),
            conflicts: string_to_vec(package.get::<usize, String>(7).unwrap()),
            dependencies: string_to_vec(package.get::<usize, String>(8).unwrap())
        });
    }).expect("Failed to execute query");

    return result.map(|r| r.unwrap()).collect();
}

/// Look for a group in a repo and return the repo it is present in
pub fn search_for_group(group: &String) -> Result<String, PackageDBError> {
    let mut repo = String::new();

    for i in get_sources() {
        let conn = Connection::open(format!("{}/etc/ps4/{}.db", get_root(), i.name)).expect("Failed to create package database");

        // Fail silently and skip, this happens when the repo is empty
        if conn.prepare("SELECT * FROM packages WHERE instr(groups, ?) > 0;").is_err() {
            println!("WARN> Repo {} is empty", i.name);
            continue;
        }

        let mut statement = conn.prepare("SELECT * FROM packages WHERE instr(groups, ?) > 0;").expect("Failed to prepare statement");
        let mut rows = statement.query([group]).expect("Failed to query database");

        while let Some(_) = rows.next().expect("Failed to get next row") {
            repo = i.name.clone();
        }

        if !repo.is_empty() {
            return Ok(repo)
        }
    }

    return Ok(repo)
}

/// Get all packages in a requested group
pub fn get_group(repo: &String, group: &String) -> Vec<PS4Package> {
    let conn = Connection::open(format!("{}/etc/ps4/{}.db", get_root(), repo)).expect("Failed to open package database");

    let mut statement = conn.prepare("SELECT * FROM packages WHERE instr(groups, ?) > 0;").expect("Failed to create statement");

    let result = statement.query_map([group], | package | {
        return Ok(PS4Package {
            name: package.get(0).unwrap(),
            version: package.get(1).unwrap(),
            upstream: package.get(2).unwrap(),
            description: package.get(3).unwrap(),
            groups: package.get(4).unwrap(),
            url: package.get(5).unwrap(),
            license: package.get(6).unwrap(),
            depends: package.get(7).unwrap(),
            optional_depends: package.get(8).unwrap(),
            provides: package.get(9).unwrap(),
            conflicts: package.get(10).unwrap(),
            replaces: package.get(11).unwrap(),
            sha512sum: package.get(12).unwrap()
        });
    }).expect("Failed to execute query");

    return result.map(|r| r.unwrap()).collect();
}

pub fn get_provides(repo: &String, package: &String) -> Vec<PS4Package> {
    let conn = Connection::open(format!("{}/etc/ps4/{}.db", get_root(), repo)).expect("Failed to open package database");

    let mut statement = conn.prepare("SELECT * FROM packages WHERE instr(provides, ?) > 0;").expect("Failed to create statement");

    let result = statement.query_map([package], | package | {
        return Ok(PS4Package{
            name: package.get(0).unwrap(),
            version: package.get(1).unwrap(),
            upstream: package.get(2).unwrap(),
            description: package.get(3).unwrap(),
            groups: package.get(4).unwrap(),
            url: package.get(5).unwrap(),
            license: package.get(6).unwrap(),
            depends: package.get(7).unwrap(),
            optional_depends: package.get(8).unwrap(),
            provides: package.get(9).unwrap(),
            conflicts: package.get(10).unwrap(),
            replaces: package.get(11).unwrap(),
            sha512sum: package.get(12).unwrap()
        });
    }).expect("Failed to execute query");

    return result.map(|r| r.unwrap()).collect();
}

pub fn get_conflicts(package: &String) -> Vec<InstalledPS4Packages> {
    let conn = Connection::open(format!("{}/etc/ps4/ps4.db", get_root())).expect("Failed to open package database");

    let mut statement = conn.prepare("SELECT * FROM installed_packages WHERE instr(conflicts, ?) > 0;").expect("Failed to create statement");

    let result = statement.query_map([package], | package | {
        return Ok(InstalledPS4Packages{
            name: package.get(0).unwrap(),
            groups: string_to_vec(package.get::<usize, String>(1).unwrap()),
            source: package.get(2).unwrap(),
            version: package.get(3).unwrap(),
            upstream: package.get(4).unwrap(),
            installed_files: package.get::<usize, String>(5).unwrap().split(",").map(|s| s.to_string()).collect(),
            provides: string_to_vec(package.get::<usize, String>(6).unwrap()),
            conflicts: string_to_vec(package.get::<usize, String>(7).unwrap()),
            dependencies: string_to_vec(package.get::<usize, String>(8).unwrap()),
        });
    }).expect("Failed to execute query");

    return result.map(|r| r.unwrap()).collect();
}

pub fn get_depended_on(package: &String) -> Vec<InstalledPS4Packages> {
    let conn = Connection::open(format!("{}/etc/ps4/ps4.db", get_root())).expect("Failed to open package database");

    let mut statement = conn.prepare("SELECT * FROM installed_packages WHERE instr(dependencies, ?) > 0;").expect("Failed to create statement");

    let result = statement.query_map([package], | package | {
        return Ok(InstalledPS4Packages{
            name: package.get(0).unwrap(),
            groups: string_to_vec(package.get::<usize, String>(1).unwrap()),
            source: package.get(2).unwrap(),
            version: package.get(3).unwrap(),
            upstream: package.get(4).unwrap(),
            installed_files: package.get::<usize, String>(5).unwrap().split(",").map(|s| s.to_string()).collect(),
            provides: string_to_vec(package.get::<usize, String>(6).unwrap()),
            conflicts: string_to_vec(package.get::<usize, String>(7).unwrap()),
            dependencies: string_to_vec(package.get::<usize, String>(8).unwrap()),
        });
    }).expect("Failed to execute query");

    return result.map(|r| r.unwrap()).collect();
}
