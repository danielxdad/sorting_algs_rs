use std::{env, fmt::Debug, time::Instant};
use std::collections::BTreeMap;
use rand::prelude::*;

type SortFnPointer<'a> = &'a dyn Fn(Vec<u32>) -> Stats<u32>;

#[derive(Default)]
struct Stats<T> {
    cmp_counter: usize,
    swap_counter: usize,
    ordered: Vec<T>,
}

/* impl <T: Ord + Copy + Default> Stats<T> {
    fn default_with_ordered_as(v: Vec<T>) -> Self {
        let mut tmp = Stats::default();
        tmp.ordered = v;
        tmp
    }
} */

fn main() {
    let cap: u32 = env::args().nth(1).unwrap_or("1000".to_string()).parse().unwrap_or(1000);
    let alg_name = env::args().nth(2).unwrap_or("".to_string()).to_lowercase();
    // let arg_fixed = env::args().find(|x| x.to_lowercase().cmp(&"--fixed".to_string()) == Ordering::Equal).is_some();
    // let arg_reversed = env::args().find(|x| x.to_lowercase().cmp(&"--reverse".to_string()) == Ordering::Equal).is_some();

    let v = {
        let mut tmpv: Vec<u32> = vec![];
        for _ in 0..cap {
            tmpv.push(random::<u32>() % 1024);
        }
        tmpv
    };

    let mut v_sorted = v.clone();

    v_sorted.sort();

    assert!(v.len() > 1);

    if v.len() < 30 {
        println!("Unordered: {:?}", v);
        println!("Ordered:   {:?}\n", v_sorted);
    } else {
        println!("Length of v: {}\n", v.len());
    }

    let funcs: [(&str, SortFnPointer); 6] = [
        ("Bubble sort",     &bubble_sort),
        ("Selection sort",  &selection_sort),
        ("Insertion sort",  &insertion_sort),
        ("Merge sort",      &merge_sort),
        ("Quick sort",      &quick_sort),
        ("Radix sort",      &radix_sort)
    ];

    for (name, f) in funcs {
        if alg_name.len() > 0 && !name.to_lowercase().contains(&alg_name) {
            continue;
        }

        println!("{}:", name);
        
        let cloned = v.clone();
        let begin = Instant::now();
        let stat = f(cloned);
        let duration = Instant::now() - begin;

        if stat.ordered.len() < 50 {
            println!("\tOrdered: {:?}", stat.ordered);
        }

        println!("\tSwap counter: {}", stat.swap_counter);
        println!("\tComp counter: {}", stat.cmp_counter);
        println!("\tTotal ops:    {}", stat.cmp_counter + stat.swap_counter);
        println!("\tDuration:     {} μs", duration.as_micros());
        println!("");

        assert!(stat.ordered == v_sorted);
    }
}

fn bubble_sort<T: Ord + Debug + Copy>(mut v: Vec<T>) -> Stats<T> {
    let mut swap_counter = 0;
    let mut cmp_counter = 0;
   
    loop {
        let mut swapped = false;

        for i in 0..v.len()-1 {
            if v[i] > v[i+1] {
                [v[i], v[i+1]] = [v[i+1], v[i]];
                swapped = true;
                swap_counter += 1;
            }
            cmp_counter += 1;
        }

        if !swapped {
            break;
        }
    }

    Stats { ordered: v, cmp_counter, swap_counter }
}

fn selection_sort<T: Ord + Debug + Copy>(mut v: Vec<T>) -> Stats<T> {
    let mut swap_counter = 0;
    let mut cmp_counter = 0;

    for i in 0..v.len()-1 {
        let mut min_index = i;

        for j in i+1..v.len() {
            if v[j] < v[min_index] {
                min_index = j;
            }
            cmp_counter += 1;
        }

        if min_index > i {
            [v[i], v[min_index]] = [v[min_index], v[i]];
            swap_counter += 1;
        }
        cmp_counter += 1;
    }

    Stats { ordered: v, cmp_counter, swap_counter }
}

fn insertion_sort<T: Ord + Debug + Copy>(mut v: Vec<T>) -> Stats<T> {
    let mut swap_counter = 0;
    let mut cmp_counter = 0;

    for last_order_index in 0..v.len() - 1 {
        let mut insert_on = last_order_index + 1;
        let curr_unordered_val = v[insert_on];

        for ordered_index in (0..=last_order_index).rev() {
            cmp_counter += 1;
            if v[ordered_index] > curr_unordered_val {
                v[ordered_index + 1] = v[ordered_index];
                insert_on = ordered_index;
                swap_counter += 1;
            } else {
                break;
            }
        }

        v[insert_on] = curr_unordered_val;
    }

    Stats { ordered: v, cmp_counter, swap_counter }
}

