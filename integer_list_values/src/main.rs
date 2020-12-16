use rand::Rng;
use std::collections::HashMap;

fn main() {
    let list = generate();
    let average = average(&list);
    let median = median(&list);

    println!("The list is {:?}", list);
    println!("The length of the list is {}", list.len());
    println!("The average of the list is {}", average);
    println!("The sorted list is {:?}", sort(&list));
    println!("The median of the sorted list is {}", median);
    println!("The mode of the list is {:?}", mode(&list));
}

fn generate() -> Vec<isize> {
    let mut rng = rand::thread_rng();
    let list: Vec<isize> = (1..rng.gen_range(10, 100))
        .map(|_| rng.gen_range(-100, 100))
        .collect();
    list
}

fn average(list: &Vec<isize>) -> f64 {
    list[..].into_iter().sum::<isize>() as f64 / list.len() as f64
}

fn sort(list: &Vec<isize>) -> Vec<isize> {
    let mut sorted_list = list.clone();
    sorted_list.sort();
    sorted_list
}

fn median(list: &Vec<isize>) -> isize {
    let mut sorted_list = list.clone();
    sorted_list.sort();
    sorted_list[list.len() / 2]
}

fn mode(list: &Vec<isize>) -> (isize, isize) {
    let mut hmap = HashMap::new();
    let mut max = 0;
    let mut item = 0isize;
    for element in list {
        let current = match hmap.get_key_value(element) {
            Some((&key,&value)) => {
                hmap.insert(key, value + 1).expect("Unable to update key")
            },
            None => {hmap.insert(element, 1); 0},
        } + 1;
        if current > max {
            max = current;
            item = *element;
        }
    }
    (item, max)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;
    #[test]
    fn average_of_five_fives_is_five() {
        assert_ulps_eq!(average(&vec![5; 5]), 5.0);
    }
    #[test]
    fn average_of_one_to_six_is_three_and_a_half() {
        assert_relative_eq!(average(&vec![1, 2, 3, 4, 5, 6]), 3.5);
    }
    #[test]
    fn median_of_five_numbers_is_at_position_three() {
        assert_eq!(median(&vec![1, 2, 3, 4, 5]), 3);
    }

    #[test]
    fn mode_of_all_fives_is_five() {
        assert_eq!(mode(&vec![5,5,5,5,5]), (5, 5));
    }

    #[test]
    fn mode_of_all_but_one_fives_is_four() {
        assert_eq!(mode(&vec![5,5,2,5,5]), (5, 4));
    }

    #[test]
    fn mode_of_none_but_one_fives_is_one() {
        assert_eq!(mode(&vec![5,4,3,2,1]), (5, 1));
    }
}
