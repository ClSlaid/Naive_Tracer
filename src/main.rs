use Naive_Tracer::Image;

fn main() {
    // generate ppm data
    let ppm = (0..128)
        .map(
            // vector with 128 rows
            |i| // the i-th row
            (i..(i + 128) as u8).map(  // editing pixel in i-th row
                |x|{
                    (x, (x as i32- 75).abs() as u8,  255 - x)
                }
            ).collect(),
        )
        .collect();

    // transform and store ppm dat
    let img = Image::from(ppm);
    // print ppm data
    img.print();
}
