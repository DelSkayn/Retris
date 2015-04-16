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

static BOARD_W:usize = 10 as usize;
static BOARD_H:usize = 25 as usize;

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
            board: board::Board::new(BOARD_W,BOARD_H),
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
            //update
            for i in 0..BOARD_H{
                for j in 0..BOARD_W{
                    let color = self.board.get_color(j as i32,i as i32);
                        let x = (20*BOARD_H - i*20) as f32;
                        let y = (j*20) as f32;
                        let rgb_color = match color {
                            0 => math::Color{r:1.0,g:0.0,b:0.0},
                            1 => math::Color{r:0.0,g:1.0,b:0.0},
                            2 => math::Color{r:0.0,g:0.0,b:1.0},
                            3 => math::Color{r:0.5,g:0.5,b:0.0},
                            4 => math::Color{r:0.5,g:0.0,b:0.5},
                            5 => math::Color{r:0.0,g:0.5,b:0.5},
                            6 => math::Color{r:0.3,g:0.3,b:0.3},
                            -1 => math::Color{r:0.1,g:0.1,b:0.1},
                            _ => math::Color{r:0.0,g:0.0,b:0.0},
                        };
                        self.render_engine.add_render_obj(render::RenderObject{
                            offset: math::Vector2{x:y,y:x},
                            scale: if color == -1 {math::Vector2{x:5.0,y:5.0}} else {math::Vector2{x:20.0,y:20.0}},
                            color: rgb_color,
                        });
                }
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

