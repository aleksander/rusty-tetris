extern crate rand;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::rc::Rc;
use std::cell::RefCell;
use opengl_graphics::{ GlGraphics, OpenGL };
use glutin_window::GlutinWindow as Window;
use piston::window::WindowSettings;
use piston::input::*;
use piston::event_loop::Events;

mod tetromino;
mod active;
mod tetris;

fn main() { 
    let mini = false;
    let (width, height) = (400, 800);
    let (width, height) = if mini { (width / 2, height / 2) } else { (width, height) };
    let opengl = OpenGL::V3_2;
    let window = Window::new(
        WindowSettings::new("Rusty Tetris", [width, height])
        .exit_on_esc(true)
        .fullscreen(false)
        .opengl(opengl)
    ).expect("Window::new()");

    let mut game = tetris::Tetris::new(if mini { 0.5 } else { 1.0 });
    let ref mut gl = GlGraphics::new(opengl);
    let window = Rc::new(RefCell::new(window));
    for e in window.events() {
        use piston::input::Button;

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                graphics::clear([1.0; 4], gl);
                game.render(&c, gl);
            });
        }

        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_press(&key);
        }

        if let Some(Button::Keyboard(key)) = e.release_args() {
            game.key_release(&key);
        }

        if let Some(uargs) = e.update_args() {
            game.update(&uargs);
        }

    }
}
