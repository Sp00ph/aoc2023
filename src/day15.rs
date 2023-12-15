use linked_hash_map::{Entry, LinkedHashMap};

fn hash(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b).wrapping_mul(17))
}

fn lenses(input: &str) -> impl Iterator<Item = &str> {
    input.trim().split(',')
}

pub fn part1(input: &str) -> String {
    lenses(input).map(|s| hash(s.as_bytes()) as usize).sum::<usize>().to_string()
}

pub fn part2(input: &str) -> String {
    // We use `LinkedHashMap` because it keeps track of insertion order,
    // but allows efficient removal of elements in the middle of the list.
    // `Vec` would require O(n) to find and remove a given box, whereas
    // `LinkedHashMap` does it in O(1).
    let mut boxes: [LinkedHashMap<&str, usize>; 256] =
        std::array::from_fn(|_| LinkedHashMap::new());
    for lens in lenses(input) {
        if let Some(name) = lens.strip_suffix('-') {
            let hash = hash(name.as_bytes());
            boxes[hash as usize].remove(name);
        } else {
            let (name, focal_length) = lens.split_once('=').unwrap();
            let focal_length = focal_length.parse().unwrap();
            let hash = hash(name.as_bytes());
            let lensbox = &mut boxes[hash as usize];

            // we can't just use `lensbox.insert()`, because
            // that would update the lenses position to the
            // end of the map.
            match lensbox.entry(name) {
                Entry::Occupied(mut o) => {
                    *o.get_mut() = focal_length;
                }
                Entry::Vacant(v) => {
                    v.insert(focal_length);
                }
            }
        }
    }

    let mut focusing_power = 0;
    for (box_idx, lensbox) in boxes.iter().enumerate() {
        for (lens_idx, focal_length) in lensbox.values().enumerate() {
            focusing_power += (box_idx + 1) * (lens_idx + 1) * focal_length
        }
    }

    focusing_power.to_string()
}
