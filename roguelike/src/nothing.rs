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