   // Set the c1 ref to mut to allow it to call
    // methods on the Circle instance that modifies the
    // values of certain fields
    let mut c1 = Circle {
        radius: 7.8,
        origin_x: 7,
        origin_y: 8,
        colour: "orange".to_string(),
    };
    println! {"The area of the circle is: {:.1$}", c1.area(), 2};
    println! {"The colour of the circle is: {}", c1.colour};
    println! {"Original Circle co-ords: {}, {}", c1.origin_x, c1.origin_y};
    c1.move_cord();
    println! {"New position of Circle co-ords: {}, {}", c1.origin_x, c1.origin_y};

// Struct class
struct Circle {
    radius: f32,
    origin_x: i32,
    origin_y: i32,
    colour: String,
}

// Any Circle methods goes into an implementation block
impl Circle {
    // Returns the area of the circle
    // &self is a reference to the 'Circle' instance
    // hence why we are able to access the fields
    fn area(&self) -> f32 {
        (3.14) * (self.radius * self.radius)
    }

    // &mut self allows you to change the current values
    // for certain fields of the Circle class
    fn move_cord(&mut self) {
        self.origin_x += 10;
        self.origin_y += 20;
    }
}
