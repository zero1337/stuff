use rand::Rng;
use std::{thread, time::Duration};

const WIDTH: usize = 60;
const HEIGHT: usize = 20;
const NUM_STARS: usize = 40;

// Star struct representing a single star in the field
#[derive(Clone)]
struct Star {
    x: f64,
    y: i32,
    symbol: char,
    speed: f64,
}

impl Star {
    // Create a new star with random properties
    fn new(rng: &mut rand::rngs::ThreadRng) -> Self {
        let symbols = ['.', '*', '+', 'o'];
        Star {
            x: rng.gen_range(0.0..WIDTH as f64),
            y: rng.gen_range(0..HEIGHT as i32),
            symbol: symbols[rng.gen_range(0..symbols.len())],
            speed: rng.gen_range(1.0..3.0),
        }
    }

    // Update star position
    fn update(&mut self) {
        self.x -= self.speed;
        
        // Reset star if it goes off screen
        if self.x < 0.0 {
            self.x = (WIDTH - 1) as f64;
            self.y = rand::thread_rng().gen_range(0..HEIGHT as i32);
        }
    }
}

// Represents the entire star field
struct StarField {
    stars: Vec<Star>,
    display: Vec<Vec<char>>,
}

impl StarField {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let stars = (0..NUM_STARS)
            .map(|_| Star::new(&mut rng))
            .collect();
        
        let display = vec![vec![' '; WIDTH]; HEIGHT];
        
        StarField { stars, display }
    }

    fn update(&mut self) {
        // Update all stars
        for star in &mut self.stars {
            star.update();
        }
    }

    fn draw(&mut self) {
        // Clear display buffer
        for row in &mut self.display {
            row.fill(' ');
        }

        // Place stars in buffer
        for star in &self.stars {
            let x = star.x as usize;
            let y = star.y as usize;
            if x < WIDTH && y < HEIGHT {
                self.display[y][x] = star.symbol;
            }
        }

        // Clear screen (ANSI escape code)
        print!("\x1B[2J\x1B[1;1H");

        // Draw top border
        println!("+{}+", "-".repeat(WIDTH));

        // Draw frame content
        for row in &self.display {
            print!("|");
            for &cell in row {
                print!("{}", cell);
            }
            println!("|");
        }

        // Draw bottom border
        println!("+{}+", "-".repeat(WIDTH));
    }
}

fn main() {
    // Create starfield
    let mut starfield = StarField::new();
    
    println!("Press Ctrl+C to exit");
    
    // Animation loop
    loop {
        starfield.draw();
        starfield.update();
        thread::sleep(Duration::from_millis(100));
    }
}
