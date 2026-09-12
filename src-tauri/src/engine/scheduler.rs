use super::CHUNK_SIZE;

/// Splits a file into `(start, end_exclusive)` byte ranges of `CHUNK_SIZE`, the last
/// one truncated to whatever remains. Pure function so it's trivially testable and
/// reusable once resume (milestone 7) needs to recompute ranges from on-disk state.
pub fn plan_chunk_ranges(file_size: u64) -> Vec<(u64, u64)> {
    if file_size == 0 {
        return vec![(0, 0)];
    }
    let chunk_size = CHUNK_SIZE as u64;
    let mut ranges = Vec::with_capacity((file_size / chunk_size + 1) as usize);
    let mut start = 0;
    while start < file_size {
        let end = (start + chunk_size).min(file_size);
        ranges.push((start, end));
        start = end;
    }
    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_into_full_chunks() {
        let ranges = plan_chunk_ranges(CHUNK_SIZE as u64 * 2);
        assert_eq!(ranges, vec![(0, CHUNK_SIZE as u64), (CHUNK_SIZE as u64, CHUNK_SIZE as u64 * 2)]);
    }

    #[test]
    fn truncates_last_chunk() {
        let ranges = plan_chunk_ranges(CHUNK_SIZE as u64 + 100);
        assert_eq!(ranges, vec![(0, CHUNK_SIZE as u64), (CHUNK_SIZE as u64, CHUNK_SIZE as u64 + 100)]);
    }

    #[test]
    fn empty_file_is_one_zero_length_range() {
        assert_eq!(plan_chunk_ranges(0), vec![(0, 0)]);
    }
}
