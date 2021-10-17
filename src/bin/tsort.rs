/*
https://www.codechef.com/problems/TSORT
 */

use std::io;

fn partition(arr: &mut Vec<i32>, low: i32, high: i32) -> i32 {
    let pivot = arr[high as usize];

    let mut i = low - 1;

    for j in low..=high {
        if arr[j as usize] < pivot {
            i += 1;
            swap(arr, i, j);
        }
    }

    swap(arr, i + 1, high);
    return i + 1;
}

fn swap(arr: &mut Vec<i32>, x: i32, y: i32) {
    let temp = arr[x as usize];
    arr[x as usize] = arr[y as usize];
    arr[y as usize] = temp;
}

fn quick_sort(arr: &mut Vec<i32>, low: i32, high: i32) {
    if low < high {
        let pi = partition(arr, low, high);
        quick_sort(arr, low, pi - 1);
        quick_sort(arr, pi + 1, high);
    }
}

fn main() {
    let mut input_buffer = String::new();
    let mut a = Vec::new();
    io::stdin().read_line(&mut input_buffer).unwrap();

    let item_count = input_buffer.trim().parse::<i32>().unwrap();

    for _i in 0..item_count {
        input_buffer.clear();
        io::stdin().read_line(&mut input_buffer).unwrap();
        let number = input_buffer.trim().parse::<i32>().unwrap();
        a.push(number);
    }

    quick_sort(&mut a, 0, item_count - 1);

    for i in a {
        println!("{}", i);
    }
}
