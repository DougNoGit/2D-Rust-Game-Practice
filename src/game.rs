extern crate graphics;
extern crate piston;
extern crate sdl2_game_window;
extern crate opengl_graphics;

use sdl2_game_window::GameWindowSDL2;
use opengl_graphics::Gl;

use piston::
{
    Game,
    GameWindowSettings,
    GameIteratorSettings,
    RenderArgs,
    UpdateArgs,
    keyboard,
    KeyPressArgs
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

impl Game for App
{
    fn render(&mut self, args: &RenderArgs)
    {
        let context = &Context::abs(args.width as f64, args.height as f64);
        context.rgba(0.0,0.0,0.1,0.1).draw(&mut self.gl);

        unsafe
        {
        context
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
                if printedAlready == false
                {
                    println!("YOLO"); // This is to establish that the player has reached the opposite side of the box.
                    printedAlready = true;
                }
            }
        }
    }

    fn key_press(&mut self, _args: &KeyPressArgs)
    {
        if _args.key == keyboard::Up
        {
            unsafe
            {
                y -= 10.0;
            }
        }
        if _args.key == keyboard::Down
        {
            unsafe
            {
                y += 10.0;
            }
        }
        if _args.key == keyboard::Left
        {
            unsafe
            {
                x -= 10.0;
            }
        }
        if _args.key == keyboard::Right
        {
            unsafe
            {
                x += 10.0;
            }
        }
    }

}

fn main()
{
    let mut window = GameWindowSDL2::new(
        GameWindowSettings
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
