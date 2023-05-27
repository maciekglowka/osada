use std::collections::HashMap;

use super::enums::Goods;

pub trait Structure: Send + Sync {
    fn produce(&self) -> HashMap<Goods, u32> { HashMap::new( )}
    fn get_next(&self, _goods: &HashMap<Goods, u32>) -> Vec<Box<dyn Structure>> { Vec::new() }
    fn name(&self) -> &str;
    // temporary, split to graphics
    fn sprite(&self) -> usize;
}

pub struct Empty;
impl Structure for Empty {
    fn sprite(&self) -> usize { 0 }
    fn get_next(&self, goods: &HashMap<Goods, u32>) -> Vec<Box<dyn Structure>> {
        let mut output = Vec::new();
        if *goods.get(&Goods::Wood).unwrap_or(&0) >= 2 {
            output.push(Box::new(WoodCutter {}) as Box<dyn Structure>);
        }
        output
    }
    fn name(&self) -> &str { "Empty" }
}

pub struct Forest;
impl Structure for Forest {
    fn produce(&self) -> HashMap<Goods, u32> {
        HashMap::from_iter([(Goods::Wood, 1)])
    }
    fn sprite(&self) -> usize { 1 }
    fn name(&self) -> &str { "Forest" }
}

pub struct WoodCutter;
impl Structure for WoodCutter {
    fn sprite(&self) -> usize { 2 }
    fn name(&self) -> &str { "WoodCutter" }
}