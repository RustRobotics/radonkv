// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

#[must_use]
pub(super) const fn check_string_length(size: usize, append: usize) -> bool {
    // TODO(Shaohua): Limit string length to 512MB
    size.checked_add(append).is_some()
}

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_sign_loss)]
pub fn prune_range(len: usize, mut start: isize, mut end: isize) -> Option<(usize, usize)> {
    // TODO(Shaohua): Handle cast error
    let len_isize = len as isize;
    if start < 0 {
        start += len_isize;
    }
    if end < 0 {
        end += len_isize;
    };
    start = start.max(0);
    end = end.max(0);
    end = end.min(len_isize - 1);

    if start > end || len == 0 {
        return None;
    }

    let start_usize = start as usize;
    let end_usize = end as usize;
    Some((start_usize, end_usize))
}

#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_sign_loss)]
pub const fn prune_index(len: usize, mut index: isize) -> Option<usize> {
    let len_isize = len as isize;
    if index < 0 {
        index += len_isize;
    }
    if index >= len_isize || index < 0 || len == 0 {
        None
    } else {
        let index_usize = index as usize;
        Some(index_usize)
    }
}

#[cfg(test)]
mod tests {
    use super::{prune_index, prune_range};

    #[test]
    fn test_prune_range() {
        assert_eq!(prune_range(16, 0, 3), Some((0, 3)));
        assert_eq!(prune_range(16, -3, -1), Some((13, 15)));
        assert_eq!(prune_range(16, 0, -1), Some((0, 15)));
        assert_eq!(prune_range(16, 10, 100), Some((10, 15)));
    }

    #[test]
    fn test_prune_index() {
        assert_eq!(prune_index(2, 0), Some(0));
        assert_eq!(prune_index(2, -1), Some(1));
        assert_eq!(prune_index(2, -2), Some(0));
        assert_eq!(prune_index(2, 3), None);
        assert_eq!(prune_index(2, -3), None);
    }
}
