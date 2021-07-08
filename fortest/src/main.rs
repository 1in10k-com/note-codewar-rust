fn main() {
    println!("hello moto");
    let realarr = vec![1,3,4,7];
    first_non_consecutive(&realarr);
}

fn first_non_consecutive(arr: &Vec<i32>) -> Option<i32> {
    let mut older_i:&i32 = &1;
    
    for i in arr {
        older_i = i;
        println!("older_i is {}", i);
        println!("i is {}", i);
        
    }
    Some(1)
}
