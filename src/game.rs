extern crate graphics;
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate input;
extern crate event;

use sdl2_window::Sdl2Window;
use opengl_graphics::Gl;
use std::io::timer;

use piston:: 
{
    WindowSettings,
    RenderArgs,
    UpdateArgs
};

use input:: 
{
    keyboard,
    Press
};

use event:: 
{
    // I think this is the new EventIterator
    Events,
    Window
};

use graphics:: 
{
    Context,
    AddRectangle,
    AddColor,
    Draw,
    RelativeTransform2d
};

static mut x : f64 = -275.0;
static mut y : f64 = 275.0;
static mut printedAlready : bool = false;

pub struct App
{
    gl: Gl,
    rotation: f64
}

impl App
{
    fn render(&mut self, args: &RenderArgs)
    {
        let context = &Context::abs(args.width as f64, args.height as f64);
        context.rgba(0.0,0.0,0.1,0.1).draw(&mut self.gl);

        context
            //draw goal zone
            .trans((args.width / 2) as f64, (args.height / 2) as f64)
            .rect(275.0, -175.0, 50.0, 50.0)
            .rgba(0.0, 1.0, 1.0, 1.0) // TODO try to implement hex codes for coloring
            .trans(-25.0, -25.0)
            .draw(&mut self.gl);
        context
            //draw goal zone upper bound outline
            .trans((args.width / 2) as f64, (args.height / 2) as f64)
            .rect(275.0, -225.0, 50.0, 50.0)
            .rgba(1.0, 0.1, 0.1, 1.0)
            .trans(-25.0, -25.0)
            .draw(&mut self.gl);
        context
            //draw goal zone lower bound outline
            .trans((args.width / 2) as f64, (args.height / 2) as f64)
            .rect(275.0, -125.0, 50.0, 50.0)
            .rgba(1.0, 0.1, 0.1, 1.0)
            .trans(-25.0, -25.0)
            .draw(&mut self.gl);
        context
            //draw goal zone upper bound
            .trans((args.width / 2) as f64, (args.height / 2) as f64)
            .rect(280.0, -220.0, 40.0, 40.0)
            .rgba(0.0, 1.0, 0.5, 1.0)
            .trans(-25.0, -25.0)
            .draw(&mut self.gl);
        context
            //draw goal zone lower bound
            .trans((args.width / 2) as f64, (args.height / 2) as f64)
            .rect(280.0, -120.0, 40.0, 40.0)
            .rgba(0.0, 1.0, 0.5, 1.0)
            .trans(-25.0, -25.0)
            .draw(&mut self.gl);
     unsafe
        {
        context
            //draw player sprite
            .trans((args.width / 2) as f64, (args.height / 2) as f64)
            .rot_rad(self.rotation)
            .rect(x, y, 50.0, 50.0)
            .rgba(0.0, 0.5, 1.0, 1.0)
            .trans(-25.0, -25.0)
            .draw(&mut self.gl)
       }
    }


    fn update(&mut self, args: &UpdateArgs) 
    {
        self.rotation += 0.0 * args.dt; // Change this value to a non-zero number to make the sprite rotate.
        unsafe
        {
            if x >= 275.0
            {
                if y == -175.0
                {
                    if printedAlready == false
                    {
                            println!("YOLO"); // This is to establish that the player has reached the opposite side of the box.
                            printedAlready = true;
                    }
                }
            }
            if x < 275.0
            {
                printedAlready = false; // this allows the player to win multiple times
            }
        }
    /* 
        unsafe
        {
           println!("current x value is {}, current y value is {}", x,y); //This code is for debugging.
        }
    */       
    }

    fn key_press(&mut self, _args: &KeyPressArgs) // These conditions involving x and y make the goalposts "solid," so the player cannot go through them.
    {
        if _args.key == keyboard::Up
        {
            unsafe
            {
                if x > 225.0
                {
                    if y > -75.0 
                    {
                        y -= 10.0;
                    }
                    if y <= -275.0
                    {
                        y -= 10.0;
                    }
                }
                if x <= 225.0
                {
                    y -= 10.0;
                }
            }
        }
        if _args.key == keyboard::Down
        {
            unsafe
            {
                if x > 225.0
                {
                    if y < -275.0
                    {
                        y += 10.0;
                    }
                    if y >= -75.0
                    {
                        y += 10.0;
                    }
                }
                if x <= 225.0
                {
                    y += 10.0;
                }
            }
        }
        if _args.key == keyboard::Left
        {
            unsafe
            {
//                println!("LEEEEEEEEEEEEEEROYYYYYYYYYYYYY JENKIIIIIIIIIIIIIIIIIINS");
                x -= 10.0;
//                timer::sleep(1000);
            }
        }
        if _args.key == keyboard::Right
        {
            unsafe
            {
                if x < 275.0
                {
                    if x >= 225.0
                    {
                        if y <= -275.0
                        {
                            x += 10.0;
                        }
                        if y >= -75.0
                        {
                            x += 10.0;
                        }
                        if y == -175.0
                        {
                            x += 10.0;
                        }
                    }   
                    if x < 225.0
                    {
                        x += 10.0;
                    }
                }
            }
        }
    }
}

fn main()
{
    let mut window = Sdl2Window::new(
        WindowSettings
        {
            title: "YOLO".to_string(),
            size: [600, 600],
            fullscreen: false,
            exit_on_esc: true
        }
    );

    let game_iter_settings = GameIteratorSettings
    {
        updates_per_second: 60,
        max_frames_per_second: 60
    };

    let mut app = App { gl: Gl::new(), rotation: 0.0 };
    
    app.run(&mut window, &game_iter_settings);
}
