use std::ops::{Add, Range};

pub fn p1(input: &str) {
    let alamnac = parse_alamanc(input);
    let min = alamnac
        .seeds
        .iter()
        .map(|s| (*s, alamnac.seed_to_location(*s)))
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    println!("Min possible location is {} for seed {}", min.1, min.0);
}

#[derive(Debug)]
struct Alamanac {
    seeds: Vec<u64>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temparature: Map,
    temparature_to_humidity: Map,
    humidity_to_location: Map,
}

#[derive(Debug)]
struct Map {
    data: Vec<(Range<u64>, Range<u64>)>,
}

impl Map {
    fn destination(&self, nr: u64) -> u64 {
        self.data
            .iter()
            .find(|(dr, sr)| sr.contains(&nr))
            .map(|(dr, sr)| nr - sr.start + dr.start)
            .unwrap_or(nr)
    }
}

fn parse_alamanc(input: &str) -> Alamanac {
    let seeds = input
        .lines()
        .next()
        .unwrap()
        .split(" ")
        .filter_map(|str| u64::from_str_radix(str, 10).ok())
        .collect::<Vec<_>>();

    let seed_range_starts = seeds
        .iter()
        .enumerate()
        .skip_while(|s| s.0 % 2 == 0)
        .map(|s| s.1);
    let seed_range_lengths = seeds
        .iter()
        .enumerate()
        .skip_while(|s| s.0 % 2 != 0)
        .map(|s| s.1);

    Alamanac {
        seeds,
        seed_to_soil: parse_map("seed-to-soil map:", input),
        soil_to_fertilizer: parse_map("soil-to-fertilizer map:", input),
        fertilizer_to_water: parse_map("fertilizer-to-water map:", input),
        water_to_light: parse_map("water-to-light map:", input),
        light_to_temparature: parse_map("light-to-temperature map:", input),
        temparature_to_humidity: parse_map("temperature-to-humidity map:", input),
        humidity_to_location: parse_map("humidity-to-location map:", input),
    }
}

fn parse_map(map_name: &str, input: &str) -> Map {
    let map_start_idx = input.find(map_name).unwrap();
    let vec = input[map_start_idx..]
        .lines()
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let mut s = l.split(" ");
            [
                u64::from_str_radix(s.next().unwrap(), 10).unwrap(),
                u64::from_str_radix(s.next().unwrap(), 10).unwrap(),
                u64::from_str_radix(s.next().unwrap(), 10).unwrap(),
            ]
        })
        .collect::<Vec<[u64; 3]>>();

    let mut mdata = Vec::new();
    for l in vec {
        let source_range = l[0]..(l[0] + l[2]);
        let destination_range = l[1]..(l[1] + l[2]);
        mdata.push((source_range, destination_range));
    }

    Map { data: mdata }
}

impl Alamanac {
    fn seed_to_location(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.destination(seed);
        let fertilizer = self.soil_to_fertilizer.destination(soil);
        let water = self.fertilizer_to_water.destination(fertilizer);
        let light = self.water_to_light.destination(water);
        let temparature = self.light_to_temparature.destination(light);
        let humidity = self.temparature_to_humidity.destination(temparature);
        let location = self.humidity_to_location.destination(humidity);

        location
    }
}
