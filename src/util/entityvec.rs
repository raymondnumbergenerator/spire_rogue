use specs::prelude::*;
use specs::saveload::{Marker, ConvertSaveload};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
pub struct EntityVec<T>{
    pub vec: Vec<T>
}

impl<T> EntityVec<T> {
    pub fn new() -> EntityVec<T> {
        EntityVec { vec: Vec::new() }
    }

    pub fn with_capacity(capacity: usize) -> EntityVec<T> {
        EntityVec { vec: Vec::with_capacity(capacity) }
    }

    pub fn with_existing(v: Vec<T>) -> EntityVec<T> {
        EntityVec { vec: v }
    }
}

impl<T> std::ops::Deref for EntityVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Vec<T> {
        &self.vec
    }
}

impl<T> std::ops::DerefMut for EntityVec<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.vec
    }
}

impl<C, M: Serialize + Marker> ConvertSaveload<M> for EntityVec<C>
    where for<'de> M: Deserialize<'de>,
    C: ConvertSaveload<M>
{
    type Data = Vec<<C as ConvertSaveload<M>>::Data>;
    type Error = <C as ConvertSaveload<M>>::Error;

    fn convert_into<F>(&self, mut ids: F) -> Result<Self::Data, Self::Error>
    where
        F: FnMut(Entity) -> Option<M>
    {
        let mut output = Vec::with_capacity(self.len());

        for item in self.iter() {
            let converted_item = item.convert_into(|entity| ids(entity))?;

            output.push(converted_item);            
        }

        Ok(output)
    }

    fn convert_from<F>(data: Self::Data, mut ids: F) -> Result<Self, Self::Error>
    where
        F: FnMut(M) -> Option<Entity>
    {
        let mut output: EntityVec<C> = EntityVec::with_capacity(data.len());

        for item in data.into_iter() {
            let converted_item = ConvertSaveload::convert_from(item, |marker| ids(marker))?;

            output.push(converted_item);            
        }

        Ok(output)
    }
}