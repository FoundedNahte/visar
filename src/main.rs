extern crate vis-core;
extern crate vis-macros;

use crate::vis_macros::initalize;
use vis_array;
use rand::{thread_rng, Rng};

fn create_rand_array(size: u16) -> Vec<u16> {
    let mut arr = Vec::<u16>::new();
    for i in 0..size {
        arr.push((i as u16) + 1);
    }
    let mut rng = thread_rng();
    for i in 0..size {
        let s: usize = rng.gen_range(0..size-1).into();
        let temp = arr[s];
        arr[s] = arr[i as usize];
        arr[i as usize] = temp;
    }
    
    arr
}

fn selection_sort(arr: &mut [u16], vis_arr: &mut vis_array::VisualArray) {
    let mut min_idx;
    for i in 0..arr.len() {
        min_idx = i;
        for j in (i+1)..arr.len() {
            if arr[min_idx] > arr[j] {
                min_idx = j;
            }
        }
        let temp = arr[min_idx];
        arr[min_idx] = arr[i];
        arr[i] = temp;
        //arr.swap(min_idx as u32, i as u32);
    }
}

fn main() {
}