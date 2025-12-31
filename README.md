# euclidean-rhythm

A Rust library for generating Euclidean rhythms using Bjorklund's algorithm.

Euclidean rhythms are mathematical patterns that distribute pulses as evenly as possible across a given number of steps. These patterns appear in traditional music from around the world and are popular in electronic music production.

## Features

- Generate Euclidean rhythm patterns with any number of steps and pulses
- Rotate patterns to different starting positions
- Convert patterns to string representations for visualization
- Fast, lightweight implementation with no dependencies
- Well-tested with property-based tests

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
euclidean-rhythm = "0.1.0"
```

## Usage

### Basic Pattern Generation

```rust
use euclidean_rhythm::euclidean;

// Generate a tresillo rhythm (3 pulses in 8 steps)
let pattern = euclidean(8, 3, 0);
// Result: [true, false, false, true, false, false, true, false]
```

### Pattern Visualization

```rust
use euclidean_rhythm::{euclidean, pattern_to_string};

let pattern = euclidean(8, 3, 0);
println!("{}", pattern_to_string(&pattern, 'x', '.'));
// Output: x..x..x.

// Or use different characters
println!("{}", pattern_to_string(&pattern, '1', '0'));
// Output: 10010010
```

### Pattern Rotation

```rust
use euclidean_rhythm::euclidean;

// Rotate the pattern by 2 steps
let pattern = euclidean(8, 3, 2);
// Result: [false, true, false, false, true, false, true, false]

// Rotation wraps around (10 % 8 = 2)
let pattern2 = euclidean(8, 3, 10);
assert_eq!(pattern, pattern2);
```

## Musical Examples

The library generates well-known rhythmic patterns from various musical traditions:

```rust
use euclidean_rhythm::{euclidean, pattern_to_string};

// Tresillo - Common in Latin music (Cuba, Haiti)
let tresillo = euclidean(8, 3, 0);
println!("Tresillo:          {}", pattern_to_string(&tresillo, 'x', '.'));
// Output: x..x..x.

// West African bell pattern
let bell = euclidean(8, 5, 0);
println!("West African Bell: {}", pattern_to_string(&bell, 'x', '.'));
// Output: x.xx.xx.

// Persian rhythm
let persian = euclidean(12, 5, 0);
println!("Persian:           {}", pattern_to_string(&persian, 'x', '.'));
// Output: x..x.x..x.x.

// Bossa Nova rhythm
let bossa = euclidean(16, 7, 0);
println!("Bossa Nova:        {}", pattern_to_string(&bossa, 'x', '.'));
// Output: x..x.x.x..x.x.x.
```

## Algorithm

This library implements Bjorklund's algorithm, which generates maximally even distributions of pulses. The algorithm works by:

1. Starting with separate groups of pulses and rests
2. Repeatedly pairing groups together
3. Concatenating pairs until a single pattern emerges

This produces rhythms where pulses are distributed as evenly as possible, which is why these patterns sound natural and appear in traditional music worldwide.

## API Reference

### `euclidean(steps: u8, pulses: u8, rotation: u8) -> Vec<bool>`

Generates a Euclidean rhythm pattern.

**Parameters:**

- `steps` - Total number of steps in the pattern (must be > 0)
- `pulses` - Number of pulses to distribute (must be ≤ steps)
- `rotation` - Number of positions to rotate the pattern (wraps with modulo)

**Returns:**

- A `Vec<bool>` where `true` represents a pulse and `false` represents a rest

**Panics:**

- Panics if `steps == 0` or `pulses > steps`

### `pattern_to_string(pattern: &[bool], pulse_char: char, rest_char: char) -> String`

Converts a boolean pattern to a string representation.

**Parameters:**

- `pattern` - The pattern to convert
- `pulse_char` - Character to represent pulses (true values)
- `rest_char` - Character to represent rests (false values)

**Returns:**

- A `String` with the pattern visualized using the specified characters

## Testing

Run the test suite:

```bash
cargo test
```

The library includes:

- Unit tests for known musical patterns
- Property-based tests (528 test cases)
- Edge case tests (empty patterns, full patterns, rotation)
- Panic tests for invalid inputs

## References

- [The Euclidean Algorithm Generates Traditional Musical Rhythms](http://cgm.cs.mcgill.ca/~godfried/publications/banff.pdf) by Godfried Toussaint
- [Bjorklund's Algorithm](https://en.wikipedia.org/wiki/Euclidean_rhythm)

## License

This project is released into the public domain. See LICENSE for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
