extern crate tcod;


use tcod::console::*;
use tcod::colors;
use tcod::colors::Color;

// actual size of the window
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
//const MAP_WIDTH: i32 = 80;
//const MAP_HEIGHT: i32 = 45;

const LIMIT_FPS: i32 = 20;  // 20 frames-per-second maximum

struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }

    ///move by the given amount
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    ///set the color and draw character that represents this object at this pos
    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }
    /// Erase the character that represents this object
    pub fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}

fn handle_keys(root: &mut Root, player: &mut Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Enter, alt: true, .. } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,  // exit game

        // movement keys
        Key { code: Up, .. } => player.move_by(0, -1),
        Key { code: Down, .. } => player.move_by(0, 1),
        Key { code: Left, .. } => player.move_by(-1, 0),
        Key { code: Right, .. } => player.move_by(1, 0),
        Key { printable: 'k', ..} => player.move_by(0, -1),   //up
        Key { printable: 'j', .. } => player.move_by(0, 1), //down
        Key { printable: 'h', .. } => player.move_by(-1, 0), //left
        Key { printable: 'l', .. } => player.move_by(1, 0), //right
        Key { printable: 'u', .. } => player.move_by(1, -1), // up right
        Key { printable: 'y', .. } => player.move_by(-1, -1), // up left
        Key { printable: 'b', .. } => player.move_by(-1, 1), // down left
        Key { printable: 'n', .. } => player.move_by(1, 1),

        _ => {}, // the _ means explicitly everything else does nothing.
    }

    false
}

fn main() {
    let mut root = Root::initializer()
        .font("terminal8x8_gs_tc.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .init();

    tcod::system::set_fps(LIMIT_FPS);
    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);
    let npc = Object::new(SCREEN_WIDTH /2-5, SCREEN_HEIGHT /2, '@', colors::YELLOW);
    let mut objects = [player, npc];

    while !root.window_closed() {
        for object in &objects {
            object.draw(&mut con);
        }
        blit(&mut con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT), &mut root, (0, 0), 1.0, 1.0);
        root.flush();


        // erase all objects at their old locations, before they move
        for object in &objects {
            object.clear(&mut con)
        };

        // handle keys and exit game if needed
        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player);
        if exit {
            break
        }
    }
}