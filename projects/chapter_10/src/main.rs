fn find_largest<T>(number_list: &[T]) -> &T
where
    T: std::cmp::PartialOrd,
{
    // Find the largest number in a list
    let mut largest = &number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn main() {
    let number_list = vec![10, 8, 7, 89, 1, 10];
    let largest = find_largest(&number_list);
    println!("The largest number is : {largest}");

    let char_list = vec!['a', 'x', 'z', 'A'];
    let largest = find_largest(&char_list);
    println!("The largest char is: {largest}");

    let string1 = String::from("abcd");
    let string2 = "xyzdghsgfjs";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

pub fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s2.len() > s1.len() {
        s2
    } else {
        s1
    }
}
