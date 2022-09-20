fn main() {
    let mut years : [i32; 3] = [1996, 2001, 2025];

    let first = years[0];
    let [ _, second, third] = years;
    years[1] = 3001;

    println!("{}", first);
    println!("{}", second);
    println!("{}", third);

    for year in years.iter() {
        println!("Next year {}", year + 1);
    }

    let years_tuples : (i32, i32, bool ) = (2021, 2023, true);

    // for year in years_tuples.iter() {
    //     println!("Next year {}", year);
    // }

}