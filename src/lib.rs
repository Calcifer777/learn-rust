pub enum Shape {
    Rectangle(f32, f32),
    Square(f32),
    Triangle(f32, f32, f32),
}

impl Shape {
    /// Returns the area of the shape
    /// 
    /// # Examples
    /// ```
    /// use learn_rust::Shape::Rectangle;
    /// let r = Rectangle(2.0, 3.0);
    /// 
    /// let area = r.area(); // 6.0
    /// assert_eq!(area, 6.0);
    /// ```
    pub fn area(&self) -> f32 {
        match self {
            Shape::Rectangle(w, h) => w*h,
            Shape::Square(s) => s*s,
            Shape::Triangle(_, _, _) => 1.0,
        }
    }
}
