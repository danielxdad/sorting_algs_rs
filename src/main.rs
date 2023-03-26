use core::arch::x86_64::_rdseed16_step;
use std::{env, cmp::Ordering, fmt::Debug};

struct Stats<T> {
    cmp_counter: u32,
    swap_counter: u32,
    ordered: Vec<T>,
}

fn main() {
    let arg_fixed = env::args().find(|x| x.to_lowercase().cmp(&"--fixed".to_string()) == Ordering::Equal).is_some();
    let arg_reversed = env::args().find(|x| x.to_lowercase().cmp(&"--reverse".to_string()) == Ordering::Equal).is_some();
    let cap = 20;
    
    let v = {
        let mut tmpv: Vec<u16> = vec![];
        if arg_fixed {
            if arg_reversed {
                tmpv = (1..(cap + 1 as u16)).into_iter().rev().collect();
            } else {
                tmpv = (1..(cap + 1 as u16)).into_iter().collect();
            }
        } else {
            let mut n = 0;
            for _ in 0..cap {
                unsafe {
                    while _rdseed16_step(&mut n) == 0 {
                        std::hint::spin_loop();
                    }
                }
                tmpv.push((n % cap as u16) + 1);
            }
        }
        tmpv
    };

    let mut v_sorted = v.clone();
    v_sorted.sort();

    assert!(v.len() > 1);
    println!("Unordered: {:?}", v);
    println!("Ordered:   {:?}\n", v_sorted);

    for f in [bubble_sort, selection_sort, insertion_sort] {
        let stat = f(v.clone());

        println!("\tOrdered: {:?}", stat.ordered);
        println!("\tSwap counter: {:?}", stat.swap_counter);
        println!("\tComp counter: {:?}", stat.cmp_counter);
        println!("\tTotal ops: {:?}", stat.cmp_counter + stat.swap_counter);
        println!("");

        assert!(stat.ordered == v_sorted);
    }

}

fn bubble_sort<T: Ord + Debug + Copy>(mut v: Vec<T>) -> Stats<T> {
    let mut swap_counter = 0;
    let mut cmp_counter = 0;
   
    println!("Bubble sort");

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

    println!("Selection sort");

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

fn insertion_sort<T: Ord + Debug + Copy>(mut v: Vec<T>) -> Stats<T>  {
    let mut swap_counter = 0;
    let mut cmp_counter = 0;

    println!("Insertion sort");

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
