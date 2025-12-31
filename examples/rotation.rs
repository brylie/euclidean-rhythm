use euclidean_rhythm::{euclidean, pattern_to_string, rotate_pattern};

fn main() {
    println!("Euclidean Rhythm Rotation Demo\n");

    let steps = 8;
    let pulses = 3;

    println!("Base pattern: E({},{})\n", pulses, steps);

    // Show rotations using euclidean function
    println!("Using euclidean() with rotation parameter:");
    for rotation in 0..steps {
        let pattern = euclidean(steps, pulses, rotation);
        let display = pattern_to_string(&pattern, 'x', '.');
        println!("Rotation {:2}: {}", rotation, display);
    }

    println!("\nUsing rotate_pattern() function:");
    let base_pattern = euclidean(steps, pulses, 0);

    for rotation in 0..steps {
        let pattern = rotate_pattern(&base_pattern, rotation as i32);
        let display = pattern_to_string(&pattern, 'x', '.');
        println!("Rotation {:2}: {}", rotation, display);
    }

    println!("\nNegative rotation (rotate right):");
    for rotation in -3..=0 {
        let pattern = rotate_pattern(&base_pattern, rotation);
        let display = pattern_to_string(&pattern, 'x', '.');
        println!("Rotation {:2}: {}", rotation, display);
    }
}
