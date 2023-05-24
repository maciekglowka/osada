use std::collections::HashMap;

use super::enums::Goods;

pub trait Structure: Send + Sync {
    fn produce(&self) -> HashMap<Goods, u32> { HashMap::new( )}
    fn next(&self, _goods: HashMap<Goods, u32>) -> Option<Box<dyn Structure>> { None }
    // temporary, split to graphics
    fn sprite(&self) -> usize;
}

pub struct Empty;
impl Structure for Empty {
    fn sprite(&self) -> usize { 0 }
    fn next(&self, goods: HashMap<Goods, u32>) -> Option<Box<dyn Structure>> {
        if *goods.get(&Goods::Wood).unwrap_or(&0) < 2 {
            return None
        }
        return Some(Box::new(WoodCutter))
    }
}

pub struct Forest;
impl Structure for Forest {
    fn produce(&self) -> HashMap<Goods, u32> {
        HashMap::from_iter([(Goods::Wood, 2)])
    }
    fn sprite(&self) -> usize { 1 }
}

pub struct WoodCutter;
impl Structure for WoodCutter {
    fn sprite(&self) -> usize { 2 }
}