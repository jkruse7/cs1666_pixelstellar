use rand::{
    seq::SliceRandom,
    thread_rng,
};

/// Generates a 1D Perlin noise value for a given position and configuration.
///
/// # Arguments
/// 
/// * `x` - The position on the x-axis for which you want to compute the noise. 
/// 
/// * `y` - The position on the y-axis for which you want to compute the noise. 
/// 
/// * `amplitude` - The scaling factor that controls the height of the noise value. 
///                 It multiplies the resulting noise to adjust the range.
///                 A larger amplitude makes the noise values more pronounced (taller peaks and deeper valleys), 
///                 while a smaller amplitude reduces the range of the noise values.
/// 
/// * `frequency` - The frequency factor for how frequently the noise value fluctuates.
///                 A higher frequency causes the noise to fluctuate more frequently over a small range of x-values, 
///                 while a lower frequency makes the noise change more slowly.
pub fn get_1d_pn_value(x: f32, y: f32, amplitude: f32, frequency: f32) -> f32 {
    let perm = generate_permutation_array();
    let noise_value = amplitude * perlin(x as f32 * frequency, 1.0 as f32 * frequency, &perm);
    noise_value
}

fn perlin(x: f32, y: f32, perm: &[usize; 512]) -> f32 {
    let x0 = x.floor() as isize;
    let y0 = y.floor() as isize;
    let x1 = x0 + 1;
    let y1 = y0 + 1;

    let dx = x - x0 as f32;
    let dy = y - y0 as f32;

    let u = fade(dx);
    let v = fade(dy);

    let hash00 = hash(x0, y0, perm);
    let hash01 = hash(x0, y1, perm);
    let hash10 = hash(x1, y0, perm);
    let hash11 = hash(x1, y1, perm);

    let n00 = grad(hash00, dx, dy);
    let n01 = grad(hash01, dx, dy - 1.0);
    let n10 = grad(hash10, dx - 1.0, dy);
    let n11 = grad(hash11, dx - 1.0, dy - 1.0);

    let nx0 = lerp(u, n00, n10);
    let nx1 = lerp(u, n01, n11);

    let mut n = lerp(v, nx0, nx1);
    n += 1.0;
    n /= 2.0;

    n
}

fn generate_permutation_array() -> [usize; 512] {
    let mut perm: [usize; 256] = [0; 256];
    for i in 0..256 {
        perm[i] = i;
    }
    let mut rng = thread_rng();
    perm.shuffle(&mut rng);

    let mut perm_extended: [usize; 512] = [0; 512];
    for i in 0..512 {
        perm_extended[i] = perm[i % 256];
    }
    perm_extended
}

fn fade(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn hash(x: isize, y: isize, perm: &[usize; 512]) -> usize {
    perm[
        (
            perm[
                (x & 255) as usize
            ] + (y & 255) as usize
        ) & 255
    ]
}

fn grad(hash: usize, x: f32, y: f32) -> f32 {
    let gradients: [(f32, f32); 8] = [
        (1.0, 1.0), (-1.0, 1.0), (1.0, -1.0), (-1.0, -1.0),
        (1.0, 0.0), (-1.0, 0.0), (0.0, 1.0), (0.0, -1.0),
    ];

    let  (gx, gy) = gradients[hash & 7];

    gx * x + gy * y
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
    a + t * (b - a)
}