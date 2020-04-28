//! Tools to copy or get bytes

/// Copy bytes from `from` into `to`
/// The number of bytes actually copied equals min(from.len(), to.len())
pub fn copy_bytes(from: &[u8], to: &mut [u8]) {
    for (to_it, from_it) in to.iter_mut().zip(from) {
        *to_it = *from_it;
    }
}

/// Copy bytes from `from` into `to`, if `to.len() < from.len()`, then
/// fill the rest part with `fill`
/// The number of bytes actually copied equals min(from.len(), to.len())
pub fn copy_bytes_with_fill(from: &[u8], to: &mut [u8], fill: u8) {
    for (to_it, from_it) in to.iter_mut().zip(
        from.iter().chain([fill].iter().cycle())
    ) {
        *to_it = *from_it;
    }
}

/// Get `n` bytes from `from`
/// if `from.len() < n`, fill the rest part with `fill`
pub fn get_bytes_with_fill(from: &[u8], n: usize, fill: u8) -> Vec<u8> {
    let mut result = vec![];
    for i in 0..n {
        result.push(from.get(i).cloned().unwrap_or(fill))
    }
    result
}