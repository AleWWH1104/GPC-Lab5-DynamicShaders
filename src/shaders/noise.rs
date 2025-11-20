// Pseudo-random function for noise generation
fn random2(v: (f32, f32)) -> f32 {
    let dot = v.0 * 12.9898 + v.1 * 78.233;
    let sin_val = dot.sin();
    ((sin_val * 43758.5453).fract() + 1.0) / 2.0 // Remap to [0, 1]
}

// Smooth interpolation function
fn smooth_interpolation(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0) // Quintic interpolation
}

// 2D noise function (simplified Perlin-like)
pub fn noise2d(p: (f32, f32), seed: f32) -> f32 {
    let p = (p.0 + seed, p.1 + seed); // Incorporate seed
    let i = (p.0.floor() as i32, p.1.floor() as i32);
    let f = (p.0 - i.0 as f32, p.1 - i.1 as f32);

    let u = smooth_interpolation(f.0);
    let v = smooth_interpolation(f.1);

    let a = random2((i.0 as f32, i.1 as f32));
    let b = random2(((i.0 + 1) as f32, i.1 as f32));
    let c = random2((i.0 as f32, (i.1 + 1) as f32));
    let d = random2(((i.0 + 1) as f32, (i.1 + 1) as f32));

    let res = a * (1.0 - u) * (1.0 - v) +
              b * u * (1.0 - v) +
              c * (1.0 - u) * v +
              d * u * v;
    res
}

// Fractional Brownian Motion (fBm) for more complex noise
pub fn fbm_noise(p: (f32, f32), time: f32, octaves: usize, persistence: f32) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 1.0;
    let mut frequency = 1.0;
    let mut total_amplitude = 0.0;

    for _ in 0..octaves {
        let noise_input = (p.0 * frequency + time * 0.5, p.1 * frequency);
        value += noise2d(noise_input, time * frequency) * amplitude;
        total_amplitude += amplitude;
        amplitude *= persistence;
        frequency *= 2.0;
    }

    value / total_amplitude
}