use comp_prog_lib_rs::data_structure::unionfind;
use proconio::input;
use comp_prog_lib_rs::math::combination::Combination;

fn main() {
    // input!(n: usize, q: usize);
    // let mut uf = unionfind::UnionFind::new(n);
    // for _ in 0..q {
    //     input!(p: usize, a: usize, b: usize);
    //     if p == 0 {
    //         uf.unite(a, b);
    //     } else {
    //         let is_same = uf.same(a, b);
    //         if is_same {
    //             println!("Yes");
    //         } else {
    //             println!("No");
    //         }
    //     }
    // }
    let comb = Combination::new(200);
    dbg!(comb.combination(5, 3));
    dbg!(comb.combination(10, 5));
    dbg!(comb.combination(100 + 100 - 1, 99));
}
