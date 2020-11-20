pub struct Image {
    // struct used to contain PPM data
    width: u16,
    height: u16,
    data: Vec<Vec<(u8, u8, u8)>>,
}

impl Image {
    pub fn new() -> Image {
        // new() a empty image
        Image {
            width: 0,
            height: 0,
            data: vec![vec![]],
        }
    }
    pub fn from(mat: Vec<Vec<(u8, u8, u8)>>) -> Image {
        // generate PPM data from Vector
        let h = mat.len();
        let w = mat[0].len();
        Image {
            width: w as u16,
            height: h as u16,
            data: mat,
        }
    }
}
impl Image {
    pub fn print(&self) {
        // print data directly to console
        // you can also redirect output to local file
        println!("P3\n{} {}\n255", self.width, self.height);
        for line in self.data.iter() {
            for pixel in line {
                print!("{}\t{}\t{}\t", pixel.0, pixel.1, pixel.2);
            }
            print!("\n");
        }
    }
}

#[cfg(test)]
mod tests {}
