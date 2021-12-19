use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::Ordering;

// Pour tester la comparaison built_in vs le if
use std::time::Instant;

fn main() {
    println!("Advent of Rust - Day 1");

    if let Ok(lines_as_iterator) = read_lines("input.txt") {
        let lines_as_vector = create_vector_from_iterator(lines_as_iterator);
        
        // Partie 1
        // Pour le fun on compare les performances de la comparaison par if avec la built-in
        compare_with_built_in(&lines_as_vector);
        compare_with_if(&lines_as_vector);

        // Partie 2
        let mut previous_value = i32::MAX;
        let mut counter = 0;
        let last_index = lines_as_vector.len() - 2; // On utilise des triplets donc on va jusqu'à n-2 pour trouver le dernier

        for index in 0..last_index {
            let triplet_sum: i32 = lines_as_vector[index..index+3].iter().sum();
            
            if triplet_sum > previous_value {
                counter = counter + 1
            }
            previous_value = triplet_sum
        }
        println!("Found {} increases", counter);

    }
    else {
        println!("Error - cannot read file")
    }
}

fn create_vector_from_iterator(lines: io::Lines<io::BufReader<File>>) -> Vec<i32> {
    let mut vector: Vec<i32> = Vec::new();

    for line in lines {
        if let Ok(value) = line {
            let value: i32 = value.parse().expect("Error - Cannot parse value to integer");
            vector.push(value)
        }
    }
    vector
}

fn compare_with_built_in(values: &Vec<i32>) {
    let mut previous_value = i32::MAX;
    let mut counter = 0; 
    let now = Instant::now();
    
    for value in values {
        // Utilisation de la fonction de comparaison built-in
        // Ok c'est un peu overkill mais c'est pour apprendre
        match value.cmp(&previous_value) {
            Ordering::Greater => counter = counter + 1,
            _ => (),
        }
        previous_value = *value;
    }
    println!("Found {} increases in {}us using built-in comparaison", counter, now.elapsed().as_micros());
}

fn compare_with_if(values: &Vec<i32>) {
    let mut previous_value = i32::MAX;
    let mut counter = 0; 
    let now = Instant::now();

    for value in values {
        if *value > previous_value {
            counter = counter + 1;
        }
        previous_value = *value;
    }
    println!("Found {} increases in {}us using if comparaison", counter, now.elapsed().as_micros());
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}