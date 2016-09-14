#[allow(dead_code)]
#[allow(unused_mut)]

extern crate rand;
use self::rand::distributions::normal::StandardNormal;
use std::collections::HashMap;
use self::rand::Rng;

#[allow(dead_code)]
pub fn vector_demo() {
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

#[allow(dead_code)]
#[allow(unused_mut)]
pub fn vector_test() {
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

#[allow(dead_code)]
#[allow(unused_mut)]
pub fn hashmap_test() {
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


// rust 生成随机数
#[allow(dead_code)]
#[allow(unused_mut)]
pub fn gen_random_value() {
    let mut rng = rand::thread_rng();
    let mut random_number = rng.gen_range(0, 100);
    println!("random number: {:?}", random_number);

}

#[allow(dead_code)]
#[allow(unused_mut)]
pub fn show_2d_vectors( hyperplane: &mut Vec<Vec<f64>>) {

    for vec in hyperplane.iter() {
        print!("[");
        for p in vec.iter() {
            print!("{:.*}, ", 2, p);
        }
        print!("]\n");
    }
}

#[allow(dead_code)]
pub fn fixed_hyperplane() -> Vec<Vec<f64>> {
    let hyperplane = vec![
    vec![0.94, -0.31, 0.80, 0.25, 0.54, -1.67, 0.60, 1.62, -0.13, -0.91, 0.42, -1.57, 0.75, -1.35, -1.16],
    vec![-0.05, -0.66, 0.98, 0.83, -0.58, 1.71, -0.65, -0.27, 1.35, 0.03, 0.69, -0.59, 1.34, 0.48, 0.67],
    vec![-1.02, 0.97, -1.25, 0.26, 0.46, -1.97, -0.20, 2.31, 0.26, -2.01, 0.72, -0.49, 0.20, -0.93, -0.49],
    vec![0.20, -0.08, 0.58, -2.20, 0.42, 0.12, -0.70, 1.12, -0.70, -0.21, -0.37, 0.06, -0.99, 0.66, 1.39],
    vec![-2.05, 1.22, -0.35, 1.29, 1.20, -0.07, 0.12, 1.78, -0.73, -0.39, -0.84, 0.15, 0.50, -0.07, 3.53],
    vec![0.16, -0.06, 0.03, -0.64, 0.55, 1.09, 0.53, -0.50, 3.20, 0.16, -0.55, -1.30, -0.80, 1.12, -1.97],
    vec![-0.63, -1.62, 2.08, -0.47, -0.37, 2.75, -1.65, -0.16, 1.23, -0.99, -1.17, -1.27, 0.40, 0.49, -1.13],
    vec![0.24, 0.86, 0.45, 1.24, -0.86, 1.01, -1.74, 2.22, -0.63, -0.40, 0.65, -0.82, 0.31, 0.50, -0.14],
    ];
    hyperplane
}

#[allow(unused_mut)]
pub fn _hash(input_vector: &mut Vec<f64>, hyperplane: &mut Vec<Vec<f64>> ) -> String {

    let mut projections = Vec::new();
    let mut len = input_vector.len();
    let mut sum:f64 = 0.0;



    for v in hyperplane.iter() {
        for i in 0..len {
            sum += input_vector[i] * v[i];
        }
        projections.push(sum);
        sum = 0.0;
    }
    println!("{:?}", projections);

    let mut hash_code:String = "".to_string();
    for p in projections.iter() {
        if p > &0.0 {
            hash_code = hash_code + "1";
        } else {
            hash_code = hash_code + "0";
        }
    };
    hash_code
}

#[allow(dead_code)]
pub fn test_hash() {
    let mut hyperplane = vec![
    vec![0.23020272, 0.31947747, 0.63655935, -0.91020602],
    vec![0.00846898, 0.2040465, -0.05346568, 0.82772385],
    vec![0.00846898, 0.2040465, -0.05346568, 0.82772385],
    vec![0.23020272, 0.31947747, 0.63655935, -0.91020602],
    vec![0.00846898, 0.2040465, -0.05346568, 0.82772385],
    vec![0.23020272, 0.31947747, 0.63655935, -0.91020602],
    ];

    let mut input_vector = vec![-0.19064272, 0.77854038, -0.78337315, 1.20084995];

    let mut projections = _hash(&mut input_vector, &mut hyperplane);
    println!("{:?}", projections);

}

#[allow(dead_code)]
#[allow(unused_mut)]
#[allow()]
pub fn projection() {

    let mut hash_size = 8;
    let mut input_dim = 15;
    let mut hyperplane = Vec::new();

    let mut i = 0;
    let mut j;
    loop {
        let mut normal_vector = Vec::new();
        j = 0;
        loop {
            let StandardNormal(standard_normal_value) = rand::random();
            normal_vector.push(standard_normal_value);
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

    println!("-------------------------------");

    show_2d_vectors(&mut hyperplane);

    let mut input_vector = Vec::new();
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

pub fn main() {
    // vector_test();
    // hashmap_test();
    // projection();
    test_hash()
    // gen_random_value();
}
