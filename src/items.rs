use std::fmt;
use std::iter;

#[derive(Clone)]
pub struct Item
{
    pub name: &'static str,
    description: &'static str,
    pub value: u32,
    pub damage: i32,
}

impl Item
{
    fn new(name: &'static str, description: &'static str, value: u32, damage: i32) -> Item
    {
        Item
        {
            name: name,
            description: description,
            value: value,
            damage: damage,
        }
    }

    pub fn empty_item() -> Item
    {
        Item::new("-", "-", 0, 0)
    }

    pub fn new_gold(value: u32) -> Item
    {
        Item::new("Gold", "It is a round coin with 5 stamped on it", value, 0)
    }

    pub fn new_5_gold() -> Item
    {
        Item::new_gold(5)
    }

    pub fn new_dagger() -> Item
    {
        Item::new("Dagger", "A small dagger with some rust. Somewhat more dangerous than a rock.", 10, 10)
    }

    pub fn new_rock() -> Item
    {
        Item::new("Rock", "A fist-sized rock, suitable for bludgeoning.", 0, 5)
    }

    pub fn get_damage(&self) -> i32
    {
        self.damage
    }

    pub fn get_name(&self) -> &'static str
    {
        self.name
    }
}

impl fmt::Debug for Item
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Item
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "\n{name}\n{underline}\n{description}\nValue: {value}",
               name=self.name,
               underline=iter::repeat("=").take(self.name.len()).collect::<String>(),
               description=self.description,
               value=self.value)
    }
}
