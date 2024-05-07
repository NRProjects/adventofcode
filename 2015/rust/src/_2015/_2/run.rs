// --- Day 2: I Was Told There Would Be No Math ---
// The elves are running low on wrapping paper, and so they need to submit an order for more. They have a list of the dimensions (length l, width w, and height h) of each present, and only want to order exactly as much as they need.
//
// Fortunately, every present is a box (a perfect right rectangular prism), which makes calculating the required wrapping paper for each gift a little easier: find the surface area of the box, which is 2*l*w + 2*w*h + 2*h*l. The elves also need a little extra paper for each present: the area of the smallest side.
//
// For example:
//
// A present with dimensions 2x3x4 requires 2*6 + 2*12 + 2*8 = 52 square feet of wrapping paper plus 6 square feet of slack, for a total of 58 square feet.
// A present with dimensions 1x1x10 requires 2*1 + 2*10 + 2*10 = 42 square feet of wrapping paper plus 1 square foot of slack, for a total of 43 square feet.
// All numbers in the elves' list are in feet. How many total square feet of wrapping paper should they order?

// --- Part Two ---
// The elves are also running low on ribbon. Ribbon is all the same width, so they only have to worry about the length they need to order, which they would again like to be exact.
//
// The ribbon required to wrap a present is the shortest distance around its sides, or the smallest perimeter of any one face. Each present also requires a bow made out of ribbon as well; the feet of ribbon required for the perfect bow is equal to the cubic feet of volume of the present. Don't ask how they tie the bow, though; they'll never tell.
//
// For example:
//
// A present with dimensions 2x3x4 requires 2+2+3+3 = 10 feet of ribbon to wrap the present plus 2*3*4 = 24 feet of ribbon for the bow, for a total of 34 feet.
// A present with dimensions 1x1x10 requires 1+1+1+1 = 4 feet of ribbon to wrap the present plus 1*1*10 = 10 feet of ribbon for the bow, for a total of 14 feet.
// How many total feet of ribbon should they order?

use std::cmp::min;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() {
    let mut path = env::current_dir().unwrap();
    path.push("src\\_2015\\_2\\input.txt");

    let file = File::open(path).unwrap();

    let reader = BufReader::new(file);

    let mut total_wrapping_paper = 0;
    let mut total_bow_amount = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let dimensions = parse_dimensions(&line);

        total_wrapping_paper += calculate_area(&dimensions);
        total_bow_amount += calculate_ribbon_amount(&dimensions);
    }

    println!("Day Two | Total Wrapping Paper: {} | Total Bow Amount: {}", total_wrapping_paper, total_bow_amount);
    return;
}

fn parse_dimensions(input: &str) -> (i32, i32, i32) {
    let parts: Vec<i32> = input.split('x').map(|num| num.parse().unwrap()).collect();
    (parts[0], parts[1], parts[2])
}

fn calculate_area(dimensions: &(i32, i32, i32)) -> i32 {
    let (length, width, height) = dimensions;

    let area1 = length * width;
    let area2 = width * height;
    let area3 = height * length;

    let smallest_area = min(area1, min(area2, area3));
    2 * (area1 + area2 + area3) + smallest_area
}

fn calculate_ribbon_amount(dimensions: &(i32, i32, i32)) -> i32 {
    let (length, width, height) = dimensions;

    let bow_amount = length * width * height;
    let ribbon_amount = 2 * (min(min(length + width, width + height), height + length));

    return bow_amount + ribbon_amount
}