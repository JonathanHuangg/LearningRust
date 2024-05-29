// lifetime annotaiton syntax
/*

&i32 -> reference
&'a i32 -> reference with explicit lifetime
&'a mut i32 -> mutable reference with explicit lifetime

*/

fn main() {

    // this won't compile because r's reference, x, has gone out of scope
    /*
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r)
    */

    // scenerio 1
    let String1 = String::from("abcd");
    let String2 = "xyz"; // this is usually for read only, it's a string slice
    // you want longest to take string slices because you don't want it to take ownership

    let result = longest(String1.as_str(), String2);

    println!("Longest string: {}", result);

    // scenerio 2, this runs because we explicitly defined lifetimes

    let string1 = String::from("long string is lone");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string in scenerio 2 is: {}", result)
    }
}

// this means that all of these have the same lifetime and will get removed together
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}