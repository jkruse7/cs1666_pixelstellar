use rand::seq::SliceRandom;
use rand::thread_rng;

// fn main() {
//     let perm = generate_permutation_array();

//     let width = 30;
//     // let height = 10;
//     let scale = 0.5;

//     for x in 0..width {
//         let noise_value = perlin(x as f32 * scale, 1.0 as f32 * scale, &perm);
//         print!("{:.2}", noise_value);
//         println!();
//     }
// }

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