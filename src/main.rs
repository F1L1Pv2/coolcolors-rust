extern crate termsize;

fn main() {
    termsize::get().map(|size| {
        for i in 0..size.rows {
            // let mut line = "".to_string();

            for j in 0..size.cols {
                let r = (i as f32 * 255. / (size.rows as f32)) as usize;
                let g = (j as f32 * 255. / (size.cols as f32)) as usize;
                print!(
                "\x1b[48;2;{};{};0m\x1b[38;2;{};{};0m ",
                r,
                g,
                255 - r,
                255 - g
                );
                // line.push_str(
                //     format!(
                //         "\x1b[48;2;{};{};0m\x1b[38;2;{};{};0m ", 
                //         r,
                //         g,
                //         255 - r,
                //         255 - g
                //     )
                //     .as_str(),
                // );
            }
            // line.push_str("\x1b[0m\n");
            // print!("{}", line);
            print!("\x1b[0m\n");
        }
    });
}
