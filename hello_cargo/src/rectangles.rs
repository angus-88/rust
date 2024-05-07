#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

pub fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => println!("Rolled a 3"),
        7 => println!("Rolled a 7"),
        other => println!("Rolled {other}, try again."),
    }

    let config_max = Some(3);
    if let Some(max) = config_max {
        println!("configMax is populated");
    }

    enum Tractor {
        CaseIh,
        JohnDeer,
        Fendt,
    }
    let tractor = Tractor::CaseIh;
    if let Tractor::CaseIh = tractor {
        println!("Great choice")
    } else {
        println!("Bad choice of tractor")
    }

    let width1 = 30;
    let height1 = 50;
    let rect1 = (30, 50);
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect3 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect4 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle is {:?} square pixels.",
        // area(width1, height1)
        // area1(rect1)
        // area2(&rect2)
        // rect2
        rect2.area(),
    );

    // dbg!(&rect2);

    println!("Can rect2 hold rect3? {}", rect2.can_hold(&rect3));
    println!("Can rect2 hold rect4? {}", rect2.can_hold(&rect4));
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area1(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area2(dimensions: &Rectangle) -> u32 {
    dimensions.height * dimensions.width
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn area_test() {
//         let expected = 35;
//         let actual = area(7, 5);
//         assert_eq!(actual, expected);
//     }
// }

#[cfg(test)]
#[path = "rectangles_test.rs"]
mod rectangles_test;
