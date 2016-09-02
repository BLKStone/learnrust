
fn main() {

    // Don't worry if you don't understand how `fold` works, the point here is that an immutable reference is borrowed.
    fn sum_vec(v: &Vec<i32>) -> i32 {
        return v.iter().fold(0, |a, &b| a + b);
    }
    // Borrow two vectors and sum them.
    // This kind of borrowing does not allow mutation to the borrowed.
    fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
        // do stuff with v1 and v2
        let s1 = sum_vec(v1);
        let s2 = sum_vec(v2);
        // return the answer
        s1 + s2
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];

    let answer = foo(&v1, &v2);
    println!("{}", answer);
}
