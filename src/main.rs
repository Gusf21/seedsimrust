struct Tile {
    soil: bool,
    seed: bool,
    plant: bool,
    rocks: bool
}

struct Field {
    height: i16,
    width: i16,
    tiles: Vec<Tile>
}

impl Tile {
    fn default() -> Tile {
        Tile {
            soil: true,
            seed:true,
            plant: false,
            rocks: false
        }
    }

    fn read(&self) {
        println!("Tile is\nSoil - {}\nSeed - {}\nPlant - {}\nRocks - {}", self.soil, self.seed, self.plant, self.rocks);
    }
}

impl Field {
    fn initialise(height: i16, width: i16) -> Field {
        let mut x = Field {
            height: height,
            width: width,
            tiles: Vec::new()
        };
        x.populate();
        return x;
    }

    fn populate(&mut self) {
        for _i in 0..self.height {
            for _j in 0..self.width {
                self.tiles.push(Tile::default());
            }
        }
    }

    fn sim_spring(&mut self) {
        for tile in &mut self.tiles {
            if tile.seed {
                tile.seed = false;
                tile.plant = true;
            }
        }
    }

    fn list(&self) {
        let num: usize = self.tiles.len();
        println!("Number of tiles is {}", num);
    }
}

fn main() {
    let field = Field::initialise(20, 20);
    field.list();


}