use specs::prelude::*;
use specs::saveload::{SimpleMarker, MarkedBuilder};
use rltk::{RGB, RandomNumberGenerator};

use super::super::{
    Name, Position, Renderable,
    creature, effects, monsters, saveload,
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

pub fn cultist(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(48, 55);

    let attack_incantation = monsters::Attacks::BuffStrength{
        name: "Incantation".to_string(),
        amount: 4,
        range: 2
    };
    let attack_dark_strike = monsters::Attacks::NormalAttack{
        name: "Dark Strike".to_string(),
        amount: 6,
        range: 1
    };
    let intent = attack_incantation.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_weighted()
        .add_weighted(attack_incantation, 1)
        .add_weighted(attack_dark_strike, 4);

    build_monster(ecs, "Cultist", x, y, rltk::to_cp437('c'), RGB::named(rltk::RED))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn jaw_worm(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(40, 45);

    let attack_chomp = monsters::Attacks::NormalAttack{
        name: "Chomp".to_string(),
        amount: 11,
        range: 1
    };
    let attack_thrash = monsters::Attacks::AttackAndBlock{
        name: "Thrash".to_string(),
        damage_amount: 7,
        block_amount: 5,
        range: 1
    };
    let attack_bellow = monsters::Attacks::BlockAndBuffStrength{
        name: "Bellow".to_string(),
        block_amount: 6,
        buff_amount: 3,
        range: 2
    };
    let intent = attack_chomp.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_weighted()
        .add_weighted(attack_chomp, 5)
        .add_weighted(attack_thrash, 6)
        .add_weighted(attack_bellow, 9);

    build_monster(ecs, "Jaw Worm", x, y, rltk::to_cp437('j'), RGB::named(rltk::RED))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn red_louse(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(10, 16);

    let attack_bite = monsters::Attacks::NormalAttack{
        name: "Bite".to_string(),
        amount: ecs.write_resource::<RandomNumberGenerator>().range(5, 8),
        range: 1
    };
    let attack_grow = monsters::Attacks::BuffStrength{
        name: "Grow".to_string(),
        amount: 4,
        range: 2
    };
    let intent = attack_bite.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_weighted()
        .add_weighted(attack_bite, 3)
        .add_weighted(attack_grow, 1);

    build_monster(ecs, "Louse", x, y, rltk::to_cp437('l'), RGB::named(rltk::RED))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn green_louse(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(10, 16);

    let attack_bite = monsters::Attacks::NormalAttack{
        name: "Bite".to_string(),
        amount: ecs.write_resource::<RandomNumberGenerator>().range(5, 8),
        range: 1
    };
    let attack_spit_web = monsters::Attacks::ApplyWeak{
        name: "Spit Web".to_string(),
        turns: 2,
        range: 2
    };
    let intent = attack_bite.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_weighted()
        .add_weighted(attack_bite, 3)
        .add_weighted(attack_spit_web, 1);

    build_monster(ecs, "Louse", x, y, rltk::to_cp437('l'), RGB::named(rltk::GREEN))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn acid_slime_l(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(65, 70);

    let attack_corrosive_spit = monsters::Attacks::AttackAndGiveCard{
        name: "Corrosive Spit".to_string(),
        amount: 11,
        card: effects::GainableCard::Slimed,
        number: 2,
        range: 2
    };
    let attack_lick = monsters::Attacks::ApplyWeak{
        name: "Lick".to_string(),
        turns: 2,
        range: 1
    };
    let attack_tackle = monsters::Attacks::NormalAttack{
        name: "Tackle".to_string(),
        amount: 16,
        range: 1
    };
    let intent = attack_corrosive_spit.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_weighted()
        .add_weighted(attack_corrosive_spit, 3)
        .add_weighted(attack_lick, 4)
        .add_weighted(attack_tackle, 3);

    build_monster(ecs, "Acid Slime", x, y, rltk::to_cp437('S'), RGB::named(rltk::GREEN))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn spike_slime_l(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(64, 71);

    let attack_flame_tackle = monsters::Attacks::AttackAndGiveCard{
        name: "Flame Tackle".to_string(),
        amount: 16,
        card: effects::GainableCard::Slimed,
        number: 2,
        range: 1
    };
    let attack_lick = monsters::Attacks::ApplyFrail{
        name: "Lick".to_string(),
        turns: 2,
        range: 1
    };
    let intent = attack_flame_tackle.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_weighted()
        .add_weighted(attack_flame_tackle, 3)
        .add_weighted(attack_lick, 7);

    build_monster(ecs, "Spike Slime", x, y, rltk::to_cp437('S'), RGB::named(rltk::TEAL))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn acid_slime_m(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(28, 33);

    let attack_corrosive_spit = monsters::Attacks::AttackAndGiveCard{
        name: "Corrosive Spit".to_string(),
        amount: 7,
        card: effects::GainableCard::Slimed,
        number: 1,
        range: 1
    };
    let attack_lick = monsters::Attacks::ApplyWeak{
        name: "Lick".to_string(),
        turns: 1,
        range: 1
    };
    let attack_tackle = monsters::Attacks::NormalAttack{
        name: "Tackle".to_string(),
        amount: 10,
        range: 1
    };
    let intent = attack_corrosive_spit.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_weighted()
        .add_weighted(attack_corrosive_spit, 3)
        .add_weighted(attack_lick, 4)
        .add_weighted(attack_tackle, 3);

    build_monster(ecs, "Acid Slime", x, y, rltk::to_cp437('S'), RGB::named(rltk::GREEN))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn spike_slime_m(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(28, 33);

    let attack_flame_tackle = monsters::Attacks::AttackAndGiveCard{
        name: "Flame Tackle".to_string(),
        amount: 8,
        card: effects::GainableCard::Slimed,
        number: 1,
        range: 1
    };
    let attack_lick = monsters::Attacks::ApplyFrail{
        name: "Lick".to_string(),
        turns: 1,
        range: 1
    };
    let intent = attack_flame_tackle.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_weighted()
        .add_weighted(attack_flame_tackle, 3)
        .add_weighted(attack_lick, 7);

    build_monster(ecs, "Spike Slime", x, y, rltk::to_cp437('S'), RGB::named(rltk::TEAL))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn acid_slime_s(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(8, 13);

    let attack_tackle = monsters::Attacks::NormalAttack{
        name: "Tackle".to_string(),
        amount: 3,
        range: 1
    };
    let attack_lick = monsters::Attacks::ApplyWeak{
        name: "Lick".to_string(),
        turns: 1,
        range: 1
    };
    let intent = attack_tackle.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_sequential()
        .add_sequential(attack_tackle)
        .add_sequential(attack_lick);

    build_monster(ecs, "Acid Slime", x, y, rltk::to_cp437('s'), RGB::named(rltk::GREEN))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn spike_slime_s(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(8, 13);

    let attack_tackle = monsters::Attacks::NormalAttack{
        name: "Tackle".to_string(),
        amount: 5,
        range: 1
    };
    let intent = attack_tackle.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_sequential()
        .add_sequential(attack_tackle);

    build_monster(ecs, "Spike Slime", x, y, rltk::to_cp437('s'), RGB::named(rltk::TEAL))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn mad_gremlin(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(20, 25);

    let attack_scratch = monsters::Attacks::NormalAttack{
        name: "Scratch".to_string(),
        amount: 4,
        range: 1
    };
    let intent = attack_scratch.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_sequential()
        .add_sequential(attack_scratch);

    build_monster(ecs, "Mad Gremlin", x, y, rltk::to_cp437('g'), RGB::named(rltk::RED))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn sneaky_gremlin(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(10, 15);

    let attack_puncture = monsters::Attacks::NormalAttack{
        name: "Puncture".to_string(),
        amount: 4,
        range: 1
    };
    let intent = attack_puncture.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_sequential()
        .add_sequential(attack_puncture);

    build_monster(ecs, "Sneaky Gremlin", x, y, rltk::to_cp437('g'), RGB::named(rltk::PINK))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn fat_gremlin(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(13, 18);

    let attack_smash = monsters::Attacks::AttackAndApplyWeak{
        name: "Smash".to_string(),
        amount: 4,
        turns: 1,
        range: 1
    };
    let intent = attack_smash.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_sequential()
        .add_sequential(attack_smash);

    build_monster(ecs, "Fat Gremlin", x, y, rltk::to_cp437('g'), RGB::named(rltk::ORANGE))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn gremlin_wizard(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(23, 26);

    let attack_charging = monsters::Attacks::Pass{
        name: "Charging".to_string(),
        range: 2
    };
    let attack_ultimate_blast = monsters::Attacks::NormalAttack{
        name: "Ultimate Blast".to_string(),
        amount: 25,
        range: 2
    };
    let intent = attack_charging.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_sequential()
        .add_sequential(attack_charging.clone())
        .add_sequential(attack_charging.clone())
        .add_sequential(attack_charging.clone())
        .add_sequential(attack_ultimate_blast);

    build_monster(ecs, "Gremlin Wizard", x, y, rltk::to_cp437('G'), RGB::named(rltk::PURPLE))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn shield_gremlin(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(12, 16);

    let attack_protect = monsters::Attacks::GainBlock{
        name: "Protect".to_string(),
        amount: 7,
        range: 1
    };
    let attack_shield_bash = monsters::Attacks::AttackAndBlock{
        name: "Shield Bash".to_string(),
        damage_amount: 6,
        block_amount: 6,
        range: 1
    };
    let intent = attack_protect.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_weighted()
        .add_weighted(attack_protect, 2)
        .add_weighted(attack_shield_bash, 1);

    build_monster(ecs, "Shield Gremlin", x, y, rltk::to_cp437('G'), RGB::named(rltk::PINK))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn blue_slaver(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(46, 51);

    let attack_stab = monsters::Attacks::NormalAttack{
        name: "Stab".to_string(),
        amount: 12,
        range: 1
    };
    let attack_rake = monsters::Attacks::AttackAndApplyWeak{
        name: "Rake".to_string(),
        amount: 7,
        turns: 1,
        range: 1
    };
    let intent = attack_stab.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_weighted()
        .add_weighted(attack_stab, 3)
        .add_weighted(attack_rake, 2);

    build_monster(ecs, "Slaver", x, y, rltk::to_cp437('l'), RGB::named(rltk::BLUE))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}

pub fn red_slaver(ecs: &mut World, x: i32, y: i32) -> Entity {
    let hp = ecs.write_resource::<RandomNumberGenerator>().range(46, 51);

    let attack_stab = monsters::Attacks::NormalAttack{
        name: "Stab".to_string(),
        amount: 13,
        range: 1
    };
    let attack_scrape = monsters::Attacks::AttackAndApplyVulnerable{
        name: "Scrape".to_string(),
        amount: 8,
        turns: 1,
        range: 1
    };
    let intent = attack_stab.clone().to_attack(ecs);

    let attack_cycle = creature::AttackCycle::new_weighted()
        .add_weighted(attack_stab, 3)
        .add_weighted(attack_scrape, 2);

    build_monster(ecs, "Slaver", x, y, rltk::to_cp437('l'), RGB::named(rltk::RED))
        .with(creature::CombatStats{ max_hp: hp, hp: hp, block: 0,
            base_strength: 0, strength: 0,
            base_dexterity: 0, dexterity: 0
        })
        .with(attack_cycle)
        .with(creature::Intent{ intent, used: false })
        .build()
}