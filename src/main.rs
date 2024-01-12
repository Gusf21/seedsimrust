use rand::Rng;
use std::thread;
use std::time::Duration;
use std::io;

struct Tile {
    soil: bool,
    seed: bool,
    plant: bool
}

struct Field {
    height: i16,
    width: i16,
    year: i16,
    tiles: Vec<Tile>
}

impl Tile {
    fn default(seeded: bool) -> Tile {
        let mut tile = Tile {
            soil: true,
            seed:false,
            plant: false
        };
        if seeded {
            tile.seed = true;
        }
        return tile;
    }
}

impl Field {
    fn initialise(height: i16, width: i16) -> Field {
        let mut x = Field {
            height: height,
            width: width,
            year: 1,
            tiles: Vec::new()
        };
        x.populate();
        return x;
    }

    fn populate(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                if ((self.height / 2) as i16 == i - 1) & ((self.width / 2) as i16 == j - 1) {
                    self.tiles.push(Tile::default(true));
                }
                else {
                    self.tiles.push(Tile::default(false));
                }
            }
        }
    }

    fn count_plants(&self) {
        let mut plants: i16 = 0;
        for tile in &self.tiles {
            if tile.plant {
                plants += 1;
            }
        }
        if plants == 1 {
            println!("There is 1 plant growing");
        }
        else {
            println!("There are {} plant's growing", plants);
        }
    }

    fn plant_seed(&mut self, position: i16) {
        if (position >= 0) & (position < self.tiles.len() as i16) {
            if self.tiles[position as usize].soil & !self.tiles[position as usize].plant {
                self.tiles[position as usize].seed = true;
           }
    }
    }

    fn plant_seeds(&mut self, position: i16) {
        if (position % self.width != 0) | (position == 0) {
            self.plant_seed(position - 1);
            self.plant_seed(position - (self.width + 1));
            self.plant_seed(position + (self.width - 1));
        }
        if (position % self.width - 1 != 0) | (position == 0) {
            self.plant_seed(position + 1);
            self.plant_seed(position - (self.width - 1));
            self.plant_seed(position + (self.width + 1));
        }
        self.plant_seed(position - self.width);
        self.plant_seed(position + self.width);
    }

    fn sim_spring(&mut self) {
        let mut frost: bool = false;
        for tile in &mut self.tiles {
            if tile.seed {
                tile.seed = false;
                tile.plant = true;
            }
        }
        let mut rng = rand::thread_rng();
        if rng.gen_range(0..2) == 1 {
            frost = true;
            let mut plant_count = 0;
            for tile in &mut self.tiles {
                if tile.plant {
                    plant_count += 1;
                    if plant_count % 3 == 0 {
                        tile.plant = false;
                    }
                }
            }
        }
        println!("Year: {}\nSeason: {}\n", self.year, "spring");
        if frost {
            println!("There has been a frost!");
            self.count_plants();
        }
        self.display_field();

    }

    fn sim_summer(&mut self) {
        let mut drought = false;
        let mut rng = rand::thread_rng();
        if rng.gen_range(0..3) == 0 {
            drought = true;
            let mut plant_count = 0;
            for tile in &mut self.tiles {
                if tile.plant {
                    plant_count += 1;
                    if plant_count % 2 == 0 {
                        tile.plant = false;
                    }
                }
            }
        }
        println!("Year: {}\nSeason: {}\n", self.year, "summer");
        if drought {
            println!("There has been a drought!");
            self.count_plants();
        }
        self.display_field();
    }

    fn sim_autumn(&mut self) {
        let length = self.tiles.len() as i16;
        for i in 0..length {
            if self.tiles[i as usize].plant {
                self.plant_seeds(i);
            }
        }
        println!("Year: {}\nSeason: {}\n", self.year, "autumn");
        self.display_field();
    }

    fn sim_winter(&mut self) {
        for tile in &mut self.tiles {
            if tile.plant {
                tile.plant = false
            }
        }
        println!("Year: {}\nSeason: {}\n", self.year, "winter");
        self.display_field();
        self.year += 1;
    }

    fn sim_year(&mut self) {
        self.sim_spring();
        self.sim_summer();
        self.sim_autumn();
        self.sim_winter();
    }

    fn display_field(&self) {
        let mut index = 0;
        let mut row = 0;
        print!("\n");
        for tile in &self.tiles {
            if tile.plant {
                print!("P");
            } else if tile.seed {
                print!("S")
            } else if tile.soil {
                print!(".")
            }
            index += 1;
            if (index / (row + 1)) == self.width {
                println!("|{:3}", row);
                row += 1;
            }
        }
        print!("\n");
    }

    fn list(&self) {
        let num: usize = self.tiles.len();
        println!("Number of tiles is {}", num);
    }
}

fn main() {
    let mut active: bool = true;
    let mut field = Field::initialise(50, 75);

    field.list();

    let binding = get_mode();
    let mode = binding.as_str();

    if  mode == "t" {
        loop {
            field.sim_year();
            thread::sleep(Duration::from_secs(1));
        }
    }
    else if mode == "s" {
        while active {
            field.sim_year();
            println!("Press enter to continue, press X to stop");
            let mut input_string: String = String::new();
            _ = io::stdin().read_line(&mut input_string);
            if input_string.trim().to_lowercase() == "x" {
                active = false
            }
        }
    }
    else {
        let yearnum: i16 = mode.parse::<i16>().unwrap();
        for _i in 0..yearnum {
            field.sim_year();
            thread::sleep(Duration::from_secs(1));
        }
    }

    
}

fn get_mode() -> String {
    let mut valid = false;
    let mut input_string: String = String::new();
    let mut return_string = "";
    while !valid {
        println!("Please choose your mode, t for timer stepping, s for manual stepping, or a positive integer for a set number of years.");
        match io::stdin().read_line(&mut input_string) {
            Ok(_result) => {}
            Err(error) => {println!("Error: {}", error)}
        }

        return_string = input_string.trim();

        if return_string.to_lowercase() == "t" {
            valid = true;
        }
        else if return_string.to_lowercase() == "s" {
            valid = true;
        }
        else if return_string.parse::<i16>().is_ok()  {
            if return_string.parse::<i16>().unwrap() > 0 {
                valid = true;
            }
        } else {
            println!("Invalid input!");
        }
    }
    return String::from(return_string);
}