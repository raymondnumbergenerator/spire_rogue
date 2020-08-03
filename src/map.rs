use specs::prelude::*;
use serde::{Serialize, Deserialize};
use rltk::{RGB, Rltk, RandomNumberGenerator, Algorithm2D, BaseMap, Point};

use super::{util::rect::Rect, util::line::line};
use super::{WINDOWWIDTH, WINDOWHEIGHT, gui::GUISIZE};

pub const MAPWIDTH: usize = WINDOWWIDTH;
pub const MAPHEIGHT: usize = WINDOWHEIGHT - GUISIZE;
// pub const MAPSIZE: usize = MAPHEIGHT * MAPWIDTH;

#[derive(PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,

    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    pub tile_content: Vec<Vec<Entity>>,
}

fn build_map(width: i32, height: i32, default_tile: TileType) -> Map {
    let size = (width * height) as usize;
    Map{
        tiles: vec![default_tile; size],
        rooms: Vec::new(),
        width: width,
        height: height,
        revealed_tiles: vec![false; size],
        visible_tiles: vec![false; size],
        blocked: vec![false; size],
        tile_content: vec![Vec::new(); size],
    }
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    pub fn populate_blocked(&mut self) {

        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    fn apply_room(&mut self, room: &Rect) {
        for y in room.y1 + 1 ..= room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }
    
    fn apply_euclidean_corridor(&mut self, x1: i32, y1: i32, x2: i32, y2: i32) {
        for (x, y) in line(x1, y1, x2, y2) {
            let idx = self.xy_idx(x, y);
            self.tiles[idx] = TileType::Floor;
        }
    }

    fn apply_manhattan_corridor(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, right: bool) {
        if right {
            self.apply_euclidean_corridor(x1, y1, x2, y1);
            self.apply_euclidean_corridor(x2, y1, x2, y2);
        } else {
            self.apply_euclidean_corridor(x1, y1, x1, y2);
            self.apply_euclidean_corridor(x1, y2, x2, y2);
        }
    }

    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = build_map(MAPWIDTH as i32, MAPHEIGHT as i32, TileType::Wall);
        
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE:i32 = 10;
    
        let mut rng = RandomNumberGenerator::new();
    
        for _ in 0 .. MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, map.width - w - 1) - 1;
            let y = rng.roll_dice(1, map.height - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) { ok = false }
            }
    
            if ok {
                map.apply_room(&new_room);
    
                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len()-1].center();
    
                    if rng.range(0, 2) == 0 {
                        map.apply_manhattan_corridor(new_x, new_y, prev_x, prev_y, true);
                    } else {
                        map.apply_manhattan_corridor(new_x, new_y, prev_x, prev_y, false);
                    }
                }
    
                map.rooms.push(new_room);
            }
        }
    
        map
    }

    pub fn new_map_field() -> Map {
        let mut map = build_map(MAPWIDTH as i32, MAPHEIGHT as i32, TileType::Floor);

        let room = Rect::new(0, 0, map.width, map.height);
        map.rooms.push(room);
    
        for x in 0 .. map.width {
            let idx_1 = map.xy_idx(x, 0);
            let idx_2 = map.xy_idx(x, map.height - 1);
            map.tiles[idx_1] = TileType::Wall;
            map.tiles[idx_2] = TileType::Wall;
        }
    
        for y in 0 .. map.height {
            let idx_1 = map.xy_idx(0, y);
            let idx_2 = map.xy_idx(map.width - 1, y);
            map.tiles[idx_1] = TileType::Wall;
            map.tiles[idx_2] = TileType::Wall;
        }
    
        let mut rng = RandomNumberGenerator::new();
    
        for _i in 0 .. (map.width * map.height) {
            let x = rng.roll_dice(1, map.width - 1);
            let y = rng.roll_dice(1, map.height - 1);
            let idx = map.xy_idx(x, y);
            let center = map.rooms[0].center();
            if idx != map.xy_idx(center.0, center.1) {
                map.tiles[idx] = TileType::Wall;
            }
        }

        map
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 { return false; }
        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx:usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;

        if self.is_exit_valid(x - 1, y) { exits.push((idx - 1, 1.0)) };
        if self.is_exit_valid(x + 1, y) { exits.push((idx + 1, 1.0)) };
        if self.is_exit_valid(x, y - 1) { exits.push((idx - w, 1.0)) };
        if self.is_exit_valid(x, y + 1) { exits.push((idx + w, 1.0)) };

        if self.is_exit_valid(x - 1, y - 1) { exits.push((idx - w - 1, 1.45)); }
        if self.is_exit_valid(x + 1, y - 1) { exits.push((idx - w + 1, 1.45)); }
        if self.is_exit_valid(x - 1, y + 1) { exits.push((idx + w - 1, 1.45)); }
        if self.is_exit_valid(x + 1, y + 1) { exits.push((idx + w + 1, 1.45)); }

        exits
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

pub fn draw_map(ecs: &World, ctx: &mut Rltk) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            match tile {
                TileType::Floor => {
                    glyph = rltk::to_cp437('.');
                    fg = RGB::from_f32(0.5, 0.5, 0.0);
                }
                TileType::Wall => {
                    glyph = rltk::to_cp437('#');
                    fg = RGB::from_f32(0.4, 0.4, 0.0);
                }
            }
            if !map.visible_tiles[idx] { fg = fg.to_greyscale() }
            ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph);
        }

        x += 1;
        if x > map.width - 1 {
            x = 0;
            y += 1;
        }
    }
}