//! # Euclidean Rhythm
//!
//! A Rust library for generating Euclidean rhythms using Bjorklund's algorithm.
//!
//! Euclidean rhythms distribute pulses as evenly as possible across a given number
//! of steps, creating musically interesting patterns found in traditional music
//! from cultures worldwide.
//!
//! ## Quick Start
//!
//! ```
//! use euclidean_rhythm::euclidean;
//!
//! // Generate a Cuban tresillo pattern: [x . . x . . x .]
//! let pattern = euclidean(8, 3, 0);
//!
//! // Generate and rotate a West African bell pattern
//! let pattern = euclidean(8, 5, 2);
//! ```
//!
//! ## Algorithm Background
//!
//! The algorithm was developed by E. Bjorklund in 2003 for timing neutron beams
//! at particle accelerators. In 2005, Godfried Toussaint discovered its musical
//! applications, showing these patterns appear in traditional music worldwide.
//!
//! ## Common Patterns
//!
//! - **E(3,8)**: Cuban tresillo - `[x . . x . . x .]`
//! - **E(5,8)**: West African bell - `[x . x x . x x .]`
//! - **E(5,12)**: Persian rhythm - `[x . . x . x . . x . x .]`
//! - **E(7,16)**: Brazilian bossa nova - `[x . . x . x . x . . x . x . x .]`
//!
//! ## References
//!
//! - Toussaint, G. (2005). "The Euclidean Algorithm Generates Traditional Musical Rhythms"
//! - Bjorklund, E. (2003). "The Theory of Rep-Rate Pattern Generation in the SNS Timing System"

/// Generates a Euclidean rhythm pattern using Bjorklund's algorithm.
///
/// Distributes `pulses` as evenly as possible across `steps`, optionally
/// rotated by `rotation` positions (circular, wraps with modulo).
///
/// # Arguments
/// * `steps` - Total number of steps in the pattern (1-64 recommended)
/// * `pulses` - Number of pulses to distribute (0 to steps)
/// * `rotation` - Number of positions to rotate the pattern (circular)
///
/// # Returns
/// A vector of booleans where `true` represents a pulse and `false` represents a rest.
///
/// # Panics
/// Panics if `pulses > steps` or if `steps == 0`.
///
/// # Examples
/// ```
/// use euclidean_rhythm::euclidean;
/// let pattern = euclidean(8, 3, 0);
/// assert_eq!(pattern, vec![true, false, false, true, false, false, true, false]);
/// ```
#[must_use = "euclidean rhythm pattern should be used"]
pub fn euclidean(steps: u8, pulses: u8, rotation: u8) -> Vec<bool> {
    if steps == 0 {
        panic!("steps == 0");
    }
    if pulses > steps {
        panic!("pulses > steps");
    }
    if pulses == 0 {
        return vec![false; steps as usize];
    }
    if pulses == steps {
        return vec![true; steps as usize];
    }
    let mut pattern = bjorklund(steps, pulses);

    // Apply rotation
    let rot = (rotation % steps) as usize;
    if rot > 0 {
        pattern.rotate_left(rot);
    }

    pattern
}

/// Converts a boolean pattern to a string representation.
///
/// # Arguments
/// * `pattern` - The pattern to convert
/// * `pulse_char` - Character to represent pulses (true values)
/// * `rest_char` - Character to represent rests (false values)
///
/// # Examples
/// ```
/// use euclidean_rhythm::{euclidean, pattern_to_string};
/// let pattern = euclidean(8, 3, 0);
/// assert_eq!(pattern_to_string(&pattern, 'x', '.'), "x..x..x.");
/// ```
pub fn pattern_to_string(pattern: &[bool], pulse_char: char, rest_char: char) -> String {
    pattern
        .iter()
        .map(|&b| if b { pulse_char } else { rest_char })
        .collect()
}

/// Rotates a pattern by a given number of steps.
///
/// Positive rotation values rotate left (earlier in time), negative values rotate
/// right (later in time). The rotation wraps around the pattern length.
///
/// # Arguments
/// * `pattern` - The pattern to rotate
/// * `rotation` - Number of positions to rotate (positive=left, negative=right)
///
/// # Examples
/// ```
/// use euclidean_rhythm::rotate_pattern;
///
/// let pattern = vec![true, false, true, false];
/// let rotated = rotate_pattern(&pattern, 1);
/// assert_eq!(rotated, vec![false, true, false, true]);
///
/// // Negative rotation (rotate right)
/// let rotated = rotate_pattern(&pattern, -1);
/// assert_eq!(rotated, vec![false, true, false, true]);
/// ```
pub fn rotate_pattern(pattern: &[bool], rotation: i32) -> Vec<bool> {
    if pattern.is_empty() {
        return Vec::new();
    }

    let len = pattern.len() as i32;
    // Handle negative rotation and normalize to 0..len
    let normalized_rotation = ((rotation % len) + len) % len;

    let mut result = Vec::with_capacity(pattern.len());
    result.extend_from_slice(&pattern[normalized_rotation as usize..]);
    result.extend_from_slice(&pattern[..normalized_rotation as usize]);
    result
}

