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
    fn test_rand_letter() {
        assert_eq!(rand_letter(1), ('a', 2));
        assert_eq!(rand_letter(5), ('e', 6));
    }

    #[test]
    fn test_five_rands() {
        assert_eq!(five_rands(), vec!(1, 2, 3, 4, 5));
    }

    #[test]
    fn test_three_rand_letters() {
        assert_eq!(three_rand_letters(), "abc");
    }

    #[test]
    fn test_even_rands() {
        assert_eq!(rand_even(3), (6, 4));
    }

    #[test]
    fn test_odd_rands() {
        assert_eq!(rand_odd(3), (7, 4));
    }

    #[test]
    fn test_rand_pair() {
        assert_eq!(rand_pair(4), (('d', 5), 6));
    }
}

type Seed = u32;
type Rand<T> = (T, Seed);
type Gen<T> = fn(Seed) -> Rand<T>;

trait Functor<A, B> {
    type Output;

    fn map(self, f: fn(A) -> B) -> Self::Output;
}

impl<A, B> Functor<A, B> for Rand<A> {
    type Output = Rand<B>;
    fn map(self, f: fn(A) -> B) -> Self::Output {
        (f(self.0), self.1)
    }
}

fn rand(seed: Seed) -> Rand<u32> {
    (seed, seed + 1)
}

fn rand_even(seed: Seed) -> Rand<u32> {
    rand(seed).map(|val| val * 2)
}

fn rand_odd(seed: Seed) -> Rand<u32> {
    rand(seed).map(|val| val * 2 + 1)
}

fn rand_letter(seed: Seed) -> Rand<char> {
    rand(seed).map(|val| i_to_a(val))
}

fn rand_pair(seed: Seed) -> Rand<(char, u32)> {
    general_pair(rand_letter, rand)(seed)
}

fn general_pair<'a, A, B>(gena: Gen<A>, genb: Gen<B>) -> Box<'a + Fn(Seed) -> Rand<(A, B)>>
    where A: 'a,
          B: 'a {
    Box::new(move |seed| {
        let (c, s1) = gena(seed);
        let (i, s2) = genb(s1);
        ((c, i), s2)
    })
}

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

fn three_rand_letters() -> String {
    let mut seed = 1;
    let rands : Vec<u32> = (0..3).collect();

    rands.iter().map(|_i| {
        let (val, new_seed) = rand_letter(seed);
        seed = new_seed;
        val
    }).collect()
}
