use euclidean_rhythm::{euclidean, pattern_to_string};

fn main() {
    println!("Common Euclidean Rhythm Patterns\n");
    println!("Format: E(pulses, steps) - Pattern name\n");

    let patterns = [
        (8, 3, "Cuban tresillo"),
        (8, 5, "West African bell"),
        (12, 5, "Persian rhythm"),
        (16, 7, "Brazilian bossa nova"),
        (8, 7, "Rock/funk pattern"),
        (16, 5, "Afro-Cuban cinquillo"),
        (12, 7, "Persian darbuka"),
        (16, 9, "Complex polyrhythm"),
    ];

    for (steps, pulses, name) in patterns {
        let pattern = euclidean(steps, pulses, 0);
        let display = pattern_to_string(&pattern, 'x', '.');
        println!("{:25} E({:2},{:2}): {}", name, pulses, steps, display);
    }

    println!("\nLegend: x = pulse (hit), . = rest (silence)");
}
