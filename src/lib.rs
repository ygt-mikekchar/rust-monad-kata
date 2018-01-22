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
        assert_eq!(rand_pair(1), (('a', 2), 3));
        assert_eq!(rand_pair(2), (('b', 3), 4));
    }

    #[test]
    fn test_rand_pure() {
        assert_eq!(rand_pure(1), (1, 1));
        assert_eq!(rand_pure(2), (2, 1));
    }

    #[test]
    fn test_gen_pure() {
        assert_eq!(gen_pure(1)(1), (1, 2));
        assert_eq!(gen_pure(2)(1), (2, 2));
        assert_eq!(gen_pure(3)(2), (3, 3));
    }

    #[test]
    fn test_gen_lift2() {
        let (res, seed) = gen_lift2(concat, rand_odd, rand_even)(0);
        assert_eq!(String::from(res), "ab");
        assert_eq!(seed, 2);
    }

    #[test]
    fn test_gen_sequence() {
        let gs = vec![rand, rand, rand, rand, rand];
        assert_eq!(vec![1, 2, 3, 4, 5], gen_sequence(gs)(1).0);
    }

    #[test]
    fn test_gen_even_sequence() {
        let gs = vec![rand_even, rand_even, rand_even, rand_even, rand_even];
        assert_eq!(vec![2, 4, 6, 8, 10], gen_sequence(gs)(2).0);
    }
}

fn concat(a: u32, b: u32) -> String {
    let mut str = String::from("");
    str.push(i_to_a(a));
    str.push(i_to_a(b));
    str
}

type Seed = u32;

type Rand<T> = (T, Seed);

type Gen<T> = fn(Seed) -> Rand<T>;

trait Functor<'t, T, U, F>
    where T: 't,
          F: 't + Fn(T) -> U {
    type Output;
    fn map(self, f: F) -> Self::Output;
}

impl <'t, T, U, F> Functor<'t, T, U, F> for Rand<T>
    where T: 't,
          F: 't + Fn(T) -> U {
    type Output = Rand<U>;
    fn map(self, f: F) -> Self::Output {
        (f(self.0), self.1)
    }
}

impl <'t, T, U, F> Functor<'t, T, U, F> for Gen<T>
    where T: 't,
          F: 't + Fn(T) -> U {
    type Output = Box<'t + Fn(Seed) -> (U, Seed)>;
    fn map(self, f: F) -> Self::Output {
        Box::new(move |s: Seed| {
            let (v, seed) = self(s);
            (f(v), seed)
        })
    }
}

fn gen_lift2<'t, T, U, V>(f: fn(T, U) -> V, t: Gen<T>, u: Gen<U>) -> Box<'t + Fn(Seed) -> Rand<V>>
    where T: 't,
          U: 't,
          V: 't {
    let gen_f = (t as Gen<T>).map(move |a| { move |b| f(a, b) });
    gen_apply(gen_f, u)
}

fn gen_apply<'t, T, U, F>(gen_f: Box<'t + Fn(Seed) -> Rand<F>>, gen_t: Gen<T>) -> Box<'t + Fn(Seed) -> Rand<U>>
    where T: 't,
          U: 't,
          F: 't + FnOnce(T) -> U {
    Box::new(move |s :Seed| {
        let (func, seed1) = gen_f(s);
        let (a, seed2) = gen_t(seed1);
        (func(a), seed2)
    })
}

fn gen_sequence<'t, T>(v: Vec<Gen<T>>) -> Box<'t + Fn(Seed) -> Rand<Vec<T>>>
    where T: 't {
    Box::new(move |s: Seed| {
        let vals = v.iter().map(|x| {
            let (val, seed) = x(s);
            s += seed;
            val
        }).collect();
        (vals, s)
    })
}

fn rand(seed: Seed) -> Rand<u32> {
    (seed, seed + 1)
}

fn rand_even(seed: Seed) -> Rand<u32> {
    (rand as Gen<u32>).map(|v| v * 2)(seed)
}

fn rand_odd(seed: Seed) -> Rand<u32> {
    (rand as Gen<u32>).map(|v| v * 2 + 1)(seed)
}

fn rand_letter(seed: Seed) -> Rand<char> {
    (rand as Gen<u32>).map(|v| i_to_a(v))(seed)
}

fn rand_pair(seed: Seed) -> Rand<(char, u32)> {
    general_pair(rand_letter, rand)(seed)
}

fn gen_pure(v: u32) -> Box<Fn(Seed) -> Rand<u32>> {
    (rand as Gen<u32>).map(move |_v| v )
}

fn rand_pure<T>(t: T) -> Rand<T> {
    (t, 1)
}

fn general_pair<'t, A: 't, B: 't>(gena: Gen<A>, genb: Gen<B>) -> Box<'t + Fn(Seed) -> Rand<(A, B)>> {
    Box::new(move |s: Seed| {
             let (a, seed_a) = gena(s);
             let (b, seed_b) = genb(seed_a);
             ((a, b), seed_b)
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
