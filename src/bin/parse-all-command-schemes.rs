// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::fs;

use tasha::cmd::command_scheme::{CommandScheme, CommandSchemeMap, ParseCommandSchemeError};

fn main() -> Result<(), ParseCommandSchemeError> {
    let dirname = "assets/commands/";
    let dir = fs::read_dir(dirname)?;
    let mut scheme_maps = CommandSchemeMap::new();

    for entry in dir {
        if let Ok(entry) = entry {
            let filename = entry.file_name();
            if let Ok(mut filename) = filename.into_string() {
                if filename.ends_with(".json") {
                    filename.insert_str(0, dirname);
                    let scheme_map = CommandScheme::parse(&filename)?;
                    println!("scheme map: {scheme_map:?}");
                    for (key, value) in scheme_map {
                        scheme_maps.insert(key, value);
                    }
                }
            }
        }
    }
    println!("scheme maps: {scheme_maps:#?}");
    Ok(())
}
