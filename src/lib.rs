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

}

type Seed = u32;

fn rand(seed: Seed) -> (u32, Seed) {
  (seed, seed + 1)
}
