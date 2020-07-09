use comp_prog_lib_rs::data_structure::unionfind;
use proconio::input;


fn main() {
    input!(n: usize,  q: usize);
    let mut uf = unionfind::UnionFind::new(n);
    for _ in 0..q {
        input!(p: usize,  a: usize, b: usize);
        if p == 0 {
            uf.unite(a, b);
        } else {
            let is_same = uf.same(a, b);
            if is_same {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}