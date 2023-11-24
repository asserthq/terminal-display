

struct ConsoleDisplay {
    buffer: Vec<Vec<u8>>

}

impl ConsoleDisplay {
    pub fn with_size(width: usize, height: usize) -> Self {
        ConsoleDisplay { 
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
}

fn main() {
    let display = ConsoleDisplay::with_size(30, 10);
    display.print();
    
}
