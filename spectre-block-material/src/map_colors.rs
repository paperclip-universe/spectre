use spectre_util_color::Color;

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
    pub color_value: Color,
}

impl MapColor {
    pub fn get_color(&self) -> MapColorValue {
        match &self {
            MapColor::Air => MapColorValue {
                color_index: 0,
                color_value: 0.into(),
            },
            MapColor::Grass => MapColorValue {
                color_index: 1,
                color_value: 8368696.into(),
            },
            MapColor::Sand => MapColorValue {
                color_index: 2,
                color_value: 16247203.into(),
            },
            MapColor::Cloth => MapColorValue {
                color_index: 3,
                color_value: 13092807.into(),
            },
            MapColor::TNT => MapColorValue {
                color_index: 4,
                color_value: 16711680.into(),
            },
            MapColor::Ice => MapColorValue {
                color_index: 5,
                color_value: 10526975.into(),
            },
            MapColor::Iron => MapColorValue {
                color_index: 6,
                color_value: 10987431.into(),
            },
            MapColor::Foliage => MapColorValue {
                color_index: 7,
                color_value: 31744.into(),
            },
            MapColor::Snow => MapColorValue {
                color_index: 8,
                color_value: 16777215.into(),
            },
            MapColor::Clay => MapColorValue {
                color_index: 9,
                color_value: 10791096.into(),
            },
            MapColor::Dirt => MapColorValue {
                color_index: 10,
                color_value: 9923917.into(),
            },
            MapColor::Stone => MapColorValue {
                color_index: 11,
                color_value: 7368816.into(),
            },
            MapColor::Water => MapColorValue {
                color_index: 12,
                color_value: 4210943.into(),
            },
            MapColor::Wood => MapColorValue {
                color_index: 13,
                color_value: 9402184.into(),
            },
            MapColor::Quartz => MapColorValue {
                color_index: 14,
                color_value: 16776437.into(),
            },
            MapColor::Adobe => MapColorValue {
                color_index: 15,
                color_value: 14188339.into(),
            },
            MapColor::Magenta => MapColorValue {
                color_index: 16,
                color_value: 11685080.into(),
            },
            MapColor::LightBlue => MapColorValue {
                color_index: 17,
                color_value: 6724056.into(),
            },
            MapColor::Yellow => MapColorValue {
                color_index: 18,
                color_value: 15066419.into(),
            },
            MapColor::Lime => MapColorValue {
                color_index: 19,
                color_value: 8375321.into(),
            },
            MapColor::Pink => MapColorValue {
                color_index: 20,
                color_value: 15892389.into(),
            },
            MapColor::Gray => MapColorValue {
                color_index: 21,
                color_value: 5000268.into(),
            },
            MapColor::Silver => MapColorValue {
                color_index: 22,
                color_value: 10066329.into(),
            },
            MapColor::Cyan => MapColorValue {
                color_index: 23,
                color_value: 5013401.into(),
            },
            MapColor::Purple => MapColorValue {
                color_index: 24,
                color_value: 8339378.into(),
            },
            MapColor::Blue => MapColorValue {
                color_index: 25,
                color_value: 3361970.into(),
            },
            MapColor::Brown => MapColorValue {
                color_index: 26,
                color_value: 6704179.into(),
            },
            MapColor::Green => MapColorValue {
                color_index: 27,
                color_value: 6717235.into(),
            },
            MapColor::Red => MapColorValue {
                color_index: 28,
                color_value: 10040115.into(),
            },
            MapColor::Black => MapColorValue {
                color_index: 29,
                color_value: 1644825.into(),
            },
            MapColor::Gold => MapColorValue {
                color_index: 30,
                color_value: 16445005.into(),
            },
            MapColor::Diamond => MapColorValue {
                color_index: 31,
                color_value: 6085589.into(),
            },
            MapColor::Lapis => MapColorValue {
                color_index: 32,
                color_value: 4882687.into(),
            },
            MapColor::Emerald => MapColorValue {
                color_index: 33,
                color_value: 55610.into(),
            },
            MapColor::Obsidian => MapColorValue {
                color_index: 34,
                color_value: 8476209.into(),
            },
            MapColor::Netherrack => MapColorValue {
                color_index: 35,
                color_value: 7340544.into(),
            },
        }
    }
}