fn merge_sort<T>(v: Vec<T>) -> Stats<T> where T: Ord + Debug + Copy {
    if v.len() < 2 {
        return Stats{ cmp_counter: 0, swap_counter: 0, ordered: v };
    }

    let middle = v.len() / 2;
    let mut left: Vec<T> = Vec::with_capacity(middle);
    let mut right: Vec<T> = Vec::with_capacity(middle + 1);
    let mut stats = Stats{ cmp_counter: 0, swap_counter: 0, ordered: vec![] };

    for i in 0..middle {
        left.push(v[i]);
    }

    for i in middle..v.len() {
        right.push(v[i]);
    }

    stats.swap_counter += v.len();

    // assert!(left.len() + right.len() == v.len(), "left={}, right={}, v={}", left.len(), right.len(), v.len());

    let stats_left = merge_sort(left);
    let stats_right = merge_sort(right);

    stats.cmp_counter += stats_left.cmp_counter + stats_right.cmp_counter;
    stats.swap_counter += stats_left.swap_counter + stats_right.swap_counter;

    left = stats_left.ordered;
    right = stats_right.ordered;

    if left[left.len() - 1] <= right[0] {
        left.append(&mut right);
        stats.ordered = left;
        return stats;
    }

    if left[0] >= right[right.len() - 1] {
        right.append(&mut left);
        stats.ordered = right;
        return stats;
    }

    // assert!(left.len() + right.len() == v.len(), "left={}, right={}, v={}", left.len(), right.len(), v.len());
    // assert!((right.len() - left.len()) < 2, "left={}, right={}", left.len(), right.len());
    
    let mut result: Vec<T> = Vec::with_capacity(left.len() + right.len());
    while left.len() > 0 && right.len() > 0 {
        // *right.drain(0..1).collect::<Vec<T>>().first().unwrap()
        if left[0] <= right[0] {
            result.push(left[0]);
            left.drain(0..1);
        } else {
            result.push(right[0]);
            right.drain(0..1);
        }
        stats.cmp_counter += 1;
        stats.swap_counter += 1;
    }

    if left.len() > 0 {
        stats.swap_counter += left.len();
        result.append(&mut left);
    }

    if right.len() > 0 {
        stats.swap_counter += right.len();
        result.append(&mut right);
    }

    // assert!(result.len() == v.len(), "result={}, v={}", result.len(), v.len());

    stats.ordered = result;
    stats
}

fn quick_sort<T>(mut v: Vec<T>) -> Stats<T> where T: Ord + Debug + Copy + Default {    
    if v.len() < 2 {
        return Stats { ordered: v, cmp_counter: 1, swap_counter: 0 };
    }

    let mut pivot_index = v.len() - 1;
    let mut index = 0;
    let mut stats = Stats { ordered: vec![], cmp_counter: 0, swap_counter: 0 };
    let pivot_value = v[pivot_index];

    while index < pivot_index {
        if v[index] > pivot_value {
            let tmp = v.remove(index);
            v.push(tmp);
            pivot_index -= 1;

            stats.swap_counter += 2;

        } else {
            index += 1;
        }

        stats.cmp_counter += 2;
    }

    let mut left = quick_sort(v[ 0..pivot_index ].to_vec());
    let mut right = quick_sort(v[ (pivot_index + 1)..v.len() ].to_vec());

    v.clear();
    v.append(&mut left.ordered);
    v.push(pivot_value);
    v.append(&mut right.ordered);

    stats.cmp_counter += left.cmp_counter;
    stats.swap_counter += left.swap_counter;
    stats.ordered = v;

    stats
}

fn radix_sort(mut v: Vec<u32>) -> Stats<u32> { // where T: Ord + Debug + Copy + Default + std::ops::Div<u32> + std::ops::Rem<u32>
    if v.len() < 2 {
        return Stats { ordered: v, cmp_counter: 1, swap_counter: 0 };
    }

    let mut stats = Stats { ordered: vec![], cmp_counter: 0, swap_counter: 0 };
    let mut bucket: BTreeMap<_, Vec<u32>> = (0..10).map(|n| (n, vec![])).collect();

    for dp in 0..(u32::MAX as f64).log10().ceil() as u32 {
        let div = 10_u32.pow(dp);

        for i in 0..v.len() {
            let key = v[i] / div % 10;
            bucket.entry(key)
                .and_modify(|e| e.push(v[i]))
                .or_insert(vec![]);

            stats.swap_counter += 4;
        }

        v.clear();

        bucket.iter_mut().for_each(|(_, e)| {
            v.append(e);
            stats.swap_counter += 1;
        });
    }

    stats.ordered = v;
    stats
}
