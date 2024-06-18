// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

use std::fs;

use tasha::cmd::command_scheme::CommandSchemeMap;

fn main() {
    let json_content = fs::read_to_string("assets/commands/strlen.json");
    assert!(json_content.is_ok());
    let json_content = json_content.unwrap();
    let scheme_map: Result<CommandSchemeMap, _> = serde_json::from_str(&json_content);
    println!("scheme map: {scheme_map:?}");
    assert!(scheme_map.is_ok());
    let scheme_map = scheme_map.unwrap();
    let scheme = scheme_map.get("STRLEN");
    assert!(scheme.is_some());
    let scheme = scheme.unwrap();
    assert_eq!(scheme.function, "strlenCommand");
}
