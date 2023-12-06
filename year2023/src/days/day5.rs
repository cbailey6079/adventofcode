use crate::{utils, Day};

pub struct Day5 {}

#[derive(Debug)]
struct Map {
    source: u64,
    destination: u64,
    range: u64,
}

impl Day for Day5 {
    fn part1(&self, file: String) -> String {
        let mut locations: Vec<u64> = Vec::new();
        let (seeds, maps) = read_map(file);

        for mut seed in seeds {
            for map in &maps {
                for row in map {
                    if seed >= row.source && seed <= row.source + row.range - 1 {
                        seed = seed.abs_diff(row.source) + row.destination;
                        break;
                    }
                }
            }
            locations.push(seed);
        }

        locations.iter().min().expect("min").to_string()
    }


    // you brute forced this problem because you are lazy, so don't run it.
    fn part2(&self, file: String) -> String {
        let mut location: u64 = u64::MAX;
        let (seeds, maps) = read_map(file);

        for i in (0..seeds.len()).step_by(2) {
            for mut seed in seeds[i]..seeds[i]+seeds[i+1] {
                for map in &maps {
                    for row in map {
                        if seed >= row.source && seed <= row.source + row.range - 1 {
                            seed = seed.abs_diff(row.source) + row.destination;
                            break;
                        }
                    }
                }
                if seed < location {
                    location = seed;
                }
            }
        }
        

        location.to_string()
    }
}

fn read_map(file: String) -> (Vec<u64>, Vec<Vec<Map>>) {
    let lines = utils::read_lines(format!("./src/files/day5/{file}.txt").as_str());
    let mut key;
    let mut maps: Vec<Vec<Map>> = Vec::new();
    let mut map: Vec<Map> = Vec::new();
    let mut seeds: Vec<u64> = Vec::new();
    let mut index = 0;
    
    for line in &lines {
        if line.is_empty() { continue; }

        if line.contains(":") {
            key = line.split(":").collect::<Vec<&str>>()[0];

            if key == "seeds" {
                seeds = line.split(":").collect::<Vec<&str>>()[1]
                    .split_ascii_whitespace()
                    .map(|i| i.parse::<u64>().unwrap())
                    .collect();
            } else {
                if index >= 1 { 
                    maps.push(map);
                    map = Vec::new(); 
                }
                index += 1;
            }

            continue;
        }

        let items: Vec<u64> = line
            .split(" ")
            .map(|i| i.parse::<u64>().unwrap())
            .collect();

        map.push(Map {destination: items[0], source: items[1], range: items[2]});
    }

    maps.push(map);

    (seeds, maps)
}