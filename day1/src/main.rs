mod first_part {
    pub fn solve(input: &str) {
        let mut calories = 0;
        let mut calories_max = usize::MIN;

        for line in input.lines() {
            if let Ok(calorie) = str::parse::<usize>(line) {
                calories += calorie;
            } else {
                calories_max = calories.max(calories_max);
                calories = 0;
            }
        }

        dbg!(calories_max);
    }
}

mod second_part {
    // Number of top calories.
    const N: usize = 3;

    // This could be simpler, but I just wanted to see if I can make the solution
    // as general as possible for any size N :)
    pub fn solve(input: &str) {
        let mut calories = 0;
        let mut top_n = [usize::MIN; N];

        for line in input.lines() {
            if let Ok(calorie) = str::parse::<usize>(line) {
                calories += calorie;
            } else {
                // Binary search returns Err(index) if the given item is not
                // found, but the item can be placed to that index while
                // maintaining the sort order. 
                if let Err(index) = top_n.binary_search_by(|c| calories.cmp(c)) {
                    // Since binary search may suggest inserting to the end of
                    // the array, `index` can be N which is out of bounds.
                    if index < N {
                        // Starting from `index`, shift elements by one.
                        top_n.copy_within(index..N - 1, index + 1);
                        top_n[index] = calories;
                    }
                }

                calories = 0;
            }
        }

        let sum: usize = top_n.iter().sum();
        dbg!(sum);
    }
}

fn main() {
    let input = include_str!("../input.txt");

    first_part::solve(input);
    second_part::solve(input);
}
