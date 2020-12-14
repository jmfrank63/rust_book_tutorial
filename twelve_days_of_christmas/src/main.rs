fn main() {
    let cardinals: [&str; 12] = [
        "", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven",
        "twelve",
    ];
    let ordinals: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "nineth",
        "tenth", "eleventh", "twelfth",
    ];
    let presents: [&str; 12] = [
        "and a partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "gold rings",
        "geese a laying",
        "swans a swimming",
        "maids a milking",
        "ladies dancing",
        "lords a leaping",
        "pipers piping",
        "drummers drumming",
    ];
    let intro = "On the  day of Christmas my true love gave to me";

    println!("Twelve Days of Christmas");
    println!("John Denver, The Muppets");

    let mut all_presents = String::from("");
    for (count, ordinal) in ordinals.iter().enumerate() {
        println!("{}{}{}", &intro[..7], ordinal, &intro[7..]);
        all_presents =
            format!("{} {}, {}", cardinals[count], presents[count], all_presents).to_owned();
        all_presents = all_presents.replace(",  ", " ");
        if count == 0 {
            all_presents = all_presents.replace(" and ", "").replace(",", ".");
        } else {
            all_presents = all_presents.replace("doves, a", "doves and");
        }
        println!("{}", all_presents);
    }
}
