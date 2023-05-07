#[allow(unused)]
use bevy::prelude::*;

use crate::modul::GamePlugin;
use crate::menu::MenuPlugin;

#[path = "game/modul.rs"]
mod modul;
#[path = "menu/menu.rs"]
mod menu;

// [2]
#[path = "game/components.rs"]
pub mod components;
pub mod constants;


fn main() {
    App::new()
        .add_plugin(MenuPlugin)
        .add_plugin(GamePlugin)
        .run();
}

/*

Useful references

[1]
Titel = 'How do I check if a thing is in a vector'
url =https://stackoverflow.com/questions/58368801/how-do-i-check-if-a-thing-is-in-a-vector

[2]

I have the mod components here because I want to use in several modules. 
Then I can use it with use crate::components::...
If I would have it in the game/modul.rs
 I would have to use use crate::modul::components::

*/

