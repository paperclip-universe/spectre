#[allow(clippy::upper_case_acronyms)]
pub enum MapColor {
    Air,
    Grass,
    Sand,
    Cloth,
    TNT,
    Ice,
    Iron,
    Foliage,
    Snow,
    Clay,
    Dirt,
    Stone,
    Water,
    Wood,
    Quartz,
    Adobe,
    Magenta,
    LightBlue,
    Yellow,
    Lime,
    Pink,
    Gray,
    Silver,
    Cyan,
    Purple,
    Blue,
    Brown,
    Green,
    Red,
    Black,
    Gold,
    Diamond,
    Lapis,
    Emerald,
    Obsidian,
    Netherrack,
}

pub struct MapColorValue {
    pub color_index: i8,
    pub color_value: i32,
}

impl MapColor {
    pub fn get_color(&self) -> MapColorValue {
        match &self {
            MapColor::Air => MapColorValue {
                color_index: 0,
                color_value: 0,
            },
            MapColor::Grass => MapColorValue {
                color_index: 1,
                color_value: 8368696,
            },
            MapColor::Sand => MapColorValue {
                color_index: 2,
                color_value: 16247203,
            },
            MapColor::Cloth => MapColorValue {
                color_index: 3,
                color_value: 13092807,
            },
            MapColor::TNT => MapColorValue {
                color_index: 4,
                color_value: 16711680,
            },
            MapColor::Ice => MapColorValue {
                color_index: 5,
                color_value: 10526975,
            },
            MapColor::Iron => MapColorValue {
                color_index: 6,
                color_value: 10987431,
            },
            MapColor::Foliage => MapColorValue {
                color_index: 7,
                color_value: 31744,
            },
            MapColor::Snow => MapColorValue {
                color_index: 8,
                color_value: 16777215,
            },
            MapColor::Clay => MapColorValue {
                color_index: 9,
                color_value: 10791096,
            },
            MapColor::Dirt => MapColorValue {
                color_index: 10,
                color_value: 9923917,
            },
            MapColor::Stone => MapColorValue {
                color_index: 11,
                color_value: 7368816,
            },
            MapColor::Water => MapColorValue {
                color_index: 12,
                color_value: 4210943,
            },
            MapColor::Wood => MapColorValue {
                color_index: 13,
                color_value: 9402184,
            },
            MapColor::Quartz => MapColorValue {
                color_index: 14,
                color_value: 16776437,
            },
            MapColor::Adobe => MapColorValue {
                color_index: 15,
                color_value: 14188339,
            },
            MapColor::Magenta => MapColorValue {
                color_index: 16,
                color_value: 11685080,
            },
            MapColor::LightBlue => MapColorValue {
                color_index: 17,
                color_value: 6724056,
            },
            MapColor::Yellow => MapColorValue {
                color_index: 18,
                color_value: 15066419,
            },
            MapColor::Lime => MapColorValue {
                color_index: 19,
                color_value: 8375321,
            },
            MapColor::Pink => MapColorValue {
                color_index: 20,
                color_value: 15892389,
            },
            MapColor::Gray => MapColorValue {
                color_index: 21,
                color_value: 5000268,
            },
            MapColor::Silver => MapColorValue {
                color_index: 22,
                color_value: 10066329,
            },
            MapColor::Cyan => MapColorValue {
                color_index: 23,
                color_value: 5013401,
            },
            MapColor::Purple => MapColorValue {
                color_index: 24,
                color_value: 8339378,
            },
            MapColor::Blue => MapColorValue {
                color_index: 25,
                color_value: 3361970,
            },
            MapColor::Brown => MapColorValue {
                color_index: 26,
                color_value: 6704179,
            },
            MapColor::Green => MapColorValue {
                color_index: 27,
                color_value: 6717235,
            },
            MapColor::Red => MapColorValue {
                color_index: 28,
                color_value: 10040115,
            },
            MapColor::Black => MapColorValue {
                color_index: 29,
                color_value: 1644825,
            },
            MapColor::Gold => MapColorValue {
                color_index: 30,
                color_value: 16445005,
            },
            MapColor::Diamond => MapColorValue {
                color_index: 31,
                color_value: 6085589,
            },
            MapColor::Lapis => MapColorValue {
                color_index: 32,
                color_value: 4882687,
            },
            MapColor::Emerald => MapColorValue {
                color_index: 33,
                color_value: 55610,
            },
            MapColor::Obsidian => MapColorValue {
                color_index: 34,
                color_value: 8476209,
            },
            MapColor::Netherrack => MapColorValue {
                color_index: 35,
                color_value: 7340544,
            },
        }
    }
}
