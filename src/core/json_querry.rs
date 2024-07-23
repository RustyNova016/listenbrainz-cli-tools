use serde::Serialize;

pub trait ReadAsJSON: Serialize + Sized {
    /// Get a field content from its path as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use serde::Serialize;
    /// # use serde::Deserialize;
    ///
    /// #[derive(Serialize)]
    /// struct Point {
    ///     pub x: u32,
    ///     pub y: u32
    /// }
    ///
    /// #[derive(Serialize)]
    /// struct Line {
    ///     pub point_a: u32,
    ///     pub point_b: u32
    /// }
    ///
    /// let point_a = Point {x: 2, y: 5};
    /// let point_b = Point {x: 3, y: 1};
    ///
    /// let line = Line {point_a, point_b};
    ///
    /// assert!(line.get_field("point_a.y"), Ok(5));
    /// assert!(line.get_field("point_a.x"), Ok(3));
    /// assert!(line.get_field("point_a.unknown"), Ok(None);
    /// assert!(line.get_field("point_a.x.foo"), Ok(None));
    ///
    /// ```
    fn get_field(self, field_path: &str) -> Result<serde_json::Value, serde_json::Error> {
        let mut value = serde_json::to_value(self)?;

        let fields = field_path.split('.');
        for field in fields {
            // Extract the content of the field X and put it as the new value
            value = value[field].take();
        }

        Ok(value)
    }
}

impl<T> ReadAsJSON for T where T: Serialize + Sized {}
