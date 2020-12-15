use rand::Rng;

fn main() {
    let mut int_list = generate_int_list();

    let length = int_list.len();

    let avg = int_list_average(&int_list, &length);

    println!("{:?}", length);
    println!("{:?}", avg);
    int_list = sort_int_list(int_list);

    println!("{:?}", int_list);
    println!("{:?}", int_list[length / 2]);
}

fn generate_int_list() -> Vec<isize> {
    let mut rng = rand::thread_rng();
    let int_list: Vec<isize> = (1..rng.gen_range(10, 100))
        .map(|_| rng.gen_range(-100, 100))
        .collect();
    int_list
}

fn int_list_average(int_list: &Vec<isize>, length: &usize) -> f64 {
    int_list[..].into_iter().sum::<isize>() as f64 / *length as f64
}

fn sort_int_list(mut int_list: Vec<isize>) -> Vec<isize> {
    int_list.sort();
    int_list
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;
    #[test]
    fn average_of_five_fives_is_five() {
        assert_ulps_eq!(int_list_average(&vec![5; 5], &5), 5.0);
    }
    #[test]
    fn average_of_one_to_six_is_three_and_a_half() {
        assert_relative_eq!(int_list_average(&vec![1, 2, 3, 4, 5, 6], &6), 3.5);
    }
    #[test]
    fn median_of_five_numbers_is_at_position_three {
    }
}
