// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Scroll down for hints :)

pub fn main() {
    let mut vec0 = Vec::new();

    // 我靠这个 vec1 为什么 不用加 mut
    let vec1 = fill_vec(&mut vec0);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

}

fn fill_vec(vec: &mut Vec<i32>) -> &mut Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

















// The difference between this one and the previous ones is that the first line
// of `fn fill_vec` that had `let mut vec = vec;` is no longer there. You can,
// instead of adding that line back, add `mut` in one place that will change
// an existing binding to be a mutable binding instead of an immutable one :)
