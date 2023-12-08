use std::collections::HashMap;

// *********************************************************************************************************
// *********************************************************************************************************
// *********************************************************************************************************
// ***************************                                              ********************************
// ***************************               Total bs star 2                ********************************
// ***************************        just lcm should not be correct        ********************************
// ***************************                                              ********************************
// *********************************************************************************************************
// *********************************************************************************************************
// *********************************************************************************************************

pub fn star2(input: &str) -> usize {
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();
    let map = lines
        .skip(1)
        .map(|line| {
            let binding = line.replace(" ", "");
            let mut it = binding.split('=');
            let from = it.next().unwrap().to_owned();
            let binding = it.next().unwrap().replace('(', "").replace(')', "");
            let mut it2 = binding.split(',');
            let to = (
                it2.next().unwrap().to_owned(),
                it2.next().unwrap().to_owned(),
            );
            (from, to)
        })
        .collect::<HashMap<_, _>>();

    let mut locations = map.keys().filter(|k| k.ends_with('A')).collect::<Vec<_>>();
    let mut steps = 0;
    let mut first_z = (0..locations.len()).map(|_| 0).collect::<Vec<usize>>();
    let mut z_diff = (0..locations.len()).map(|_| 0).collect::<Vec<usize>>();

    while !z_diff.iter().map(|n| n != &0).fold(true, |acc, b| acc && b) {
        let ins_index = steps % instructions.len();
        let ins = instructions[ins_index];
        for i in 0..locations.len() {
            let diff = z_diff[i];
            if diff == 0 {
                let loc = locations[i];
                let options = map.get(&loc.to_string()).unwrap();
                let o1 = &options.0;
                let o2 = &options.1;
                let new_loc = if ins == 'L' { o1 } else { o2 };
                if new_loc.ends_with('Z') {
                    let last_z = first_z[i];
                    if last_z == 0 {
                        first_z[i] = steps;
                    } else {
                        z_diff[i] = steps - last_z;
                    }
                }
                locations[i] = new_loc
            }
        }
        steps += 1
    }
    let lcm = lcm_of_vec(z_diff);
    // let max_start = first_z.iter().reduce(|acc, n| acc.max(n)).unwrap();
    // // let offsets = first_z.iter().map(|start| max_start - start).collect::<Vec<_>>();
    // // dbg!(offsets.clone());
    // // dbg!(z_diff.clone());
    // dbg!(max_start);
    lcm
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn lcm_of_vec(numbers: Vec<usize>) -> usize {
    numbers.into_iter().fold(1, |lcm_so_far, num| lcm(lcm_so_far, num))
}
    // let mut visited_locations: Vec<Vec<(&String, usize)>> =


// fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
//     if a == 0 {
//         (b, 0, 1)
//     } else {
//         let (g, x, y) = extended_gcd(b % a, a);
//         (g, y - (b / a) * x, x)
//     }
// }

// fn mod_inverse(a: i128, m: i128) -> Option<i128> {
//     let (g, x, _) = extended_gcd(a, m);
//     if g == 1 {
//         Some((x % m + m) % m)
//     } else {
//         None
//     }
// }

// fn chinese_remainder_theorem(offsets: Vec<usize>, loop_lengths: Vec<usize>) -> Option<i128> {
//     let residues: Vec<i128> = offsets
//         .iter()
//         .zip(loop_lengths.iter())
//         .map(|(&offset, &length)| {
//             let offset_i128: i128 = offset.try_into().unwrap();
//             let length_i128: i128 = length.try_into().unwrap();
//             // Modifying the offset for the CRT application
//             (length_i128 - offset_i128 % length_i128) % length_i128
//         })
//         .collect();
//     let moduli: Vec<i128> = loop_lengths.iter().map(|&i| i.to_owned().try_into().unwrap()).collect();

//     let prod: i128 = moduli.iter().product();
//     let mut sum = 0;

//     for (&residue, modulus) in residues.iter().zip(&moduli) {
//         let p = prod / modulus;
//         sum += residue * mod_inverse(p, *modulus)? * p;
//     }

//     Some(sum % prod)
// }

#[cfg(test)]
mod test_star2 {
    use super::star2;

    #[test]
    fn it_works() {
        let test_input = include_str!("./t2");
        let result = star2(test_input);
        assert_eq!(result, 6)
    }
}
