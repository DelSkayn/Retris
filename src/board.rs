use rand;


#[derive(Clone,Copy)]
pub struct Position{
    x: i32,
    y: i32,
}


pub struct Board{
    board_color: Vec<i32>,
    board_active: Vec<bool>,
    w:i32,
    h:i32,
    current_shape:Shape,
    x:i32,
    y:i32,
}

impl Board{
    pub fn new(w:usize,h:usize) -> Board{
        Board{
            board_color: vec![-1; h*w],
            board_active: vec![false; h*w],
            w:w as i32,
            h:h as i32,
            current_shape: Shape::new_l(),
            x:0,
            y:0,
        }
    }

    pub fn get_color(&self,x: i32,y : i32) -> i32{
        if x < 0 || x > self.w -1 || y < 0 || y > self.h -1 {
            return -2;//-2 equals border
        }
        self.board_color[(y*self.w+x) as usize]
    }

    pub fn get_active(&self,x: i32,y : i32) -> bool{
        if x < 0 || x > self.w -1 || y < 0 || y > self.h -1 {
            return false;
        }
        self.board_active[(y*self.w+x) as usize]
    }

    pub fn set_color(&mut self,x: i32,y : i32, value: i32) {
        self.board_color[(y*self.w+x) as usize] = value;
    }

    pub fn set_active(&mut self,x: i32,y : i32, value: bool){
        self.board_active[(y*self.w+x) as usize] = value;
    }

    pub fn print(&self){
        println!("");
        for i in 0..self.h{
            for j in 0..self.w{
                print!("|{}: {}",self.get_color(j,i),self.get_active(j,i));
            }
            println!("");
        }
    }

    pub fn move_left(&mut self) -> bool{
        for i in 0..self.h{
            for j in 0..self.w{
                if self.get_active(j,i) {
                    if self.get_color(j-1,i) != -1 && !self.get_active(j-1,i){
                        return false;
                    }
                }
            }
        }
        for i in 0..self.h{
            for j in 0..self.w{
                if self.get_active(j,i) {
                    self.board_active[(i*self.w+j) as usize] = false;
                    self.board_active[(i*self.w+j-1) as usize] = true;
                    self.board_color[(i*self.w+j-1) as usize] = self.board_color[(i*self.w+j) as usize];
                    self.board_color[(i*self.w+j) as usize] = -1;
                }
            }
        }
        self.x-=1;
        true
    }

    pub fn move_right(&mut self) -> bool{
        for i in 0..self.h{
            for j in 0..self.w{
                if self.get_active(j,i) {
                    if self.get_color(j+1,i) != -1 && !self.get_active(j+1,i){
                        return false;
                    }
                }
            }
        }
        for i in 0..self.h{
            for j in (0..self.w).rev(){
                if self.get_active(j,i) {
                    self.board_active[(i*self.w+j) as usize] = false;
                    self.board_active[(i*self.w+j+1) as usize] = true;
                    self.board_color[(i*self.w+j+1) as usize] = self.board_color[(i*self.w+j) as usize];
                    self.board_color[(i*self.w+j) as usize] = -1;
                }
            }
        }
        self.x+=1;
        true
    }

    pub fn update(&mut self) -> bool{
        for i in 0..self.h{
            for j in 0..self.w{
                if self.get_active(j,i) {
                    if self.get_color(j,i+1) != -1 && !self.get_active(j,i+1){
                        for i in 0..self.h{
                            for j in 0..self.w{
                                self.set_active(j,i,false);
                            }
                        }
                        return false;
                    }
                }
            }
        }
        for i in (0..self.h).rev(){
            for j in 0..self.w{
                if self.get_active(j,i) {
                    self.board_active[(i*self.w+j) as usize] = false;
                    self.board_active[((i+1)*self.w+j) as usize] = true;
                    self.board_color[((i+1)*self.w+j) as usize] = self.board_color[(i*self.w+j) as usize];
                    self.board_color[(i*self.w+j) as usize] = -1;
                }
            }
        }
        self.y += 1;
        true
    }

    pub fn rotate(&mut self){
        let mut possible = true;
        {
            let mut shape = self.current_shape.clone();
            for _ in 0..self.current_shape.rotation+1{
                shape.rotate();
            }
            for i in 0..4 {
                let x = 4+shape.blocks[i].x + self.x;
                let y = 1+shape.blocks[i].y + self.y;
                if self.get_color(x,y) != -1 && !self.get_active(x,y){
                    possible = false;
                }
            }
        }
        if possible {
            let color = self.get_color(4+self.current_shape.blocks[0].x + self.x,1+self.current_shape.blocks[0].y + self.y);
            for i in 0..4{
                let x = 4+self.current_shape.blocks[i].x + self.x;
                let y = 1+self.current_shape.blocks[i].y + self.y;
                self.set_color(x,y,-1);
                self.set_active(x,y,false);
            }
            self.current_shape.rotate();
            for i in 0..4{
                let x = 4+self.current_shape.blocks[i].x + self.x;
                let y = 1+self.current_shape.blocks[i].y + self.y;
                self.set_color(x,y,color);
                self.set_active(x,y,true);
            }
        }
    }

    pub fn add_shape(&mut self,shape : Shape){
        self.current_shape = shape.clone();
        self.x = 0;
        self.y = 0;
        let color = (rand::random::<u16>() % 7) as i32;
        for i in 0..4 {
            self.set_color(4+shape.blocks[i].x,1+shape.blocks[i].y,color);
            self.set_active(4+shape.blocks[i].x,1+shape.blocks[i].y,true);
        }
    }

    pub fn check_rows(&mut self) -> i32{
        let mut res = 0i32;
        for j in 0..self.h{
            let mut row_complete = true;
            for i in 0..self.w{
                if self.get_color(i,j) == -1 || self.get_active(i,j) {
                    row_complete = false;
                    break;
                }
            }
            if row_complete {
                for k in (0..j).rev(){
                    println!("k: {}", k);
                    for i in 0..self.w{
                        let value = self.get_color(i,k);
                        self.set_color(i,k+1,value);
                        self.set_color(i,k,-1);
                    }
                }
                res+=1;
            }
        }
        res
    }
}

#[derive(Clone,Copy)]
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
                Position{x:1,y:-1}],
                rotation: 0,
        }
    }
    pub fn new_s_inv() -> Shape {
        Shape{
            blocks: [
                Position{x:0,y:0},
                Position{x:0,y:1},
                Position{x:-1,y:0},
                Position{x:-1,y:-1}],
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

    pub fn rotate(&mut self){
        for i in 0..4{
            let temp = self.blocks[i].x;
            self.blocks[i].x = self.blocks[i].y*-1;
            self.blocks[i].y = temp;
        }
        self.rotation += 1;
        if self.rotation == 4{
            self.rotation = 0;
        }
    }
}


