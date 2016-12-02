use std::io;
use world;
use player;
use actions;
use tiles;

pub fn play()
{
    let mut map = world::load_default();
    let mut current_player = player::Player::make_player(&map);
    while current_player.is_alive() & !current_player.victory
    {
        let mut current_room = tiles::Room::empty_room(current_player.position.x, current_player.position.y);
        {
            current_room = world::tile_exists(&map, current_player.position.x, current_player.position.y).unwrap().clone();
        }
        current_room.modify_player(&mut current_player);
        if current_player.is_alive() & !current_player.victory
        {
            println!("Choose an action:");
            let available_actions = current_room.available_actions(&map);
            for action in available_actions
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
                        match actions::Action::identify_action_with_enemies(&action_input.trim(), &mut current_player, &mut map)
                        {
                            true => break 'inputloop,
                            false => continue 'inputloop,
                        }
                    }
                    Err(_) => continue,
                }
            }
        }
    }
}
