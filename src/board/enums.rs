use serde::Deserialize;

#[derive(Clone, Copy, Debug, Deserialize, Hash, Eq, PartialEq)]
pub enum TileKind {
    Forest,
    Mountains,
    Plains
}
impl TileKind {
    pub fn name(&self) -> &str {
        match self {
            Self::Forest => "Forest",
            Self::Mountains => "Mountains",
            Self::Plains => "Plains",
        }
    }
}


#[derive(Clone, Copy, Debug, Deserialize, Hash, Eq, PartialEq)]
pub enum SiteKind {
    Mines,
    Village,
    Town
}
impl SiteKind {
    pub fn name(&self) -> &str {
        match self {
            Self::Mines => "Mines",
            Self::Village => "Village",
            Self::Town => "Town",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Hash, Eq, PartialEq)]
pub enum Goods {
    Food,
    Ore,
    Tools
}
