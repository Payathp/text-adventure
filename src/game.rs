use std::io;
use world;
use player;
use actions;
use tiles;

/*
pub struct GameManager
{
    player: player::Player,
    map: Vec<tiles::Room>,
    available_actions: Vec<actions::Action>,
}

impl GameManager
{
    fn new(player: player::Player, map: Vec<tiles::Room>, available_actions: Vec<actions::Action>) -> GameManager
    {
        GameManager
        {
            player: player,
            map: map,
            available_actions: available_actions,
        }
    }

    fn init_new_game() -> GameManager
    {
        let mut map = world::load_default();
        let mut player = player::Player::make_player(&map);
        let mut available_actions = tiles::Room::  player.current_position();
        GameManager::new(player::Player::make_player(&map), map, )
    }
}*/

pub fn play()
{
    let mut map = world::load_default();
    let mut current_player = player::Player::make_player(&map);
    while current_player.is_alive() & !current_player.victory
    {
        let mut current_room = &mut world::tile_exists(&map, current_player.position.x, current_player.position.y).unwrap().clone();
        current_room.modify_player(&mut current_player);
        if current_player.is_alive() & !current_player.victory
        {
            println!("Choose an action:");
            let available_actions = current_room.available_actions(&map);
            for action in &available_actions
            {
                println!("{}", action);
            }

            'inputloop: loop
            {
                let mut action_input = String::new();
                match io::stdin().read_line(&mut action_input)
                {
                    Ok(_) =>
                    {
                        for action in available_actions.clone()
                        {
                            if action.hotkey == action_input.trim()
                            {
                                match actions::Action::identify_action_with_enemies(&action_input.trim(), &mut current_player, &mut map)
                                {
                                    true => break 'inputloop,
                                    _ => continue 'inputloop,
                                }
                            }
                        }/*
                        match actions::Action::identify_action_with_enemies(&action_input.trim(), &mut current_player, &mut map)
                        {
                            true => break 'inputloop,
                            false => continue 'inputloop,
                    }*/
                        println!("This is not possible in this room. Try something else.");

                    },
                    Err(_) =>
                    {
                        println!("This is not possible in this room. Try something else.");
                        continue
                    },
                }
            }
        }
    }
}
