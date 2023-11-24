

struct ConsoleDisplay {
    width: usize,
    height: usize,
    buffer: Vec<Vec<u8>>
}

impl ConsoleDisplay {
    pub fn with_size(width: usize, height: usize) -> Self {
        ConsoleDisplay { 
            width,
            height,
            buffer: Self::create_buffer(width, height)
        }
    }

    fn create_buffer(width: usize, height: usize) -> Vec<Vec<u8>> {
        let mut buf: Vec<Vec<u8>> = Vec::with_capacity(height);
        let mut row: Vec<u8> = Vec::with_capacity(width);
        row.resize(width, b'_');
        buf.resize(height, row);
        buf
    }

    pub fn print(&self) {
        for line in self.buffer.iter() {
            println!("{}", String::from_utf8(line.to_vec()).unwrap());
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn set(&mut self, pos: (usize, usize), sym: u8) -> Result<(), &str> {
        if !self.contains(pos) {
            return Err("Position is out of size");
        }

        self.buffer[pos.1][pos.0] = sym;
        Ok(())
    }

    pub fn set_row(&mut self, row: usize, line: &[u8]) -> Result<(), &str> {
        if row >= self.height {
            return Err("Position is out of size");
        }

        self.buffer[row] = line.to_vec();
        Ok(())
    }

    fn contains(&self, point: (usize, usize)) -> bool {
        let (x, y) = point;
        x < self.width && y < self.height
    }
}

fn main() {
    let mut display = ConsoleDisplay::with_size(10, 3);
    display.print();
    display.set((3, 1), b'X').unwrap();
    display.set_row(2, b"YYYYYYYYYY").unwrap();
    display.print();
}
