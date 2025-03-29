pub fn map<F, T, R>(input: Vec<T>, mut _function: F) -> Vec<R>
where
    F: FnMut(T) -> R,
{
    //version that uses a loop to build a Vec<R>
    let mut v = Vec::new();
    for i in input {
        v.push(_function(i));
    }
    v
    // Shorter version thar collects to a Vec<R>
    //input.into_iter().map(_function).collect()
}

// println!("[]: {:?}", map(vec![2.0, 3.0, 4.0, 5.0], |x| x * x)); //"1".to_string(), "2".into(), "3".into()
// println!(
//     "[]: {:?}",
//     map(vec!["1".to_string(), "2".into(), "3".into()], |x| x
//         .repeat(2))
// );

// let input = vec!["Hello", "world"];
// let expected = vec!["HELLO", "WORLD"];
// println!("{:?}, {:?}", map(input, str::to_uppercase), expected);

// let mut counter = 0;
// let input = vec![-2, 3, 4, -5];
// let expected = vec![2, 3, 4, 5];
// let result = map(input, |x: i64| {
//     counter += 1;
//     x.abs()
// });
// println!("{:?}, {:?}", result, expected);
