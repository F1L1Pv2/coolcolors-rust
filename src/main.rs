extern crate termsize;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    execute, queue,
    style::{
        Attribute as CAttribute, Color as CColor, Print, SetAttribute, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{self, Clear, ClearType}, event::read,
};
use std::io::{self, Write};
//import time
use std::time::Duration;

use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    let size = termsize::get().unwrap();
    let mut stdout = io::stdout();
    let mut player = (size.rows / 2, size.cols / 2);

    let device_state = DeviceState::new();

    //disable cursor
    execute!(stdout, Hide).unwrap();

    loop {



        let keys: Vec<Keycode> = device_state.get_keys();

        for key in keys {
            match key {
                Keycode::W => {
                    if player.0 > 0{
                        player.0 -= 1;
                    }
                }
                Keycode::S => {
                    if player.0 < size.rows-1 {
                        player.0 += 1;
                    }
                }
                Keycode::A => {
                    if player.1 > 0{
                        player.1 -= 1;
                    }
                }
                Keycode::D => {
                    if player.1 < size.cols-1 {
                        player.1 += 1;
                    }
                }
                _ => {}
            }
        }
        



        //clear terminal
        // print!("\x1b[2J");
    
        //clear terminal
        // execute!(
        //     stdout,
        //     Clear(ClearType::All),
        //     MoveTo(0, 0),
        //     SetForegroundColor(CColor::Rgb {
        //         r: 255,
        //         g: 255,
        //         b: 255
        //     }),
        //     SetBackgroundColor(CColor::Rgb {
        //         r: 0,
        //         g: 0,
        //         b: 0
        //     }),
        //     Print(" ")
        // ).unwrap();

        for i in 0..size.rows {
            // let mut line = "".to_string();

            for j in 0..size.cols {
                let r = (i as f32 * 255. / (size.rows as f32)) as usize;
                let g = (j as f32 * 255. / (size.cols as f32)) as usize;
                // print!(
                // "\x1b[48;2;{};{};0m\x1b[38;2;{};{};0m ",
                // r,
                // g,
                // 255 - r,
                // 255 - g
                // );

                //use write! to avoid flushing stdout
                // write!(
                //     stdout,
                //     "\x1b[48;2;{};{};0m\x1b[38;2;{};{};0m ",
                //     r,
                //     g,
                //     255 - r,
                //     255 - g
                // );


                //use move cursor


                // execute!(
                //     stdout,
                //     MoveTo(j as u16, i as u16),
                //     SetForegroundColor(CColor::Rgb {
                //         r: r as u8,
                //         g: g as u8,
                //         b: 0
                //     }),
                //     SetBackgroundColor(CColor::Rgb {
                //         r: 255 - r as u8,
                //         g: 255 - g as u8,
                //         b: 0
                //     }),
                //     Print(" ")
                // ).unwrap();


                if i == player.0 && j == player.1 {
                    execute!(
                        stdout,
                        MoveTo(j as u16, i as u16),
                        SetForegroundColor(CColor::Rgb {
                            r: 255,
                            g: 255,
                            b: 255
                        }),
                        SetBackgroundColor(CColor::Rgb {
                            r: 0,
                            g: 0,
                            b: 0
                        }),
                        Print(" ")
                    ).unwrap();
                }else{
                    execute!(
                        stdout,
                        MoveTo(j as u16, i as u16),
                        SetForegroundColor(CColor::Rgb {
                            r: r as u8,
                            g: g as u8,
                            b: 0
                        }),
                        SetBackgroundColor(CColor::Rgb {
                            r: 255 - r as u8,
                            g: 255 - g as u8,
                            b: 0
                        }),
                        Print(" ")
                    ).unwrap();

                }



              

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
            // print!("\x1b[0m\n");
        }

        


    }
}
