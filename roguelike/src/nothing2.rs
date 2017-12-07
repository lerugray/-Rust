fn handle_keys(root: &mut Root, player_x: &mut i32, player_y: &mut i32) -> bool {
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
        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => *player_x -= 1,
        Key { code: Right, .. } => *player_x += 1,
        Key { printable: 'k', ..} => *player_y -= 1,   //up
        Key { printable: 'j', .. } => *player_y += 1, //down
        Key { printable: 'h', .. } => *player_x -= 1, //left
        Key { printable: 'l', .. } => *player_x += 1, //right
        Key { printable: 'u', .. } => { //up right
            *player_y -= 1;
            *player_x += 1;
        }
        Key { printable: 'y', .. } => { //up left
            *player_y -= 1;
            *player_x -= 1;
        }
        Key { printable: 'b', .. } => { // down left
            *player_y += 1;
            *player_x -= 1;
        }
        Key {printable: 'n', .. } => { //down right
            *player_y += 1;
            *player_x += 1;
        }
        _ => {}, // the _ means basically except everything else.
    }

    false
}