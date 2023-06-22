fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    if let Some(third) = v.get(2) {
        println!("the third is {third}");
    } else {
        println!("there is no third number")
    }

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);
}
