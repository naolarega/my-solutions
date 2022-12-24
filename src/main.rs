mod leetcode;
mod codeforces;
mod hacker_rank;

fn main() {
    // println!("{:?}", leetcode::fizzbuzz::Solution::fizz_buzz(12));

    // println!("{:?}", hacker_rank::grading_students::Solution::grading_students(&[73, 67, 38, 33]))

    // codeforces::domino_piling::Solution::domino_piling();

    // hacker_rank::bubble_sort::Solution::bubble_sort();

    // hacker_rank::insertion_sort::Solution::insertion_sort();

    // hacker_rank::counting_sort::Solution::counting_sort();

    println!("{}", leetcode::sorting_the_sentence::Solution::sort_sentence("Myself2 Me1 I4 and3".to_string()));
}