/// Core Bjorklund algorithm implementation.
///
/// Distributes pulses evenly by repeatedly pairing and concatenating groups
/// until a single pattern emerges. Uses Vec<Vec<bool>> to represent groups
/// during algorithm execution, which is necessary for Bjorklund's pairing
/// and concatenation logic. The final flattening step produces the output Vec<bool>.
///
/// Assumes inputs are already validated by the caller.
#[inline]
fn bjorklund(steps: u8, pulses: u8) -> Vec<bool> {
    let steps = steps as usize;
    let pulses = pulses as usize;

    if pulses == 1 {
        let mut pattern = vec![false; steps];
        pattern[0] = true;
        return pattern;
    }

    if pulses == 0 || pulses == steps {
        // Trivial cases handled by caller
        return Vec::new();
    }

    // Initialize with pulses as 1s and rests as 0s
    let mut pattern: Vec<Vec<bool>> = Vec::new();

    // Start with pulses number of [true] groups
    for _ in 0..pulses {
        pattern.push(vec![true]);
    }

    // And (steps - pulses) number of [false] groups
    for _ in 0..(steps - pulses) {
        pattern.push(vec![false]);
    }

    // Apply Bjorklund's algorithm: repeatedly distribute groups
    let mut split_idx = pulses;

    loop {
        let num_left = split_idx;
        let num_right = pattern.len() - split_idx;

        if num_right <= 1 {
            break;
        }

        let num_pairs = num_left.min(num_right);

        // Append right groups to left groups
        for i in 0..num_pairs {
            let right_pattern = pattern[split_idx + i].clone();
            pattern[i].extend(right_pattern);
        }

        // Remove the paired right groups
        pattern.drain(split_idx..split_idx + num_pairs);

        // Update split index
        split_idx = num_pairs;
    }

    // Flatten the groups into a single pattern
    pattern.into_iter().flatten().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tresillo() {
        let pattern = euclidean(8, 3, 0);
        assert_eq!(
            pattern,
            vec![true, false, false, true, false, false, true, false]
        );
    }

    #[test]
    fn west_african_bell() {
        let pattern = euclidean(8, 5, 0);
        assert_eq!(
            pattern,
            vec![true, false, true, true, false, true, true, false]
        );
    }

    #[test]
    fn persian() {
        let pattern = euclidean(12, 5, 0);
        assert_eq!(
            pattern,
            vec![
                true, false, false, true, false, true, false, false, true, false, true, false
            ]
        );
    }

    #[test]
    fn bossa_nova() {
        let pattern = euclidean(16, 7, 0);
        assert_eq!(
            pattern,
            vec![
                true, false, false, true, false, true, false, true, false, false, true, false,
                true, false, true, false
            ]
        );
    }

    #[test]
    fn all_pulses() {
        let pattern = euclidean(8, 8, 0);
        assert_eq!(pattern, vec![true; 8]);
    }

    #[test]
    fn all_rests() {
        let pattern = euclidean(8, 0, 0);
        assert_eq!(pattern, vec![false; 8]);
    }

    #[test]
    fn single_pulse() {
        let pattern = euclidean(8, 1, 0);
        assert_eq!(
            pattern,
            vec![true, false, false, false, false, false, false, false]
        );
    }

    #[test]
    #[should_panic]
    fn pulses_gt_steps() {
        let _ = euclidean(8, 9, 0);
    }

    #[test]
    #[should_panic]
    fn steps_zero() {
        let _ = euclidean(0, 0, 0);
    }

    #[test]
    fn rotation_wraps() {
        let pattern = euclidean(8, 3, 10); // 10 % 8 == 2
        assert_eq!(
            pattern,
            vec![false, true, false, false, true, false, true, false]
        );
    }

    #[test]
    fn large_steps() {
        let pattern = euclidean(64, 5, 0);
        assert_eq!(pattern.len(), 64);
        assert_eq!(pattern.iter().filter(|&&x| x).count(), 5);
    }

    #[test]
    fn pattern_length_always_matches_steps() {
        // Property: The length of the output should always equal steps
        for steps in 1..=32 {
            for pulses in 0..=steps {
                let pattern = euclidean(steps, pulses, 0);
                assert_eq!(
                    pattern.len(),
                    steps as usize,
                    "E({},{}) length mismatch",
                    pulses,
                    steps
                );
            }
        }
    }

    #[test]
    fn pulse_count_always_matches() {
        // Property: The number of pulses in output should always equal pulses parameter
        for steps in 1..=32 {
            for pulses in 0..=steps {
                let pattern = euclidean(steps, pulses, 0);
                let actual_pulses = pattern.iter().filter(|&&x| x).count();
                assert_eq!(
                    actual_pulses, pulses as usize,
                    "E({},{}) pulse count mismatch",
                    pulses, steps
                );
            }
        }
    }

    #[test]
    fn rotation_by_one() {
        let original = euclidean(8, 3, 0);
        let rotated = euclidean(8, 3, 1);
        // Verify the rotation shifted correctly
        assert_eq!(rotated[0], original[1]);
        assert_eq!(rotated[7], original[0]);
    }

    #[test]
    fn pattern_to_string_works() {
        let pattern = euclidean(8, 3, 0);
        assert_eq!(pattern_to_string(&pattern, 'x', '.'), "x..x..x.");
        assert_eq!(pattern_to_string(&pattern, '1', '0'), "10010010");
    }

    #[test]
    fn rotate_pattern_works() {
        let pattern = vec![true, false, false, true];

        // Rotate left by 1
        let rotated = rotate_pattern(&pattern, 1);
        assert_eq!(rotated, vec![false, false, true, true]);

        // Rotate right by 1 (negative rotation)
        let rotated = rotate_pattern(&pattern, -1);
        assert_eq!(rotated, vec![true, true, false, false]);

        // Rotation wraps around
        let rotated = rotate_pattern(&pattern, 5); // 5 % 4 = 1
        assert_eq!(rotated, vec![false, false, true, true]);

        // Zero rotation returns same pattern
        let rotated = rotate_pattern(&pattern, 0);
        assert_eq!(rotated, pattern);

        // Empty pattern
        let empty: Vec<bool> = vec![];
        assert_eq!(rotate_pattern(&empty, 1), empty);
    }
}
