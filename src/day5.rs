use core::fmt;
use std::{ops::Range, str::Lines};

#[derive(Debug)]
struct Map {
    ranges: Vec<MapRange>,
}

#[derive(Clone, Copy)]
struct MapRange {
    dst: usize,
    src: usize,
    len: usize,
}

impl MapRange {
    fn take(&mut self, len: usize) {
        assert!(len <= self.len);
        self.len -= len;
        self.src += len;
        self.dst += len;
    }
}

impl fmt::Debug for MapRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}..{} -> {}..{}",
            self.src,
            self.src + self.len,
            self.dst,
            self.dst + self.len
        )
    }
}

#[derive(Debug)]
struct Input {
    seeds: Vec<usize>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temp: Map,
    temp_to_humidity: Map,
    humidity_to_location: Map,
}

impl Map {
    fn map(&self, seed: usize) -> usize {
        let range_idx = match self.ranges.partition_point(|r| r.src <= seed) {
            0 => return seed,
            n => n - 1,
        };
        let range = &self.ranges[range_idx];
        let offset = seed - range.src;
        if offset < range.len {
            range.dst + offset
        } else {
            seed
        }
    }

    /// Returns a map `m` such that `m.map_seed(seed) == self.map_seed(rhs.map_seed(seed))`
    fn compose(&self, rhs: &Map) -> Map {
        // This works similarly to the merge step of merge sort. We sort the ranges of rhs by
        // their destination start and the ranges of lhs by their source start. Then we iterate
        // through both lists, merging the ranges as we go.

        let mut lhs_ranges = self.ranges.clone();
        let mut rhs_ranges = rhs.ranges.clone();
        rhs_ranges.sort_unstable_by_key(|r| r.dst);

        let mut out_ranges = Vec::new();

        let (mut i, mut j) = (0, 0);

        while i < lhs_ranges.len() && j < rhs_ranges.len() {
            let lhs_range = &mut lhs_ranges[i];
            let rhs_range = &mut rhs_ranges[j];

            if lhs_range.len == 0 {
                i += 1;
                continue;
            }

            if rhs_range.len == 0 {
                j += 1;
                continue;
            }

            if lhs_range.src < rhs_range.dst {
                let len = (rhs_range.dst - lhs_range.src).min(lhs_range.len);
                out_ranges.push(MapRange {
                    dst: lhs_range.dst,
                    src: lhs_range.src,
                    len,
                });
                lhs_range.take(len);
                continue;
            }

            if rhs_range.dst < lhs_range.src {
                let len = (lhs_range.src - rhs_range.dst).min(rhs_range.len);
                out_ranges.push(MapRange {
                    dst: rhs_range.dst,
                    src: rhs_range.src,
                    len,
                });
                rhs_range.take(len);
                continue;
            }
            let len = lhs_range.len.min(rhs_range.len);
            out_ranges.push(MapRange {
                dst: lhs_range.dst,
                src: rhs_range.src,
                len,
            });
            lhs_range.take(len);
            rhs_range.take(len);
        }

        while i < lhs_ranges.len() {
            let lhs_range = &lhs_ranges[i];
            if lhs_range.len == 0 {
                i += 1;
                continue;
            }
            out_ranges.push(*lhs_range);
            i += 1;
        }

        while j < rhs_ranges.len() {
            let rhs_range = &rhs_ranges[j];
            if rhs_range.len == 0 {
                j += 1;
                continue;
            }
            out_ranges.push(*rhs_range);
            j += 1;
        }

        out_ranges.sort_unstable_by_key(|r| r.src);

        Map { ranges: out_ranges }
    }

    fn min_output_in_input_range(&self, range: Range<usize>) -> usize {
        let min_in_map_range = |map_range: &MapRange| {
            let overlaps = range.start < map_range.src + map_range.len && range.end > map_range.src;
            if !overlaps {
                return None;
            }
            let offset = range.start.saturating_sub(map_range.src);
            Some(map_range.dst + offset)
        };

        self.ranges
            .iter()
            .filter_map(min_in_map_range)
            .min()
            .unwrap()
    }
}

impl Input {
    fn map_seed(&self, seed: usize) -> usize {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temp = self.light_to_temp.map(light);
        let humidity = self.temp_to_humidity.map(temp);
        self.humidity_to_location.map(humidity)
    }

    fn compose_all(&self) -> Map {
        self.humidity_to_location
            .compose(&self.temp_to_humidity)
            .compose(&self.light_to_temp)
            .compose(&self.water_to_light)
            .compose(&self.fertilizer_to_water)
            .compose(&self.soil_to_fertilizer)
            .compose(&self.seed_to_soil)
    }
}

fn parse_seeds(line: &str) -> Vec<usize> {
    line.strip_prefix("seeds: ")
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn parse_map(lines: &mut Lines) -> Map {
    let _name = lines.next();
    let mut ranges = Vec::new();

    for line in lines {
        if line.is_empty() {
            break;
        }
        let (dst, line) = line.split_once(' ').unwrap();
        let (src, len) = line.split_once(' ').unwrap();
        let dst = dst.trim().parse().unwrap();
        let src = src.trim().parse().unwrap();
        let len = len.trim().parse().unwrap();
        ranges.push(MapRange { dst, src, len });
    }

    // Maybe this will allow a nice binary search later?
    ranges.sort_unstable_by_key(|r| r.src);

    Map { ranges }
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.trim().lines();
    let seeds = parse_seeds(lines.next().unwrap());
    let _ = lines.next();

    Input {
        seeds,
        seed_to_soil: parse_map(&mut lines),
        soil_to_fertilizer: parse_map(&mut lines),
        fertilizer_to_water: parse_map(&mut lines),
        water_to_light: parse_map(&mut lines),
        light_to_temp: parse_map(&mut lines),
        temp_to_humidity: parse_map(&mut lines),
        humidity_to_location: parse_map(&mut lines),
    }
}

pub fn part1(input: &str) -> String {
    let input = parse_input(input);
    input
        .seeds
        .iter()
        .map(|s| input.map_seed(*s))
        .min()
        .unwrap()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let input = parse_input(input);
    let composed = input.compose_all();
    let seed_ranges = input.seeds.chunks(2).map(|c| c[0]..c[0] + c[1]);

    seed_ranges
        .map(|r| composed.min_output_in_input_range(r))
        .min()
        .unwrap()
        .to_string()
}
