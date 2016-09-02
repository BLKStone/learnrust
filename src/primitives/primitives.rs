
fn main() {
    // Variables can be type annotated
    let logical: bool = true;

    // regular annotation
    let a_float: f64 = 1.0;
    // suffix annotation
    let an_integer = 5i32;

    // Or a default will be used
    let default_float = 3.0; // `f64`
    let default_integer = 7; // `i32`

    let mut mutable = 12; // Mutable `i32`

    // Error! The type of a variable can't be changed
    // mutable = true;

}
