// fn main() {
//     let res = abbrev_name("Da Jiba");
//     println!("res is {}", res)
// }
// fn abbrev_name(name: &str) -> String {
//     name.split(' ')
//         .map(|x| x.chars().nth(0).unwrap().to_string().to_uppercase())
//         .collect::<Vec<_>>()
//         .join(".")
// }
#![feature(core_intrinsics)]
fn print_type_of<T>(_: T) {
    println!("{}", unsafe { std::intrinsics::type_name::<T>() });
}

fn main() {
    let res = "da jiba".split(' ');
    print_type_of(res);
}