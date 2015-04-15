#![allow(dead_code)]
#![allow(unused_imports)]

extern crate glfw;
extern crate gl;
extern crate rand;

mod board;
mod render;
mod math;

use glfw::{Action, Context, Key,WindowMode};
use std::sync::mpsc::Receiver;

type EventReciever = Receiver<(f64,glfw::WindowEvent)>;

pub struct Game{
    window: glfw::Window,
    glfw: glfw::Glfw,
    events: EventReciever,
    board: board::Board,
    render_engine: render::Engine,
}

impl Game{
    pub fn new(w: u32 ,h: u32) -> Game{

        let mut glfw_temp = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        let (mut window, events) = glfw_temp.create_window(w,h,"Retris",glfw::WindowMode::Windowed).expect("failed to create window");
        glfw_temp.make_context_current(Some(&window));
        window.set_key_polling(true);

        gl::load_with(|s| window.get_proc_address(s));
        unsafe{
            gl::ClearColor(0.0,0.0,0.0,1.0);
            gl::FrontFace(gl::CCW);
            gl::CullFace(gl::BACK);
            gl::Enable(gl::CULL_FACE);
            gl::DepthRange(0f64,1f64);
        }

        Game{
            window: window,
            glfw: glfw_temp,
            events: events,
            board: board::Board::new(),
            render_engine: render::Engine::new(),
        }
    }

    pub fn start(&mut self){
        self.window.make_current();
        println!("starting");
        while !self.window.should_close(){
            unsafe{
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            //handle events
            self.glfw.poll_events();
            for(_, event) in glfw::flush_messages(&self.events){
                Game::handle_events(&mut self.board,&mut self.window, event);
            }
            self.render_engine.render();
            self.window.swap_buffers();
        }
    }


    fn handle_events(board: &mut board::Board,win: &mut glfw::Window,event: glfw::WindowEvent){
        match event{
            glfw::WindowEvent::Key(Key::Escape, _,Action::Press, _) => win.set_should_close(true),
            glfw::WindowEvent::Key(Key::Left, _,Action::Press, _) => {
                println!("Left pressed");
                board.move_left();
            },
            glfw::WindowEvent::Key(Key::Right, _,Action::Press, _) => {
                println!("Right pressed: {}",board.move_right());
            },
            glfw::WindowEvent::Key(Key::Down, _,Action::Press, _) => {
                println!("Down pressed");
                board.update();
            },
            glfw::WindowEvent::Key(Key::P, _,Action::Press, _) => {
                board.print();
            },
            glfw::WindowEvent::Key(Key::N, _,Action::Press, _) => {
                let s = board::Shape::new_rand_shape();
                board.add_shape(s);
            },
            _ => {},

        }
    }
}

#[test]
fn it_works() {
}

