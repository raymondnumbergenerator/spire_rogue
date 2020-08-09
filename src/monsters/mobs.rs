use specs::prelude::*;
use specs::saveload::{SimpleMarker, MarkedBuilder};
use rltk::{RGB, RandomNumberGenerator};

use super::super::{
    Name, Position, Renderable,
    creature, monsters, saveload,
};

fn build_monster<S: ToString>(ecs: &mut World, name: S, x: i32, y: i32, glyph: rltk::FontCharType, fg: RGB) -> EntityBuilder {
    ecs.create_entity()
        .with(Name{ name: name.to_string() })
        .with(Position{ x, y })
        .with(Renderable{
            glyph,
            fg,
            bg: RGB::named(rltk::BLACK),
            render_order: 1
        })
        .with(creature::Creature{})
        .with(creature::Monster{})
        .with(creature::Viewshed{ visible_tiles: Vec::new(), range: 8, dirty: true})
        .with(creature::BlocksTile{})
        .marked::<SimpleMarker<saveload::SerializeMe>>()
}

fn cultist(ecs: &mut World, x: i32, y: i32) {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(48, 55);
    let mut attack_cycle: Vec<monsters::Attacks> = Vec::new();
    attack_cycle.push(monsters::Attacks::NormalAttack{
        name: "Dark Strike".to_string(),
        amount: 6,
        range: 1});
    let intent = monsters::build_attack(ecs, attack_cycle[0].clone()).build();

    build_monster(ecs, "Cultist", x, y, rltk::to_cp437('c'), RGB::named(rltk::RED))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, dexterity: 0, strength: 0, block: 0 })
        .with(creature::AttackCycle{ attacks: attack_cycle, cycle: 0 })
        .with(creature::Intent{ intent, used: false })
        .build();
}

fn acid_slime_s(ecs: &mut World, x: i32, y: i32) {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(8, 12);
    let mut attack_cycle: Vec<monsters::Attacks> = Vec::new();
    attack_cycle.push(monsters::Attacks::NormalAttack{
        name: "Tackle".to_string(),
        amount: 3,
        range: 1});
    attack_cycle.push(monsters::Attacks::ApplyWeak{
        name: "Lick".to_string(),
        turns: 1,
        range: 1});
    let intent = monsters::build_attack(ecs, attack_cycle[0].clone()).build();

    build_monster(ecs, "Acid Slime", x, y, rltk::to_cp437('l'), RGB::named(rltk::RED))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, dexterity: 0, strength: 0, block: 0 })
        .with(creature::AttackCycle{ attacks: attack_cycle, cycle: 0 })
        .with(creature::Intent{ intent, used: false })
        .build();
}

fn jaw_worm(ecs: &mut World, x: i32, y: i32) {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(40, 45);
    let mut attack_cycle: Vec<monsters::Attacks> = Vec::new();
    attack_cycle.push(monsters::Attacks::NormalAttack{
        name: "Chomp".to_string(),
        amount: 11,
        range: 1});
    attack_cycle.push(monsters::Attacks::AttackAndBlock{
        name: "Thrash".to_string(),
        damage_amount: 7,
        block_amount: 5,
        range: 1});
    let intent = monsters::build_attack(ecs, attack_cycle[0].clone()).build();

    build_monster(ecs, "Jaw Worm", x, y, rltk::to_cp437('j'), RGB::named(rltk::RED))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, dexterity: 0, strength: 0, block: 0 })
        .with(creature::AttackCycle{ attacks: attack_cycle, cycle: 0 })
        .with(creature::Intent{ intent, used: false })
        .build();
}

pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let roll: i32;
    {
        let mut rng = ecs.write_resource::<RandomNumberGenerator>();
        roll = rng.roll_dice(1, 3);
    }

    match roll {
        1 => { cultist(ecs, x, y) }
        2 => { acid_slime_s(ecs, x, y) }
        _ => { jaw_worm(ecs, x, y) }
    }
}