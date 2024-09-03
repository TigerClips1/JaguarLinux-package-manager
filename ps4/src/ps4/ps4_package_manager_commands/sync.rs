/**************************************************************************/
/* sync.rs                                                             */
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
use std::io::{copy, Read};

use isahc::http::StatusCode;
use ring::digest::{Context, SHA512};

use hex::ToHex;

use crate::ps4::ps4_package_config::ps4_mirror_config_main::get_sources;
use crate::ps4::database::ps4dbmain::update_cached_repos;
use crate::ps4::ps4_lock_package::{create_lock, lock_exists, remove_lock};
use crate::ps4::ps4_package_progess_bar::{get, get_root};
use crate::ps4::ps4mirror::load_mirrors;

use isahc::prelude::*;
use isahc::{Body, Response};

pub fn sync() {
    sudo::escalate_if_needed().expect("Failed to escalate to root.");

    lock_exists();

    create_lock().expect("Failed to create lock file. (Does /tmp/ps4.lock already exist?)");

    println!(" Synchronizing Repo Databases :)");

    for i in get_sources() {
        println!(" Updating :) {}", i.name);

        let mirror_list = load_mirrors();

        for x in mirror_list {
            let url: String;

            if i.url.is_some() {
                url = format!("{}/ps4.db", i.url.clone().expect("Failed to extract custom repo url"));
            } else {
                url = format!("{}/ps4.db", x.replace("$repo", &*i.name));
            }

            let db_response = get(&url);

            if db_response.is_err() {
                println!("Failed to get {}. Error: {}", &url, db_response.err().unwrap());
                continue;
            }

            let mut db_response_unwrap: Response<Body> = db_response.expect("Response errored while bypassing the check");

            if db_response_unwrap.status() != StatusCode::OK  {
                println!("Failed to get {}. Status: {}", &url, db_response_unwrap.status());
                continue;
            }

            let content_bytes = db_response_unwrap.bytes().expect("Failed to get bytes from response");
            let mut content = content_bytes.as_slice();
            let mut content_save = content;

            let hash_url: String;

            if i.url.is_some() {
                hash_url = format!("{}/ps4.hash", i.url.clone().expect("Failed to extract custom repo url"));
            } else {
                hash_url = format!("{}/ps4.hash", x.replace("$repo", &*i.name));
            }

            let hash_response = get(&hash_url);

            if hash_response.is_err() {
                println!("Failed to get {}. Error: {}", &hash_url, hash_response.err().unwrap());
                continue;
            }

            let mut hash_response_unwrap: Response<Body> = hash_response.expect("Response errored while bypassing the check");

            if hash_response_unwrap.status() != StatusCode::OK  {
                println!("Failed to get {}. Status: {}", &hash_url, hash_response_unwrap.status());
                continue;
            }

            let hash = hash_response_unwrap.bytes().expect("Failed to read database bytes");
            let hash_string = String::from_utf8(hash.clone()).expect("Failed to convert hash to string");

            let mut context = Context::new(&SHA512);
            let mut buffer = [0; 1024];

            loop {
                let read = content.read(&mut buffer).expect("Failed to read database.db! Aborting... :(");
                if read == 0 {
                    break;
                }
                context.update(&buffer[..read]);
            }

            let generated_hash = context.finish();

            if generated_hash.as_ref().encode_hex::<String>() != hash_string {
                println!("!!!> Verification failed for {}, trying next mirror. <!!!", hash_url);
                continue;
            }

            let mut dest = File::create(format!("{}/etc/ps4/{}.db", get_root(), i.name)).expect("Failed to create database file!");
            copy(&mut content_save, &mut dest).expect("Failed to copy downloaded content");

            update_cached_repos(&i.name, &hash_string);

            break;
        }
    }

    println!(" Synchronization Complete UwU");

    remove_lock().expect("Failed to remove lock?");
}
