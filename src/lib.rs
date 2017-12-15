#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn first_seed() {
        assert_eq!(rand(1), (1, 2));
        assert_eq!(rand(5), (5, 6));
    }

    // #[test]
    // fn test_rand_letter() {
    //     assert_eq!(rand_letter(1), ('a', 2));
    //     assert_eq!(rand_letter(5), ('e', 6));
    // }

    #[test]
    fn test_five_rands() {
        assert_eq!(five_rands(), vec!(1, 2, 3, 4, 5));
    }

    // #[test]
    // fn test_three_rand_letters() {
    //     assert_eq!(three_rand_letters(), "abc");
    // }

    #[test]
    fn test_even_rands() {
        assert_eq!(rand_even(3), (6, 4));
    }

    #[test]
    fn test_odd_rands() {
        assert_eq!(rand_odd(3), (7, 4));
    }
}

type Seed = u32;

type Rand = (u32, Seed);

trait Functor {
    fn map(&self, f: fn(u32) -> u32) -> Self;
}

impl Functor for Rand {
    fn map(&self, f: fn(u32) -> u32) -> Rand {
        (f(self.0), self.1)
    }
}

fn rand(seed: Seed) -> Rand {
    (seed, seed + 1)
}

fn rand_even(seed: Seed) -> Rand {
    rand(seed).map(|v| v * 2)
}

fn rand_odd(seed: Seed) -> Rand {
    rand(seed).map(|v| v * 2 + 1)
}

// fn rand_letter(seed: Seed) -> Rand<char> {
//     rand_map(|val| i_to_a(val), rand(seed))
// }

fn i_to_a(val: u32) -> char {
    let a = 'a' as u32;
    match std::char::from_u32(a - 1 + val) {
        Some(r) => r,
        None => '\0'
    }
}

fn five_rands() -> Vec<u32> {
    let mut seed = 1;
    let rands : Vec<u32> = (0..5).collect();

    rands.iter().map(|_i| {
        let (val, new_seed) = rand(seed);
        seed = new_seed;
        val
    }).collect()
}

// fn three_rand_letters() -> String {
//     let mut seed = 1;
//     let rands : Vec<u32> = (0..3).collect();

//     rands.iter().map(|_i| {
//         let (val, new_seed) = rand_letter(seed);
//         seed = new_seed;
//         val
//     }).collect()
// }
