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

use std::fs::File;
use std::io::{Read, Write};

use data_encoding::HEXLOWER;
use ring::digest::{Context, SHA512};
use rusqlite::{Connection, params};
use serde_derive::Deserialize;
use walkdir::{DirEntry, WalkDir};
#[allow(dead_code)]
#[derive(Debug)]
#[derive(Deserialize)]
 struct Config {
     name: String,
}
#[allow(dead_code)]
#[derive(Deserialize)]

 struct PackageInfo {
     name: String,
     version: String,
     upstream: i32,
     description: String,
     groups: Vec<String>,
     url: String,
     license: Vec<String>,
     depends: Vec<String>,
     optional_depends: Vec<String>,
     make_depends: Vec<String>,
     provides: Vec<String>,
     conflicts: Vec<String>,
     replaces: Vec<String>,
     maintainers: Vec<String>
}

/// Converts a vec of strings to a flat string separated by ","
pub fn vec_to_string(vec: &Vec<String>) -> String {
    let mut temp_string: String = String::new();
    let mut x: usize = 0;
    for i in vec {
        temp_string.push_str(&*i);
        if !(x == (vec.len() - 1)) {
            temp_string.push_str(",");
        }
        x += 1;
    }
    temp_string
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn main() {
    let current_unix_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards?")
        .as_millis();

    /* UNUSED, but will be in the future
    if !Path::new("REPOINFO").exists() {
        println!("This is not a valid repository");
        exit(1);
    }

    // Get and load the config file
    let config_file_contents = std::fs::read_to_string("REPOINFO").unwrap();
    let config: Config = toml::from_str(&config_file_contents).unwrap();

    println!("=== Generating {} ===", &config.name);
    */

    println!(" Generating {} at {} ", "PS4 Database", current_unix_time);

    // Create the initial sqlite database with rust-sqlite
    println!(" Creating Database ");
    let mut db = Connection::open("ps4.db").expect("Failed to create database! Aborting...");

    db.execute(
        "create table if not exists packages
            (
                name             text       not null
                                            primary key
                                            unique,
                version          text       not null,
                upstream         integer    not null,
                description      text       not null,
                groups           text,
                url              text,
                license          text,
                depends          text,
                optional_depends text,
                provides         text,
                conflicts        text,
                replaces         text,
                checksum         text
            );
        ",
        [],
    ).expect("Failed to insert table into database! Aborting...");
    println!(" Database Created!");

    println!(" Populating Database ");
    let tx = db.transaction().expect("Failed to creation a transaction!");

    let mut flat_db: String = String::new();
    for entry in WalkDir::new("packages/").into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok()) {
        if entry.file_name().to_str().unwrap().ends_with("SRCINFO") {
            // Load the .SRCINFO file as JSON and put it into PackageInfo
            let package_info_file_contents = std::fs::read_to_string(entry.path()).unwrap();
            let package_info: PackageInfo = serde_json::from_str(&package_info_file_contents).unwrap();

            println!(" Inserting: {} v{}-{}", &package_info.name, &package_info.version, &package_info.upstream);

            tx.execute("
                INSERT OR REPLACE INTO packages
                    (
                        name,
                        version,
                        upstream,
                        description,
                        groups,
                        url,
                        license,
                        depends,
                        optional_depends,
                        provides,
                        conflicts,
                        replaces,
                        checksum
                    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13);
            ",
            params![
                &package_info.name,
                &package_info.version,
                &package_info.upstream,
                &package_info.description,
                vec_to_string(&package_info.groups),
                &package_info.url,
                vec_to_string(&package_info.license),
                vec_to_string(&package_info.depends),
                vec_to_string(&package_info.optional_depends),
                vec_to_string(&package_info.provides),
                vec_to_string(&package_info.conflicts),
                vec_to_string(&package_info.replaces),
                ""
            ]
            ).expect("Failed to insert package into database! Aborting...");

            flat_db.push_str(&format!("{}{}", &package_info.name, "\n"));
        }
    }

    tx.commit().expect("Failed to commit transaction!");
    
    File::create("ps4.flat").expect("Failed to create flat db")
        .write_all(flat_db.as_ref()).expect("Failed to write flat db");

    println!(" Database Populated!");

    println!(" Generating Hash");
    let mut database_file = File::open("ps4.db").expect("Failed to open ps4.db! Aborting...");

    let mut context = Context::new(&SHA512);
    let mut buffer = [0; 1024];

    loop {
        let read = database_file.read(&mut buffer).expect("Failed to read ps4.db! Aborting...");
        if read == 0 {
            break;
        }
        context.update(&buffer[..read]);
    }

    let hash = context.finish();
    let hash_string = HEXLOWER.encode(hash.as_ref()).to_lowercase();

    println!("=> Hash: {}", &hash_string);

    let mut database_hash_file = File::create("ps4.hash").expect("Failed to create ps4.hash! Aborting...");
    database_hash_file.write_all(hash_string.as_ref()).expect("Failed to write to ps4.hash! Aborting...");
    println!(" Hash File Created!");
    println!(" Hash Generated! ");

    println!(" Finished!")
}


