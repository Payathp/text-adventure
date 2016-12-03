use std::io::prelude::*;
use std::fs::File;
use std::iter::Iterator;
use tiles;


pub fn load_map_from_file<'a>(file_name: &'a str) -> Vec<tiles::Room>
{
    let mut file = File::open(file_name).unwrap();
    let mut container = String::new();
    file.read_to_string(&mut container).unwrap();
    process(&container[ .. ])
}

pub fn load_default() -> Vec<tiles::Room>
{
    load_map_from_file("resources/map")
}

pub fn dimensions(map: &str) -> (u32, u32)
{
    let height = count_occurence(map, '\n');
    let width = (count_occurence(map, ';') / height) as u32;
    return (width, height);
}

pub fn need_size<'a>(map: &'a str) -> u32
{
    let res = dimensions(map);
    return res.0 * res.1;
}

pub fn process<'a>(map: &'a str) -> Vec<tiles::Room>
{
    let mut map_display: Vec<tiles::Room> = Vec::new();
    map_display.reserve(need_size(&map) as usize);
    let value_vec: Vec<&str> = map.split(';').collect();
    let mut y = 0;
    let mut x = 0;
    let sem_round = count_occurence(&map, ';');
    let mut iter = value_vec.into_iter();
    for _ in 1..sem_round
    {
        let current_str = iter.next().unwrap();
        if current_str.contains("\n")
        {
            x = 0;
            y += 1;
        }
        let current_str_raw = &current_str.replace("\n", "");
        map_display.push(tiles::Room::make_room_from_str(x, y, current_str_raw));
        x += 1;
    }

    return map_display;
}

pub fn count_occurence<'a>(map: &'a str, sym: char) -> u32
{
    let map_chars = map.chars();
    let mut res = 0;
    for c in map_chars
    {
        if c == sym
        {
            res += 1;
        }
    }
    return res;
}

pub fn tile_exists(map: &Vec<tiles::Room>, x: i32, y: i32) -> Option<&tiles::Room>
{
    for room in map
    {
        let room_x = room.position.x;
        let room_y = room.position.y;
        let room_name = room.name;
        let room_copy = room;
        {
            if (room_x == x) & (room_y == y) & (room_name != "EMPTY")
            {
                return Some(room_copy);
            }
        }
    }

    return None;
}

pub fn mut_tile_exists(map: &mut Vec<tiles::Room>, x: i32, y: i32) -> Option<&mut tiles::Room>
{
    for room in map
    {
        let room_x = room.position.x;
        let room_y = room.position.y;
        let room_name = room.name;
        {
            if (room_x == x) & (room_y == y) & (room_name != "EMPTY")
            {
                return Some(room);
            }
        }
    }
    return None;
}

pub fn tile_in_existence(map: &Vec<tiles::Room>, x: i32, y: i32) -> bool
{
    match tile_exists(map, x, y)
    {
        Some(_) => return true,
        _ => return false,
    }
}

pub fn get_x_limit(map: &Vec<tiles::Room>) -> i32
{
    let mut biggest_x = 0;
    for room in map
    {
        if biggest_x < room.position.x
        {
            biggest_x = room.position.x;
        }
    }

    return biggest_x;
}

pub fn get_y_limit(map: &Vec<tiles::Room>) -> i32
{
    let mut biggest_y = 0;
    for room in map
    {
        if biggest_y < room.position.y
        {
            biggest_y = room.position.y;
        }
    }

    return biggest_y;
}

pub fn starting_room(map: &Vec<tiles::Room>) -> &tiles::Position
{
    for tile in map
    {
        if tile.name == "StartingRoom"
        {
            return &tile.position;
        }
    }

    panic!("No StartingRoom found!");
}
