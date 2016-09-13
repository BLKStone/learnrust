use std::collections::HashMap;
extern crate rand;
use rand::Rng;

fn vector_demo() {
    // Iterators can be collected into vectors
    let collected_iterator: Vec<i32> = (0..10).collect();
    println!("Collected (0..10) into: {:?}", collected_iterator);

    // The `vec!` macro can be used to initialize a vector
    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    // Insert new element at the end of the vector
    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);

    // Error! Immutable vectors can't grow
    // collected_iterator.push(0);
    // FIXME ^ Comment out this line

    // The `len` method yields the current size of the vector
    println!("Vector size: {}", xs.len());

    // Indexing is done using the square brackets (indexing starts at 0)
    println!("Second element: {}", xs[1]);

    // `pop` removes the last element from the vector and returns it
    println!("Pop last element: {:?}", xs.pop());

    // Out of bounds indexing yields a panic
    println!("Fourth element: {}", xs[3]);
}

fn vector_test() {
    let mut semantic_vector1: [i32; 15] = [0; 15];
    let mut semantic_vector2: [i32; 15] = [1; 15];
    let mut semantic_vector3: [i32; 15] = [2; 15];
    let mut v: Vec<[i32; 15]> = Vec::new();
    v.push(semantic_vector1);
    v.push(semantic_vector2);
    v.push(semantic_vector3);

    println!("Vector size: {}", v.len());
    for vec in v.iter() {
        println!("{:?}", vec);
        for p in vec.iter() {
            print!("{}; ", p);
        }
        print!("\n");
    }

}

fn hashmap_test() {
    // <&str, Vec<[i32; 15]>>
    let mut storage = HashMap::new();

    let mut vecs1: Vec<[i32; 15]> = Vec::new();
    let mut vecs2: Vec<[i32; 15]> = Vec::new();
    let mut vecs3: Vec<[i32; 15]> = Vec::new();
    let mut vecs4: Vec<[i32; 15]> = Vec::new();

    let mut semantic_vector1: [i32; 15] = [0; 15];
    let mut semantic_vector2: [i32; 15] = [1; 15];
    let mut semantic_vector3: [i32; 15] = [2; 15];
    let mut semantic_vector4: [i32; 15] = [3; 15];
    let mut semantic_vector5: [i32; 15] = [4; 15];
    let mut semantic_vector6: [i32; 15] = [5; 15];

    vecs1.push(semantic_vector1);
    vecs1.push(semantic_vector2);

    vecs2.push(semantic_vector3);

    vecs3.push(semantic_vector4);
    vecs3.push(semantic_vector5);

    vecs4.push(semantic_vector6);

    storage.insert("01100011101", vecs1);
    storage.insert("01001011101", vecs2);
    storage.insert("01010011001", vecs3);
    storage.insert("11111111001", vecs4);

    for (key, value) in &storage {
        println!("{:?}: \"{:?}\"", key, value);
    };

}

fn projection() {

    let mut hash_size = 8;
    let mut input_dim = 15;
    let mut hyperplane = Vec::new();

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("{:?}", secret_number);

    // if rng.gen() { // random bool
    //     println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>());
    // }

    let mut i = 0;
    let mut j = 0;

    loop {
        let mut normal_vector = Vec::new();
        j = 0;
        loop {
            normal_vector.push(0);
            j += 1;
            if j >= input_dim {
                break;
            }
        }
        hyperplane.push(normal_vector);
        i += 1;
        if i >= hash_size {
            break;
        }
    }

    // 查看情况
    for vec in hyperplane.iter() {
        for p in vec.iter() {
            print!("{}, ", p);
        }
        print!("\n");
    }

    let input_vector = Vec::new();
    i = 0;
    loop {
        input_vector.push(0);
        i += 1;
        if i >= input_dim {
            break;
        }
    }


}

// fn _hash() -> &str {
//     let p = "test";
//     p
// }

fn main() {
    // vector_test();
    // hashmap_test();
    projection();
}
