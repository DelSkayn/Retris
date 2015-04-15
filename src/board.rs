use rand;

pub struct Position{
    x: i32,
    y: i32,
}


pub struct Board{
    board_color: [i32; 104],
    board_active: [bool; 104],
}

impl Board{
    pub fn new() -> Board{
        Board{
            board_color: [-1; 13*8],
            board_active: [false; 13*8],
        }
    }

    pub fn get_color(&self,x: i32,y : i32) -> i32{
        if x < 0 || x > 7 || y < 0 || y > 12 {
            return -2;//-2 equals border
        }
        self.board_color[(y*8+x) as usize]
    }

    pub fn get_active(&self,x: i32,y : i32) -> bool{
        if x < 0 || x > 7 || y < 0 || y > 12 {
            return false;
        }
        self.board_active[(y*8+x) as usize]
    }

    pub fn set_color(&mut self,x: i32,y : i32, value: i32) {
        self.board_color[(y*8+x) as usize] = value;
    }

    pub fn set_active(&mut self,x: i32,y : i32, value: bool){
        self.board_active[(y*8+x) as usize] = value;
    }

    pub fn print(&self){
        println!("Color: ");
        for i in 0..13{
            for j in 0..8{
                print!(" {}",self.get_color(j,i));
            }
            println!("");
        }
        println!("Active: ");
        for i in 0..13{
            for j in 0..8{
                print!(" {}",self.get_active(j,i));
            }
            println!("");
        }
    }

    pub fn move_left(&mut self) -> bool{
        for i in 0..13{
            for j in 0..8{
                if self.get_active(j,i) {
                    if self.get_color(j-1,i) != -1 && !self.get_active(j-1,i){
                        return false;
                    }
                }
            }
        }
        for i in 0..13{
            for j in 0..8{
                if self.get_active(j,i) {
                    self.board_active[(i*8+j) as usize] = false;
                    self.board_active[(i*8+j-1) as usize] = true;
                    self.board_color[(i*8+j-1) as usize] = self.board_color[(i*8+j) as usize];
                    self.board_color[(i*8+j) as usize] = -1;
                }
            }
        }
        true
    }

    pub fn move_right(&mut self) -> bool{
        for i in 0..13{
            for j in 0..8{
                if self.get_active(j,i) {
                    if self.get_color(j+1,i) != -1 && !self.get_active(j+1,i){
                        return false;
                    }
                }
            }
        }
        for i in 0..13{
            for j in (0..8).rev(){
                if self.get_active(j,i) {
                    self.board_active[(i*8+j) as usize] = false;
                    self.board_active[(i*8+j+1) as usize] = true;
                    self.board_color[(i*8+j+1) as usize] = self.board_color[(i*8+j) as usize];
                    self.board_color[(i*8+j) as usize] = -1;
                }
            }
        }
        true
    }

    pub fn update(&mut self) -> bool{
        for i in 0..13{
            for j in 0..8{
                if self.get_active(j,i) {
                    if self.get_color(j,i+1) != -1 && !self.get_active(j,i+1){
                        return false;
                    }
                }
            }
        }
        for i in (0..13).rev(){
            for j in 0..8{
                if self.get_active(j,i) {
                    self.board_active[(i*8+j) as usize] = false;
                    self.board_active[((i+1)*8+j) as usize] = true;
                    self.board_color[((i+1)*8+j) as usize] = self.board_color[(i*8+j) as usize];
                    self.board_color[(i*8+j) as usize] = -1;
                }
            }
        }
        true
    }

    pub fn add_shape(&mut self,shape : Shape){
        let color = (rand::random::<u16>() % 10) as i32 ;
        for i in 0..4 {
            self.set_color(4+shape.blocks[i].x,1+shape.blocks[i].y,color);
            self.set_active(4+shape.blocks[i].x,1+shape.blocks[i].y,true);
        }
    }
}

pub struct Shape{
    blocks: [Position; 4],
    rotation: u8,
}

impl Shape{
    pub fn new_line() -> Shape {
        Shape{
            blocks: [
                Position{x:0,y:0},
                Position{x:0,y:1},
                Position{x:0,y:-1},
                Position{x:0,y:2}],
            rotation: 0,
        }
    }
    pub fn new_block() -> Shape {
        Shape{
            blocks: [
                Position{x:0,y:0},
                Position{x:0,y:1},
                Position{x:1,y:1},
                Position{x:1,y:0}],
            rotation: 0,
        }
    }
    pub fn new_plus() -> Shape {
        Shape{
            blocks: [
                Position{x:0,y:0},
                Position{x:0,y:1},
                Position{x:1,y:0},
                Position{x:0,y:-1}],
            rotation: 0,
        }
    }
    pub fn new_l() -> Shape {
        Shape{
            blocks: [
                Position{x:0,y:0},
                Position{x:0,y:1},
                Position{x:0,y:-1},
                Position{x:1,y:1}],
            rotation: 0,
        }
    }
    pub fn new_l_inv() -> Shape {
        Shape{
            blocks: [
                Position{x:0,y:0},
                Position{x:0,y:1},
                Position{x:0,y:-1},
                Position{x:-1,y:1}],
            rotation: 0,
        }
    }
    pub fn new_s() -> Shape {
        Shape{
            blocks: [
                Position{x:0,y:0},
                Position{x:0,y:1},
                Position{x:1,y:0},
                Position{x:1,y:1}],
            rotation: 0,
        }
    }
    pub fn new_s_inv() -> Shape {
        Shape{
            blocks: [
                Position{x:0,y:0},
                Position{x:0,y:1},
                Position{x:-1,y:0},
                Position{x:-1,y:1}],
            rotation: 0,
        }
    }

    pub fn new_rand_shape() -> Shape{
        let num = rand::random::<u8>() % 7;
        println!("Rand = {}", num);
        match num {
            0 => Shape::new_line(),
            1 => Shape::new_l(),
            2 => Shape::new_s(),
            3 => Shape::new_s_inv(),
            4 => Shape::new_l_inv(),
            5 => Shape::new_plus(),
            6 => Shape::new_block(),
            _ => Shape::new_line(),
        }
    }
}

