use shared::math::choose;

fn main() {
    // From https://en.wikipedia.org/wiki/Lattice_path#Counting_lattice_paths, the number of paths
    // can be calculated directly using the combination
    let result = choose(40, 20);
    println!("{result}");
}
