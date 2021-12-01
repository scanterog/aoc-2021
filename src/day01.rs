use std::fs;

fn main() {
    let measurements = get_measurements("inputs/day01.txt");
    println!("number of increases (part1): {}", count_increases(&measurements));
    println!("number of increases (part2): {}", count_increases_sliding_window(&measurements));
}

fn get_measurements(input: &str) -> Vec<i32> {
    let content = fs::read_to_string(input).expect("error reading input file");
    let measurements: Vec<i32> = content.split("\n").map(|x| x.parse::<i32>().unwrap()).collect();
    measurements
}

fn count_increases(measurements: &Vec<i32>) -> usize {
    measurements.windows(2).filter(|&x| x[1] > x[0]).count()
}

fn count_increases_sliding_window(measurements: &Vec<i32>) -> usize {
    let mut m_iter = measurements.windows(3);
    let mut prev_win: i32 = m_iter.next().unwrap().iter().copied().sum();
    let mut count = 0;
    for cur in m_iter {
        let cur_win: i32 = cur.iter().copied().sum();
        if cur_win > prev_win {
            count += 1
        }
        prev_win = cur_win
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let m = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let val = count_increases(&m);
        assert_eq!(val, 7)

    }

    #[test]
    fn test_part2() {
        let m = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let val = count_increases_sliding_window(&m);
        assert_eq!(val, 5)

    }
}