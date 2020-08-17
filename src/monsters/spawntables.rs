use super::Encounters;

use super::super::{util::RandomTable};

pub fn spawn_table(floor: i32) -> RandomTable<Encounters> {
    match floor {
        1 => {
            RandomTable::new()
                .add(Encounters::Cultist(1), 1)
                .add(Encounters::JawWorm(1), 1)
                .add(Encounters::Louses(2), 1)
                .add(Encounters::SlimePair, 1)
        }
        _ => {
            RandomTable::new()
                .add(Encounters::GremlinGang(4), 1)
                .add(Encounters::SmallSlimes(5), 1)
                .add(Encounters::Louses(3), 1)
                .add(Encounters::LargeSlime, 1)
                .add(Encounters::Slaver(1), 2)
        }
    }
}