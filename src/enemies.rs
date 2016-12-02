use std::fmt;


#[derive(Clone)]
pub struct Enemy
{
    name: &'static str,
    pub hp: i32,
    pub damage: i32,
}

impl Enemy
{
    fn new(name: &'static str, hp: i32, damage: i32) -> Enemy
    {
        Enemy
        {
            name: name,
            hp: hp,
            damage: damage,
        }
    }

    pub fn empty_enemy() -> Enemy
    {
        Enemy::new("-", 0, 0)
    }

    pub fn new_giant_spider() -> Enemy
    {
        Enemy::new("Giant Spider", 10, 2)
    }

    pub fn new_ogre() -> Enemy
    {
        Enemy::new("Ogre", 30, 15)
    }

    pub fn is_alive(&self) -> bool
    {
        self.hp > 0
    }

    pub fn get_name<'a>(&self) -> &'a str
    {
        self.name
    }

    pub fn inflict_damage(&self, damage: i32) -> Enemy
    {
        Enemy::new(self.name, self.hp-damage, self.damage)
    }
}



impl fmt::Debug for Enemy
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Enemy
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.name)
    }
}
