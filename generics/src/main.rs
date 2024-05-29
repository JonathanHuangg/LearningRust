struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Point {x, y}
    }

    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<T, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// constrained point

impl Point<f64, f64> {
    fn dist_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}


fn main() {
    // normal function that finds largest number in a list

    let number_list = vec![1, 30, 29, 36, 43, 21, 4, 5];

    let mut largest = &number_list[0];

    for number in &number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("Part 1: The largest number is {}", largest);

    // issue is that if you have another array, you need to duplicate this code to do it again. Using fn function:

    let number_list = vec![5, 33, 29, 36, 43, 22, 4, 5];
    let result = largest_func(&number_list);
    println!("Part 1: The largest number is {}", result);

    // Following generics with points

    let integer = Point{x: 5, y: 10};
    let float = Point{x: 1.0, y: 4.0};

    // Use the impl from the enum

    let point1 = Point::new("xCoord", 5);

    println!("point1.x = {}, point1.y = {}", point1.x(), point1.y());

}

fn largest_func(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/*
// literally the only differnece is the signature, remember to restrict to ordinal because you are comparing
fn largest_generic_func<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest= item
        }
    }

    largest
}
*/

