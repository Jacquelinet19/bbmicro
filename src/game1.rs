use crate::api::{BBMicroApi, BBMicroGame, Button};

pub struct StopLight {
    top: u8,
    bott: u8,
    x: f32,
    y: f32,
}

pub struct Game1 {
    count: u32,
    x: f32,
    y: f32,
    p1x: f32,
    p1y:f32,
    p2x:f32,
    p2y: f32,
    p1w: bool,
    p2w: bool,
    stop_light: StopLight,
    green_light: bool,
    spr1 : u8,
    spr2: u8,
    rd_lt_timer: u8,
    valid_move: bool,
}

impl Game1 {
    pub fn new() -> Game1 {
        Game1 {
            count: 0,
            x:100.0,
            y:100.0,
            p1x: 40.0,
            p1y: 100.0,
            p2x: 40.0,
            p2y: 80.0,
            p1w: false,
            p2w: false,
            stop_light: 
                StopLight{
                    top: Tiles::RDTop as u8,
                    bott: Tiles::RDBott as u8,
                    x: 100.0,
                    y: 52.0
                },
            green_light: true,
            spr1:8,
            spr2:9,
            rd_lt_timer: 0,
            valid_move: true,
        }
    }
}

enum Tiles {
    FinishLine = 68,
    Grass = 48,
    Startline=71,
    Track=70,
    RDTop = 64,
    RDBott = 80,
    GrTop = 65,
    GRBott = 81
}

impl BBMicroGame for Game1 {
    fn init(&mut self, api: &mut BBMicroApi) {
        // Draw the base map on layer 0.
        for x in 0..256 {
            for y in 0..16 {
                api.mset(x, y, 0, Tiles::Grass as u8);
            }
        }

        //Play BGM
        api.music("bgm", 0,0); //Uncomment after adding music.mp3
    }
    
    fn update(&mut self, api: &mut BBMicroApi) {

        if !self.p1w && !self.p2w {
            
            if self.rd_lt_timer > 0 {
                self.rd_lt_timer -= 1;
                if 90 - self.rd_lt_timer > 15 && self.valid_move {
                    self.valid_move = false;
                }
            } 
            else {
            self.green_light = true;
            self.valid_move = true;
            }
            //Controls for moving forward
            if api.btn(Button::RIGHT) {
                if self.p1x < 155.0{
                    if self.valid_move {
                        self.p1x += 2.0;
                        if self.spr1 == 8 {self.spr1=24}else {self.spr1=8 };
                    } else {
                        self.p1x -= 10.0;
                    }
                }else{
                    self.p1w = true;
                }
            }
            if api.btn(Button::D){
                if self.p2x < 155.0 {
                    if self.valid_move {
                        self.p2x += 2.0;
                        if self.spr2 == 9 {self.spr2=25}else {self.spr2=9 };
                    }
                    else {
                        self.p2x -= 10.0
                    }
                }else{
                    self.p2w = true;
                }
            }

            //Controls for turning light red
            if api.btn(Button::LEFT) {
                if self.green_light { 
                    self.green_light = false;
                    self.rd_lt_timer = 90;
                }
                
            }
            if api.btn(Button::A) {
                if self.green_light {
                    self.green_light = false;
                    self.rd_lt_timer = 90;
                }
            }
            
        } else if api.btnp(Button::A) || api.btnp(Button::D) || api.btnp(Button::LEFT) || api.btnp(Button::RIGHT) {
            //Restart
            self.p1x = 40.0;
            self.p1y = 100.0;
            self.p2x = 40.0;
            self.p2y = 80.0;
            self.p1w = false;
            self.p2w = false;
            self.green_light = true;
            self.valid_move = true;
            self.rd_lt_timer = 0;

        }

        
        self.stop_light.top = if self.green_light {Tiles::GrTop as u8} else {Tiles::RDTop as u8};
        self.stop_light.bott = if self.green_light {Tiles::GRBott as u8} else {Tiles::RDBott as u8};

        self.stop_light.x = self.x;
        self.stop_light.y = self.y - 48.0;
    }

    fn draw(&mut self, api: &mut BBMicroApi) {
        api.camera(self.x - 60.0, self.y - 60.0);

        // Draw map layer 0.
        api.map(0, 0, 0.0, 0.0, 256, 256, 0);


        api.rect(10.0, 10.0, 20.0, 20.0, 1);
        //Makes sprites
        api.spr(self.spr1, self.p1x, self.p1y, 8.0, 8.0, false, false);
        api.spr(self.spr2, self.p2x, self.p2y, 8.0, 8.0, false, false);

        //Draw finish line
        api.mset(20, 10, 0, Tiles::FinishLine as u8);
        api.mset(20, 11, 0, Tiles::FinishLine as u8);
        api.mset(20, 13, 0, Tiles::FinishLine as u8);
        api.mset(20, 12, 0, Tiles::FinishLine as u8);

        //Draw start line
        api.mset(5, 10, 0, Tiles::Startline as u8);
        api.mset(5, 11, 0, Tiles::Startline as u8);
        api.mset(5, 13, 0, Tiles::Startline as u8);
        api.mset(5, 12, 0, Tiles::Startline as u8);

        //Draw track
        for x in 6..20 {
            api.mset(x, 10, 0, Tiles::Track as u8);
            api.mset(x, 11, 0, Tiles::Track as u8);
            api.mset(x, 13, 0, Tiles::Track as u8);
            api.mset(x, 12, 0, Tiles::Track as u8);
        }

        //Draw Stop Light
        api.spr(self.stop_light.top, self.stop_light.x, self.stop_light.y, 8.0, 8.0, false, false);
        api.spr(self.stop_light.bott, self.stop_light.x, self.stop_light.y + 8.0, 8.0, 8.0, false, false);
  

        // Draw map layer 1.
        api.map(80, 0, 0.0, 0.0, 256, 256, 1);

        if self.p1w == true{
            api.print("PLAYER ONE WINS", 5.0, 5.0, false);
        }else if self.p2w == true {
            api.print("PLAYER TWO WINS", 5.0, 5.0, false);
        }
        
    }
}
