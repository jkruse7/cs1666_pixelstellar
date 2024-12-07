use rand::Rng;

pub fn generate_cave(
    width: usize,
    height: usize,
    wall_prob: f32,
    steps: usize,
    birth_limit: usize,
    survival_limit: usize,
) -> Vec<Vec<u8>> {
    let mut grid = initialize_grid(width, height, wall_prob);

    for _ in 0..steps {
        grid = simulate_step(&grid, birth_limit, survival_limit);
    }

    grid
}

pub fn initialize_grid(width: usize, height: usize, wall_prob: f32) -> Vec<Vec<u8>> {
    let mut rng = rand::thread_rng();
    (0..height)
        .map(|_| {
            (0..width)
                .map(|_| if rng.gen::<f32>() < wall_prob { 1 } else { 0 })
                .collect()
        })
        .collect()
}

pub fn simulate_step(grid: &Vec<Vec<u8>>, birth_limit: usize, survival_limit: usize) -> Vec<Vec<u8>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut new_grid = vec![vec![0; width]; height];

    for y in 0..height {
        for x in 0..width {
            let wall_neighbors = count_wall_neighbors(grid, x, y);

            if grid[y][x] == 1 {
                // Cell is a wall
                new_grid[y][x] = if wall_neighbors >= survival_limit { 1 } else { 0 };
            } else {
                // Cell is empty
                new_grid[y][x] = if wall_neighbors >= birth_limit { 1 } else { 0 };
            }
        }
    }

    new_grid
}

pub fn count_wall_neighbors(grid: &Vec<Vec<u8>>, x: usize, y: usize) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut count = 0;

    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                count += grid[ny as usize][nx as usize];
            } else {
                count += 1; // Treat out-of-bounds as walls
            }
        }
    }

    count.into()
}