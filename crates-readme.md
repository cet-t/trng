# trng

A tiny, `no_std`, allocation-free abstraction for random number generators.

## Usage

```rust
use trng::Rng;

struct XorShift32 {
    state: u32,
}

impl Rng for XorShift32 {
    type Word = u32;

    fn nextu(&mut self) -> u32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }
}

fn main() {
    let mut rng = XorShift32 { state: 2463534242 };

    let _: u32 = rng.nextu();             // [0, 2^32-1]
    let _: f32 = rng.nextf();             // [0, 1)
    let _: i32 = rng.randi(-10, 10);      // [-10, 10]
    let _: f32 = rng.randf(-1.0, 1.0);    // [-1.0, 1.0)
}
```

## Notes

- **No cryptographic security guarantees.**
  - Implementing `Rng` does not imply that a generator is cryptographically secure. In particular, the biased `randi` mapping is unsuitable where unbiased sampling is required.

- **`randi` has a slight bias.** This function uses "multiply-shift" to map a word to the range `[min, max]` without rejection sampling. Let `N` be the word width (32 or 64) and `range = max - min + 1`. Then:
  - If `2^N` is divisible by `range` (e.g., `range` is a power of two), the result is perfectly uniform.
  - Otherwise, each value is generated from either `floor(2^N / range)` or `ceil(2^N / range)` input words, so the probability of each value differs by at most `2^-N`.
  - The relative bias (the ratio of the highest to lowest probability) is `ceil / floor`, which is approximately `1 + range / 2^N` when `range` is much smaller than `2^N`. When `range` is slightly larger than `2^(N-1)`, this ratio can be up to 2:1.
  - For perfect uniformity, apply rejection sampling in addition to `nextu`.

- `randi` assumes `min <= max` but does not check this condition.
