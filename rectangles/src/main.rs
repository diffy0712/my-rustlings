
fn main() {
    let rect = Rectangle {
        width: 10,
        height: 20
    };

    println!("The rect is: {:?}", rect);
    
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect2 = Rectangle::square(5);

    if rect.can_hold(&rect2) {
        println!("{:?} can hold {:?}", rect, rect2);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn square(width: i32) -> Self {
        Self {
            width,
            height: width
        }
    }

    // FIXME: handle negative values as negative width, or height does not makes sense.
    fn area(&self) -> i32{
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_square() {
        let square = Rectangle::square(20);

        assert_eq!(square.width, 20);
        assert_eq!(square.height, 20);
    }

    // NOTE: should test for negative values as well
    // NOTE: should test for more values, but with no parameters yet, I dont want to add more
    #[test]
    fn area_calculates_correctly() {
        let rect1 = Rectangle {
            width: 10,
            height: 10
        };

        assert_eq!(rect1.area(), 100);

        let rect2 = Rectangle {
            width: 22,
            height: 11
        };

        assert_eq!(rect2.area(), 242);
    }

    #[test]
    fn can_hold_smaller_rectangle() {
        let rect1 = Rectangle {
            width: 10,
            height: 10
        };

        let rect2 = Rectangle {
            width: 9,
            height: 9
        };

        assert_eq!(rect1.can_hold(&rect2), true);
    }

    #[test]
    fn cannot_hold_bigger_rectangle() {
        let rect1 = Rectangle {
            width: 10,
            height: 10
        };

        let rect2 = Rectangle {
            width: 9,
            height: 9
        };

        assert_eq!(rect2.can_hold(&rect1), false);
    }
}

