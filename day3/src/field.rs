use super::parser;

pub struct Field {
    field : Vec<u8>,
    size_x : i32,
    size_y : i32
}

impl Field {
    pub fn new(size_x : i32, size_y: i32) -> Field{
        let mut field = Vec::with_capacity((size_x * size_y) as usize);
        for i in 0..(size_x*size_y) {
            field.push(0);
        }
        Field{size_x: size_x, size_y: size_y, field:field}
    }
    fn coord2index(&self, x:i32, y:i32) -> i32 {
        self.size_x * y + x
    }

    pub fn add_reservation(&mut self, resv: parser::ParsedLine) {
        for x in resv.start_x..(resv.start_x+resv.size_x) {
            for y in resv.start_y..(resv.start_y+resv.size_y) {
                let idx = self.coord2index(x, y);
                self.field[idx as usize] += 1;
            }
        }
    }
    pub fn count_conflicts(&mut self) -> i32 {
        let mut result = 0;
        for x in 0..self.size_x {
            for y in 0..self.size_x {
                let idx = self.coord2index(x, y);
                if self.field[idx as usize] > 1 {
                    result += 1;
                }
            }
        }
        return result;
    }
}