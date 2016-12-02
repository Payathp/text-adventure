use enemies;
use items;
use tiles;
use world;
use actions;


#[derive(Debug)]
pub struct Player
{
    pub inventory: Vec<items::Item>,
    pub hp: i32,
    pub position: tiles::Position,
    pub victory: bool,
}

impl Player
{
    fn new(inventory: Vec<items::Item>, hp: i32, x: i32, y: i32, victory: bool) -> Player
    {
        Player
        {
            inventory: inventory,
            hp: hp,
            position: tiles::Position::new(x, y),
            victory: false,
        }
    }

    pub fn make_player(map: &Vec<tiles::Room>) -> Player
    {
        // integrate map
        let pos = world::starting_room(map);
        let mut res = Player::new(Vec::with_capacity(100), 100, pos.x, pos.y, false);
        res.inventory.push(items::Item::new_gold(15));
        return res;
    }

    pub fn is_alive(&self) -> bool
    {
        self.hp > 0
    }

    pub fn print_inventory(&mut self)
    {
        println!("{:?}", self.inventory);
    }

    fn move_to(&mut self, map: &mut Vec<tiles::Room>, dx: i32, dy: i32)
    {
        self.position.x += dx;
        self.position.y += dy;
        // integrate map
        println!("{}",  world::tile_exists(map, self.position.x, self.position. y).unwrap().intro_text());
    }

    pub fn move_east(&mut self, map: &mut Vec<tiles::Room>)
    {
        self.move_to(map, 1, 0);
    }

    pub fn move_west(&mut self, map: &mut Vec<tiles::Room>)
    {
        self.move_to(map, -1, 0);
    }

    pub fn move_north(&mut self, map: &mut Vec<tiles::Room>)
    {
        self.move_to(map, 0, -1);
    }

    pub fn move_south(&mut self, map: &mut Vec<tiles::Room>)
    {
        self.move_to(map, 0, 1);
    }

    pub fn current_position(&self) -> &tiles::Position
    {
        &self.position
    }

    pub fn attack(&mut self, enemy: &enemies::Enemy)
    {
        let mut best_weapon = &items::Item::empty_item();
        for item in &self.inventory
        {
            if item.get_damage() > best_weapon.get_damage()
            {
                best_weapon = item;
            }
        }

        println!("You use {} against {}!", best_weapon, enemy.get_name());
        enemy.inflict_damage(best_weapon.damage);
        if !enemy.is_alive()
        {
            println!("You killed {}!", enemy.get_name());
        }
        else
        {
            println!("{} HP is {}", enemy.get_name(), enemy.hp);
        }
    }

    pub fn do_action<'a>(&mut self, map: &mut Vec<tiles::Room>, action: &'a str)
    {
        let mut action_method = actions::Action::identify_action_with_enemies(action, self, map);
    }

    pub fn inflict_damage(&mut self, damage: i32)
    {
        self.hp -= damage;
    }
}
