#[derive(Debug)] // Outer attribute
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // fn area(self: &Self) -> u32 {
        dbg!(self.width * self.height)
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width:size,
            height: size,
        }
    }
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    println!(
        "The area_tuple of the rectangle is {} square pixels.",
        area_tuple((width1, height1))
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    dbg!(&rect1);

    println!(
        "The area_struct of the rectangle is {} square pixels.",
        area_struct(&rect1)
    );

    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("{:?}", rect2.width);

    println!(
        "The area_struct of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );

    if rect2.width() {
        println!(
            "The area method of the rectangle is {} square pixels.",
            rect2.area()
        );
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let sq = Rectangle::square(3);
    println!("{:#?}", sq);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    // println!("{:#?}", rectangle);
    rectangle.width * rectangle.height
}
