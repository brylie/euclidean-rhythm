use euclidean_rhythm::{euclidean, pattern_to_string};

fn main() {
    println!("Euclidean Rhythm Visualization\n");

    let steps = 16;

    println!("Patterns with {} steps:\n", steps);

    for pulses in 1..=steps {
        let pattern = euclidean(steps, pulses, 0);

        // Show different visualizations
        let binary = pattern_to_string(&pattern, '1', '0');
        let drums = pattern_to_string(&pattern, 'x', '.');
        let blocks = pattern_to_string(&pattern, '█', '░');

        println!("E({:2},{:2})", pulses, steps);
        println!("  Binary: {}", binary);
        println!("  Drums:  {}", drums);
        println!("  Blocks: {}", blocks);
        println!();
    }

    println!("\nCircular visualization of E(5,8):");
    let pattern = euclidean(8, 5, 0);
    println!("     {}", if pattern[0] { "x" } else { "." });
    println!(
        "   {}   {}",
        if pattern[7] { "x" } else { "." },
        if pattern[1] { "x" } else { "." }
    );
    println!(
        " {}       {}",
        if pattern[6] { "x" } else { "." },
        if pattern[2] { "x" } else { "." }
    );
    println!(
        "{}    O    {}",
        if pattern[5] { "x" } else { "." },
        if pattern[3] { "x" } else { "." }
    );
    println!(
        "   {}   {}",
        if pattern[4] { "x" } else { "." },
        " ".repeat(3)
    );
    println!(
        "\nReading clockwise from top: {}",
        pattern_to_string(&pattern, 'x', '.')
    );
}
