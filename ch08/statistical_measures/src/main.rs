use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum Measure {
    Mean(f32),
    Median(f32),
    Mode(Vec<i32>),
}
fn main() {
    println!("Hello, world!");

    let list_length = get_i32_input("Type the length of the number list:");
    let result = get_measures(list_length);
    dbg!(result);
}

fn get_i32_input(message: &str) -> i32 {
    loop {
        let mut input_buffer = String::new();

        println!("{}", message);

        io::stdin()
            .read_line(&mut input_buffer)
            .expect("Failed to read line");

        if let Ok(value) = input_buffer.trim().parse::<i32>() {
            return value;
        }

        println!("Type a valid i32 value!");
    }
}

fn get_vector_value(values_vector: &[i32], index: i32) -> f32 {
    if let Some(value) = values_vector.get(index as usize) {
        (*value) as f32
    } else {
        0f32
    }
}

fn get_measures(list_length: i32) -> Vec<Measure> {
    if list_length == 0 {
        return Vec::new();
    }

    let mut result_vector: Vec<Measure> = Vec::new();
    let mut values_vector: Vec<i32> = Vec::new();
    let mut modes_vector: Vec<i32> = Vec::new();
    let mut sum = 0;
    let mut count_frequency = HashMap::new();
    let mut max_key;
    let mut max_value = 0;

    for _iterator in 0..list_length {
        let vec_value = get_i32_input("Type a value:");
        values_vector.push(vec_value);

        sum += vec_value;
        let count = count_frequency.entry(vec_value).or_insert(0);
        *count += 1;

        if max_value < *count {
            modes_vector.clear();
        }

        if max_value <= *count {
            max_value = *count;
            max_key = vec_value;
            modes_vector.push(max_key);
        }
    }

    modes_vector.sort_unstable(); // Sorts the slice, but might not preserve the order of equal elements
    values_vector.sort_unstable(); 
    let mean: f32 = sum as f32 / list_length as f32;
    let median: f32 = if list_length % 2 != 0 {
        get_vector_value(&values_vector, (list_length + 1) / 2 - 1)
    } else {
        (get_vector_value(&values_vector, list_length / 2 - 1)
            + get_vector_value(&values_vector, list_length / 2)) as f32
            / 2.0
    };

    
    result_vector.push(Measure::Mean(mean));
    result_vector.push(Measure::Median(median));
    result_vector.push(Measure::Mode(modes_vector));

    result_vector
}
