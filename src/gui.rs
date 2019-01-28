use crate::cf32;

#[cfg(feature = "fft_chfft")]
use crate::fft::{Scale, Fft, Cfft};

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate chfft;
extern crate rand;

use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

/// Generic over I: Input type
pub trait Liveplot<I>{
    fn render(&self, gl : &mut GlGraphics, args: &RenderArgs);
    fn update(&mut self, newdata : Vec<I>);
}

/// Create a simple waterfall plot  
/// With the given number of FFT bins
/// # Example
///```
/// use aether_primitives::cf32;
/// use aether_primitives::gui;
/// use aether_primitives::channel::noise;
/// use std::sync::mpsc;
/// let bins = 512;
/// let (s, r) = mpsc::channel::<Vec<cf32>>();
/// let waterfall = gui::waterfall(bins);
/// 
/// // this would spawn of the gui thread
/// // this either stops if the sender is dropped
/// // or the exit button is clicked on the waterfall window
/// // let jh = gui::launch(r, waterfall);
/// let mut g = noise::generator();
/// let v = g.iter().take(bins).collect();
/// s.send(v).expect("Failed to send");
/// let v = g.iter().take(bins).collect();
/// s.send(v).expect("Failed to send");
/// // ok these should be rendered now
/// // lets close the gui and reap the thread
/// drop(s);
/// // jh.join().expect("Failed to rejoin thread")
/// 
///```
#[cfg(feature = "fft_chfft")]
pub fn waterfall(ncol : usize) -> Waterfall{
    const NROWS : usize = 200;
    Waterfall {
        nrows : NROWS,
        ncols : ncol,
        fft : Cfft::with_len(ncol),
        all_rows: vec![],
    }
}


/// Launch the configured Liveplot with the provided input source
/// 
pub fn launch<I: Sync + Send + 'static, L: Liveplot<I> + Send + 'static>(input : Receiver<Vec<I>>,mut l : L) -> thread::JoinHandle<()>{
    thread::spawn(move ||{
        let opengl = OpenGL::V3_2;

        let mut window: GlutinWindow = WindowSettings::new("GUI", [1000, 700])
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .expect("Failed to open glutin window");

        let mut gl = GlGraphics::new(opengl);

        let mut events = Events::new(EventSettings::new());


        while let Some(e) = events.next(&mut window) {
            // handle resizing the window etc
            if let Some(arg) = e.render_args() {
                l.render(&mut gl, &arg);
            }

            // if there is no more data close the mainloop
            match input.try_recv(){
                Err(TryRecvError::Disconnected) => break,
                Err(TryRecvError::Empty) => (),
                Ok(data) => l.update(data)
            }
        }
    })
}


#[cfg(feature = "fft_chfft")]
pub struct Waterfall {
    nrows : usize,
    ncols : usize,
    fft : crate::fft::Cfft,
    all_rows: Vec<Vec<f32>>,
}

impl Liveplot<cf32> for Waterfall{
    fn render(&self, gl : &mut GlGraphics, args: &RenderArgs) {
        // helpers
        use graphics::{clear,Colored,rectangle, Transformed};

        // use cyan for 0 and 
        const CYAN: [f32; 4] = [0.0, 1.0, 1.0, 1.0];
        //

        let rowheight: f64 = args.height / self.nrows as f64;
        let colwidth: f64 = args.width / self.ncols as f64;

        let bottom = args.height - rowheight;

        let all_rows = &self.all_rows;

        gl.draw(args.viewport(), |c, gl| {
            clear(CYAN, gl);
            if all_rows.is_empty() {
                return;
            }

            all_rows.iter().rev().enumerate()
            .for_each(|(rownum, row)| {
                let row_offset = bottom - rowheight * rownum as f64;
                row.iter().enumerate()
                    .for_each(|(colnum, v)| {
                        rectangle(
                            CYAN.hue_deg(v*0.5*360f32), // hue_deg is a function of the graphics::Colored trait
                            [0.0, 0.0, colwidth, rowheight], //this defines a rectangle x,y,w,h
                            c.transform.trans(colnum as f64 * colwidth, row_offset), // this generates a 2x2 transform matrix
                            gl,
                        )
                });
            })
        });
    }

    fn update(&mut self, new_row: Vec<cf32>) {
        // recycle the vecs used for internal state
        let mut v = if self.all_rows.len() == self.nrows{
            // drop the first element if we are at the limit
            self.all_rows.remove(0)
        } else{
            // OPT: do not zero first
            vec![0.0; self.ncols]
        };

        let mid = v.len()/2;
        let (front, end) = v.split_at_mut(mid);

        // manually mirrors
        self.fft.tfwd(&new_row, Scale::SN).iter()
            .zip(end.iter_mut().chain(front.iter_mut()))
            .for_each(|(c,v)|*v = c.norm());

        // push the new line to the back
        self.all_rows.push(v);
    }

}