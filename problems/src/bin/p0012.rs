use itertools::Itertools;
use prime_factorization::Factorization;
use shared::math::TriangleNumbers;

fn main() {
    let result = first_triangle(500);
    println!("{result}");
}

fn first_triangle(min_factors: usize) -> u64 {
    let triangles = TriangleNumbers::new();
    for triangle in triangles {
        let prime_factors = Factorization::run(triangle);
        let num_factors = prime_factors
            .factors
            .into_iter()
            .chunk_by(|&x| x)
            .into_iter()
            .map(|(_key, chunk)| chunk.count() + 1)
            .reduce(|acc, e| acc * e);
        if let Some(f) = num_factors
            && min_factors < f
        {
            return triangle;
        }
    }
    unreachable!()
}
