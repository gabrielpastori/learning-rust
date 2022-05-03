fn main() {
    println!("On the first day of Christmas,\nmy true love sent to me\nA partridge in a pear tree.\n");

    let  ordinals = ["second", "third", "fourth", "fifth", "sixth", "seventh", "eighth",
    "ninth", "tenth", "eleventh", "twelfth"];
    let lines = ["Two turtle doves,", "Three French hens,", "Four calling birds,", 
    "Five golden rings,", "Six geese a-laying,", "Seven swans a-swimming,", "Eight maids a-milking,",
    "Nine ladies dancing,", "Ten lords a-leaping,", "Eleven pipers piping,", "Twelve drummers drumming,"];

    for order in 0..11 {
        println!("On the {} day of Christmas,", ordinals[order]); 
        println!("my true love sent to me");

        for line_idx in (0..order+1).rev() {
            println!("{}",lines[line_idx as usize]);
        }
        
        println!("And a partridge in a pear tree.\n");

    }
    
}
