use std::thread;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut nums: Vec<i32> = vec![];
    args.remove(0);
    for n in args {
        nums.push(n.parse::<i32>().unwrap());
    }
    
    println!("{:#?}", mergeSort(nums));
}


fn mergeSort(list: Vec<i32>) -> Vec<i32> {
    if list.len() <= 1 {//base case
        return list;
    }
    //sort around pivot in linear time
    let mut lhs: Vec<i32> = vec![];
    let mut rhs: Vec<i32> = vec![];

    for n in 1..list.len() {
        if list[n] < list[0] {
            lhs.push(list[n]);
        } else {
            rhs.push(list[n]);
        }
    }
    lhs.push(list[0]);

    //x = spawn Mergesort(lhs)
    let join_handle: thread::JoinHandle<Vec<i32>> = thread::spawn(move || -> Vec<i32> {
        return mergeSort(lhs);
    });

    //y = Mergesort(rhs)
    let mut y = mergeSort(rhs);
    //await
    let mut x: Vec<i32> = join_handle.join().expect("Thread join failure!!!!");

    //return merge(x, y);
    x.append(&mut y);

    x
}
