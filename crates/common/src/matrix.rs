pub struct FlatMatrix {
    matrix: String,
    width: usize,
    height: usize,
}

impl FlatMatrix {
    pub fn new(matrix: &str) -> Self {
        Self::new_with_delimiter(matrix, '\n')
    }

    pub fn new_with_delimiter(matrix: &str, delimiter: char) -> Self {
        let mut split = matrix.split(delimiter);
        let width = split.next().expect("delimiter not found").len();
        let height = split.count() + 1;

        Self {
            matrix: matrix.replace('\n', ""),
            width,
            height,
        }
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn at(&self, idx: usize) -> char {
        self.matrix.as_bytes()[idx] as char
    }

    pub fn update(&mut self, idx: usize, val: char) {
        unsafe {
            self.matrix.as_bytes_mut()[idx] = val as u8;
        }
    }

    pub fn neighbors(&self, idx: usize) -> [Option<char>; 8] {
        let x: i64 = (idx % self.width) as i64;
        let y: i64 = (idx / self.width) as i64;
        let bytes = self.matrix.as_bytes();
        [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .map(|(x, y)| {
            if x < 0 || x >= self.width as i64 || y < 0 || y >= self.height as i64 {
                None
            } else {
                let idx = y as usize * self.width + x as usize;
                Some(bytes[idx] as char)
            }
        })
    }
}
