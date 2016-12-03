use std::fmt;
use items;
use enemies;
use player;
use actions;
use world;

#[derive(Clone)]
pub struct Position
{
    pub x: i32,
    pub y: i32,
}

impl Position
{
    pub fn new(x: i32, y: i32) -> Position
    {
        Position
        {
            x: x,
            y: y,
        }
    }
}

impl fmt::Debug for Position
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "X: {}, Y: {}", self.x, self.y)
    }
}

impl fmt::Display for Position
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{x}, {y}", x=self.x, y=self.y)
    }
}

#[derive(Clone)]
pub struct Room
{
    pub position: Position,
    pub name: &'static str,
    pub enemy: enemies::Enemy,
    pub item: items::Item,
}

impl Room
{
    fn new(x: i32, y: i32, name: &'static str, enemy: enemies::Enemy, item: items::Item) -> Room
    {
        Room
        {
            position: Position::new(x, y),
            name: name,
            enemy: enemy,
            item: item,
        }
    }

    pub fn empty_room(x: i32, y: i32) -> Room
    {
        Room::new(x, y, "EMPTY", enemies::Enemy::empty_enemy(), items::Item::empty_item())
    }

    pub fn make_leave_cave_room(x: i32, y: i32) -> Room
    {
        Room::new(x, y, "LeaveCaveRoom", enemies::Enemy::empty_enemy(), items::Item::empty_item())
    }

    pub fn make_find_gold_room(x: i32, y: i32) -> Room
    {
        Room::new(x, y, "Find5GoldRoom", enemies::Enemy::empty_enemy(), items::Item::new_5_gold())
    }

    pub fn make_giant_spider_room(x: i32, y: i32) -> Room
    {
        Room::new(x, y, "GiantSpiderRoom", enemies::Enemy::new_giant_spider(), items::Item::empty_item())
    }

    pub fn make_empty_cave_path(x: i32, y: i32) -> Room
    {
        Room::new(x, y, "EmptyCavePath", enemies::Enemy::empty_enemy(), items::Item::empty_item())
    }

    pub fn make_starting_room(x: i32, y: i32) -> Room
    {
        Room::new(x, y, "StartingRoom", enemies::Enemy::empty_enemy(), items::Item::empty_item())
    }

    pub fn make_find_dagger_room(x: i32, y: i32) -> Room
    {
        Room::new(x, y, "FindDaggerRoom", enemies::Enemy::empty_enemy(), items::Item::new_dagger())
    }

    pub fn make_snake_pit_room(x: i32, y: i32) -> Room
    {
        Room::new(x, y, "SnakePitRoom", enemies::Enemy::empty_enemy(), items::Item::empty_item())
    }

    pub fn make_room_from_str<'a>(x: i32, y: i32, room_name: &'a str) -> Room
    {
        match room_name
        {
            "" => return Room::empty_room(x, y),
            "StartingRoom" => return Room::make_starting_room(x, y),
            "FindDaggerRoom" => return Room::make_find_dagger_room(x, y),
            "SnakePitRoom" => return Room::make_snake_pit_room(x, y),
            "Find5GoldRoom" => return Room::make_find_gold_room(x, y),
            "GiantSpiderRoom" => return Room::make_giant_spider_room(x, y),
            "EmptyCavePath" => return Room::make_empty_cave_path(x, y),
            "LeaveCaveRoom" => return Room::make_leave_cave_room(x, y),
            x => panic!("Unknown Room: {} .", x),
        }
    }

    pub fn intro_text(&self) -> &'static str
    {
        match self.name
        {
            "" => return "",
            "StartingRoom" =>
                return "You find yourself in a cave with a flickering toch on the wall.\
                        You can make out four paths, each equally as dark and foreboding.",
            "FindDaggerRoom" =>
                return "You notice something shiny in the corner.\
                        It's a dagger! You pick it up.",
            "SnakePitRoom" =>
                return "You have fallen into a pit of deadly snakes!\n\
                        You have died!",
            "Find5GoldRoom" =>
                return "Someone dropped a 5 gold piece. You pick it up",
            "GiantSpiderRoom" =>
            {
                if self.enemy.is_alive()
                {
                    return "A giant spider jumps dowm from its web in front of you!"
                }
                else
                {
                    return "The corps of a dead spider rots on the ground."
                }
            },
            "EmptyCavePath" =>
                return "Another unremarkable part of the cave. You must forge onwards.",
            "LeaveCaveRoom" =>
                return "You see a bright light in the distance ...\
                        ... it grows as you get closer! It's sunlight\n\
                        Victory is yours!",
            x => panic!("Unknown Room: {} ", x),
        }
    }

    pub fn modify_player(&mut self, player: &mut player::Player)
    {
        match self.name
        {
            "FindDaggerRoom" =>
            {
                player.inventory.push(items::Item::new_dagger());
            },
            "SnakePitRoom" =>
            {
                player.hp = 0;
            },
            "Find5GoldRoom" =>
            {
                player.inventory[0].value += 5;
            },
            "GiantSpiderRoom" =>
            {
                if self.enemy.is_alive()
                {
                    player.inflict_damage(self.enemy.damage);
                    println!("Enemy does {} damage. You have {} HP remaining.", self.enemy.damage, player.hp);
                }
            },
            "LeaveCaveRoom" =>
            {
                player.victory = true;
            },
            "EmptyCavePath"|"StartingRoom" =>
            {
                // TODO
            },
            _ => panic!("Not implemented!")
        }
    }

    pub fn get_name(&self) -> &'static str
    {
        self.name
    }

    pub fn not_empty_tile(&self) -> bool
    {
        self.name != "EMPTY"
    }

    pub fn adjacent_moves(&self, map: &Vec<Room>) -> Vec<actions::Action>
    {
        let mut moves: Vec<actions::Action> = Vec::with_capacity(4);
        if (self.position.x+1 <= world::get_x_limit(map)) & world::tile_in_existence(map, self.position.x+1, self.position.y)
        {
            moves.push(actions::Action::new("Move east", "e"));
        }
        if (self.position.x-1 >= 0) & world::tile_in_existence(map, self.position.x-1, self.position.y)
        {
            moves.push(actions::Action::new("Move west", "w"));
        }
        if (self.position.y-1 >= 0) & world::tile_in_existence(map, self.position.x, self.position.y-1)
        {
            moves.push(actions::Action::new("Move north", "n"));
        }
        if (self.position.y <= world::get_y_limit(map)) & world::tile_in_existence(map, self.position.x, self.position.y+1)
        {
            moves.push(actions::Action::new("Move south", "s"));
        }

        return moves;
    }

    pub fn available_actions(&mut self, map: &Vec<Room>) -> Vec<actions::Action>
    {
        let mut moves = Vec::new();

        if self.enemy.is_alive()
        {
            moves.push(actions::Action::new("Attack", "a"));
            return moves;
        }
        else
        {
            moves =self.adjacent_moves(map);
            moves.push(actions::Action::new("View inventory", "i"));
            return moves;
        }
    }

}

impl fmt::Debug for Room
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "\n[{}, ({:?}), E: {}, I: {:?}]\n", self.name, self.position, self.enemy, self.item)
    }
}

impl fmt::Display for Room
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.name)
    }
}
