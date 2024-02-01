fn main() {
    let days = ["first", "second", "third", "fourth", "fifth"];
    let animals = ["A partridge in a pear tree", "Two turtle doves", "Three french hens", "Four calling birds", "Five gold rings"];
    //for loop accessing index in days and length
    for i in 0..days.len() {
        //printing the first sentence 
        println!("On the {} day of Christmas my true love sent to me",days[i]);
        for j in (0..=i).rev() {
            //accessing the j for equal to i
            println!("{} ", animals[j])
        }
        println!("");
    }
}

