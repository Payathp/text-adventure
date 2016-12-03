use std::fmt;
use player;
use enemies;
use world;
use tiles;


#[derive(Debug, Clone)]
pub struct Action
{
    pub name: &'static str,
    pub hotkey: &'static str,
    pub enemy: enemies::Enemy,
    //args: Vec<()>,
}

impl Action
{
    pub fn new(name: &'static str, hotkey: &'static str) -> Action
    {
        Action
        {
            name: name,
            hotkey: hotkey,
            enemy: enemies::Enemy::empty_enemy(),
        }
    }

    pub fn identify_action_with_enemies<'a>(action: &'a str, player: &mut player::Player, map: &mut Vec<tiles::Room>) -> bool
    {
        match action
        {
            "n"|"N"|"Move north"|"move north" => {
                player.move_north(map);
                return true;
            },
            "s"|"S"|"Move south"|"move south" =>
            {
                player.move_south(map);
                return true;
            },
            "e"|"E"|"Move east"|"move east" =>
            {
                player.move_east(map);
                return true;
            },
            "w"|"W"|"Move west"|"move west" =>
            {
                player.move_west(map);
                return true;
            },
            "i"|"I"|"View inventory"|"view inventory" =>
            {
                player.print_inventory();
                return true;
            },
            "a"|"A"|"Attack"|"attack" => {

                // PROBLEM
                let p_x = player.position.x;
                let p_y = player.position.y;
                player.attack(world::mut_tile_exists(map, p_x, p_y).unwrap());
                return true;
            },
            _ =>
            {
                println!("Bad input");
                return false;

            }
        }
    }
}

impl fmt::Display for Action
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}: {}", self.hotkey, self.name)
    }
}
