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

    #[test]
    fn test_five_rands() {
        assert_eq!(five_rands(), vec!(1, 2, 3, 4, 5));
    }

}

type Seed = u32;

fn rand(seed: Seed) -> (u32, Seed) {
  (seed, seed + 1)
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
