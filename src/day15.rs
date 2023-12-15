fn hash(bytes: &[u8]) -> u8 {
    bytes
        .iter()
        .fold(0u8, |acc, &b| acc.wrapping_add(b).wrapping_mul(17))
}

fn lenses(input: &str) -> impl Iterator<Item = &str> {
    input.trim().split(',')
}

pub fn part1(input: &str) -> String {
    lenses(input)
        .map(|s| hash(s.as_bytes()) as usize)
        .sum::<usize>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    // Asymptotically, this solution is not very efficient, as scanning a Vec or removing an
    // element is O(n), whereas with something like a linked hash map, it would be O(1). However,
    // the lists stay short enough that using a vector is over 2x faster than a linked hash map
    // for my input.
    let mut boxes: [Vec<(&str, usize)>; 256] = std::array::from_fn(|_| Vec::new());
    for lens in lenses(input) {
        if let Some(name) = lens.strip_suffix('-') {
            let hash = hash(name.as_bytes());
            let lensbox = &mut boxes[hash as usize];
            
            // Remove the lens from the box if it's in there.
            if let Some(idx) = lensbox.iter().position(|(n, _)| n == &name) {
                lensbox.remove(idx);
            }
        } else {
            let (name, focal_length) = lens.split_once('=').unwrap();
            let focal_length = focal_length.parse().unwrap();
            let hash = hash(name.as_bytes());
            let lensbox = &mut boxes[hash as usize];

            // If the lens is already in the box, replace it. Otherwise, add it.
            if let Some((_, existing_focal_length)) = lensbox.iter_mut().find(|(n, _)| n == &name) {
                *existing_focal_length = focal_length;
            } else {
                lensbox.push((name, focal_length));
            }
        }
    }

    let mut focusing_power = 0;
    for (box_idx, lensbox) in boxes.iter().enumerate() {
        for (lens_idx, (_, focal_length)) in lensbox.iter().enumerate() {
            focusing_power += (box_idx + 1) * (lens_idx + 1) * focal_length
        }
    }

    focusing_power.to_string()
}
