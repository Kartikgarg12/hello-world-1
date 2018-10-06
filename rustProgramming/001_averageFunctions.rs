use std::io::stdin;

fn main() {
    printNumberTo(10);
    println!("{}", isEven(31));
}
//Works
fn isEven(num: i32) -> bool{
    return num % 2 ==0;
}

//Get average in int form
fn get_mean_int(arr: &mut [i32]) -> i32{
    let mut total_num: i32 = 0;
    
    for i in 0..arr.len(){
        total_num += arr[i];
    }
    return total_num / arr.len() as i32;
}

fn get_mean_float(arr: &mut [f64]) -> f64{
    let mut total_num: f64 = 0.0;
    
    for i in 0..arr.len(){
        total_num += arr[i];
    }
    return total_num / arr.len() as f64;
}