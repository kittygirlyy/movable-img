extern crate piston_window;

use piston_window::*;

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Chat miaou", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();

let cat_texture = Texture::from_path(
    &mut window.create_texture_context(),
    "assets/cat.png",
    Flip::None,
    &TextureSettings::new()
).unwrap();



    let mut cat_position = [0.0, 0.0];
    while let Some(event) = window.next() {
        window.draw_2d(&event, |context, graphics, _device| {
            clear([1.0, 1.0, 1.0, 1.0], graphics);

            image(&cat_texture, context.transform.trans(cat_position[0], cat_position[1]), graphics);
        });

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::Up => cat_position[1] -= 5.0,
                Key::Down => cat_position[1] += 5.0,
                Key::Left => cat_position[0] -= 5.0,
                Key::Right => cat_position[0] += 5.0,
                Key::Space => {
                    println!("meow");
                }
                _ => {}
            }
        }
    }
}