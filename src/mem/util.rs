// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

#[must_use]
pub(super) fn check_string_length(size: usize, append: usize) -> bool {
    // TODO(Shaohua): Limit string length to 512MB
    size.checked_add(append).is_some()
}
