use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_pair = (0, 0);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_pair = (i, i + 1);
        }
    }

    (min_sum, min_pair.0, min_pair.1)
}

fn display_result(data: &[i32]) {
    let (min_sum, idx1, idx2) = min_adjacent_sum(data);

    // Виведення індексів
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    print!("data:  [");
    for (i, &val) in data.iter().enumerate() {
        print!("{:>3}", val);
        if i < data.len() - 1 {
            print!(", ");
        }
    }
    println!("]");

    print!("indexes: ");
    for i in 0..data.len() {
        if i == idx1 {
            print!(" \\__");
        } else if i == idx2 {
            print!(" __/ ");
        } else {
            print!("     ");
        }
    }
    println!();

    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[idx1],
        data[idx2],
        min_sum,
        idx1,
        idx2
    );
    println!();
}

fn main() {
    let data = gen_random_vector(20);
    display_result(&data);
}