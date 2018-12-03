#[derive(Debug)]
pub struct ParsedLine {
    pub id: i32,
    pub start_x: i32,
    pub start_y:i32,
    pub size_x:i32,
    pub size_y:i32
}

impl ParsedLine{
    pub fn new(id: i32, start_x: i32, start_y:i32, size_x:i32, size_y:i32) -> ParsedLine {
        ParsedLine{id:id, start_x:start_x, start_y:start_y, size_x:size_x, size_y:size_y}
    }

    fn split_coords(coords: &str, separator: char) -> (i32, i32) {
        let line_splitted : Vec<&str> =  coords.split(separator).collect();
        let x_str = line_splitted[0];
        let y_str = line_splitted[1]; 
        return (
            x_str.parse().unwrap(),
            y_str.parse().unwrap())
    }
    pub fn from_line(line:&str) -> ParsedLine {
        let (_, line) = line.split_at(1); // skip first #
        // pop the id_str
        let line_splitted : Vec<&str> =  line.split(" @ ").collect();
        let id_str = line_splitted[0];
        let id = id_str.parse().unwrap();
        let line = line_splitted[1]; 
        // pop coords and the size
        let line_splitted : Vec<&str> =  line.split(": ").collect();
        let coords_str = line_splitted[0];
        let size_str = line_splitted[1];
        let (start_x, start_y) = Self::split_coords(coords_str, ',');
        let (size_x, size_y) = Self::split_coords(size_str, 'x');
        //let [coords_str, size] = line.split(": ");
        return ParsedLine::new(id, start_x, start_y, size_x, size_y);
    }
}