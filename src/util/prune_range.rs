// Copyright (c) 2024 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU Affero General Public License
// that can be found in the LICENSE file.

#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
#[allow(clippy::cast_sign_loss)]
pub fn prune_range(len: usize, mut start: i64, mut end: i64) -> Option<(usize, usize)> {
    // TODO(Shaohua): Handle cast error
    let len_i64 = len as i64;
    if start < 0 {
        start += len_i64;
    }
    if end < 0 {
        end += len_i64;
    };
    start = start.max(0);
    end = end.max(0);
    end = end.min(len_i64 - 1);

    if start > end || len == 0 {
        return None;
    }

    let start_usize = start as usize;
    let end_usize = end as usize;
    Some((start_usize, end_usize))
}

#[cfg(test)]
mod tests {
    use super::prune_range;

    #[test]
    fn test_prune_range() {
        assert_eq!(prune_range(16, 0, 3), Some((0, 3)));
        assert_eq!(prune_range(16, -3, -1), Some((13, 15)));
        assert_eq!(prune_range(16, 0, -1), Some((0, 15)));
        assert_eq!(prune_range(16, 10, 100), Some((10, 15)));
    }
}