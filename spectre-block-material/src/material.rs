use crate::map_colors::MapColor;

pub enum BlockLightInteraction {
    Transparent,
    Translucent,
    Opaque,
}

pub enum BlockPlayerInteraction {
    Blocks,
    Allows,
}

pub enum BlockMatter {
    Solid,
    Liquid,
    None,
}

pub enum BlockMobility {
    Pushable,
    NotPushable,
    Immovable,
}

pub struct MaterialProperties {
    pub color: MapColor,
    pub light_interaction: BlockLightInteraction,
    pub matter: BlockMatter,
    pub block_mobility: BlockMobility,
    pub player_mobility: BlockPlayerInteraction,
    pub is_replaceable: bool,
    pub is_flammable: bool,
    pub requires_tool: bool,
    pub is_adventure_mode_exempt: bool,
}

impl MaterialProperties {
    fn new(color: MapColor) -> MaterialProperties {
        MaterialProperties {
            color,
            light_interaction: BlockLightInteraction::Opaque,
            matter: BlockMatter::Solid,
            block_mobility: BlockMobility::Pushable,
            player_mobility: BlockPlayerInteraction::Blocks,
            is_replaceable: false,
            is_flammable: false,
            requires_tool: false,
            is_adventure_mode_exempt: false,
        }
    }

    fn new_transparent(color: MapColor) -> MaterialProperties {
        MaterialProperties {
            color,
            light_interaction: BlockLightInteraction::Transparent,
            matter: BlockMatter::None,
            block_mobility: BlockMobility::Pushable,
            player_mobility: BlockPlayerInteraction::Allows,
            is_replaceable: true,
            is_flammable: false,
            requires_tool: true,
            is_adventure_mode_exempt: false,
        }
    }

    fn new_liquid(color: MapColor) -> MaterialProperties {
        MaterialProperties {
            color,
            light_interaction: BlockLightInteraction::Transparent,
            matter: BlockMatter::Liquid,
            block_mobility: BlockMobility::Pushable,
            player_mobility: BlockPlayerInteraction::Blocks,
            is_replaceable: false,
            is_flammable: false,
            requires_tool: true,
            is_adventure_mode_exempt: false,
        }
    }

    // TODO: figure out what is logic
    fn new_logic(color: MapColor) -> MaterialProperties {
        MaterialProperties {
            color,
            light_interaction: BlockLightInteraction::Transparent,
            matter: BlockMatter::None,
            block_mobility: BlockMobility::Pushable,
            player_mobility: BlockPlayerInteraction::Allows,
            is_replaceable: false,
            is_flammable: false,
            requires_tool: true,
            is_adventure_mode_exempt: true,
        }
    }

    fn new_portal(color: MapColor) -> MaterialProperties {
        MaterialProperties {
            color,
            light_interaction: BlockLightInteraction::Transparent,
            matter: BlockMatter::None,
            block_mobility: BlockMobility::Immovable,
            player_mobility: BlockPlayerInteraction::Allows,
            is_replaceable: true,
            is_flammable: false,
            requires_tool: false,
            is_adventure_mode_exempt: false,
        }
    }

    fn set_flammable(mut self) -> MaterialProperties {
        self.is_flammable = true;
        self
    }

    fn set_requires_tool(mut self) -> MaterialProperties {
        self.requires_tool = true;
        self
    }

    fn set_mobility(mut self, mobility: BlockMobility) -> MaterialProperties {
        self.block_mobility = mobility;
        self
    }

    fn set_visibility(mut self, visibility: BlockLightInteraction) -> MaterialProperties {
        self.light_interaction = visibility;
        self
    }

    fn set_adventure_mode_exempt(mut self) -> MaterialProperties {
        self.is_adventure_mode_exempt = true;
        self
    }

    fn set_replaceable(mut self) -> MaterialProperties {
        self.is_replaceable = true;
        self
    }
}

pub enum Material {
    Air,
    Grass,
    Ground,
    Wood,
    Rock,
    Iron,
    Anvil,
    Water,
    Lava,
    Leaves,
    Plants,
    Vine,
    Sponge,
    Cloth,
    Fire,
    Sand,
    Circuits,
    Carpet,
    Glass,
    RedstoneLight,
    Tnt,
    Coral,
    Ice,
    PackedIce,
    Snow,
    CraftedSnow,
    Cactus,
    Clay,
    Gourd,
    DragonEgg,
    Portal,
    Cake,
    Web,
}

impl Material {
    pub fn get_material_properties(&self) -> MaterialProperties {
        match &self {
            Material::Air => MaterialProperties::new_transparent(MapColor::Air),
            Material::Grass => MaterialProperties::new(MapColor::Grass),
            Material::Ground => MaterialProperties::new(MapColor::Dirt),
            Material::Wood => MaterialProperties::new(MapColor::Wood).set_flammable(),
            Material::Rock => MaterialProperties::new(MapColor::Stone).set_requires_tool(),
            Material::Iron => MaterialProperties::new(MapColor::Iron).set_requires_tool(),
            Material::Anvil => MaterialProperties::new(MapColor::Iron)
                .set_requires_tool()
                .set_mobility(BlockMobility::NotPushable),
            Material::Water => MaterialProperties::new_liquid(MapColor::Water)
                .set_mobility(BlockMobility::NotPushable),
            Material::Lava => MaterialProperties::new_liquid(MapColor::TNT)
                .set_mobility(BlockMobility::NotPushable),
            Material::Leaves => MaterialProperties::new(MapColor::Foliage)
                .set_flammable()
                .set_visibility(BlockLightInteraction::Translucent)
                .set_mobility(BlockMobility::NotPushable),
            Material::Plants => MaterialProperties::new_logic(MapColor::Foliage)
                .set_mobility(BlockMobility::NotPushable),
            Material::Vine => MaterialProperties::new_logic(MapColor::Foliage)
                .set_mobility(BlockMobility::NotPushable)
                .set_flammable()
                .set_replaceable(),
            Material::Sponge => MaterialProperties::new(MapColor::Yellow),
            Material::Cloth => MaterialProperties::new(MapColor::Cloth).set_flammable(),
            Material::Fire => MaterialProperties::new_transparent(MapColor::Air)
                .set_mobility(BlockMobility::NotPushable),
            Material::Sand => MaterialProperties::new(MapColor::Sand),
            Material::Circuits => MaterialProperties::new_logic(MapColor::Air),
            Material::Carpet => MaterialProperties::new_logic(MapColor::Cloth).set_flammable(),
            Material::Glass => {
                MaterialProperties::new_transparent(MapColor::Air).set_adventure_mode_exempt()
            }
            Material::RedstoneLight => {
                MaterialProperties::new(MapColor::Air).set_adventure_mode_exempt()
            }
            Material::Tnt => MaterialProperties::new(MapColor::TNT)
                .set_flammable()
                .set_visibility(BlockLightInteraction::Translucent),
            Material::Coral => {
                MaterialProperties::new(MapColor::Foliage).set_mobility(BlockMobility::NotPushable)
            }
            Material::Ice => MaterialProperties::new(MapColor::Ice)
                .set_visibility(BlockLightInteraction::Translucent)
                .set_adventure_mode_exempt(),
            Material::PackedIce => {
                MaterialProperties::new(MapColor::Ice).set_adventure_mode_exempt()
            }
            Material::Snow => MaterialProperties::new_logic(MapColor::Snow)
                .set_replaceable()
                .set_visibility(BlockLightInteraction::Translucent)
                .set_requires_tool()
                .set_mobility(BlockMobility::NotPushable),
            Material::CraftedSnow => MaterialProperties::new(MapColor::Snow).set_requires_tool(),
            Material::Cactus => MaterialProperties::new(MapColor::Foliage)
                .set_visibility(BlockLightInteraction::Translucent)
                .set_mobility(BlockMobility::NotPushable),
            Material::Clay => MaterialProperties::new(MapColor::Clay),
            Material::Gourd => {
                MaterialProperties::new(MapColor::Foliage).set_mobility(BlockMobility::NotPushable)
            }
            Material::DragonEgg => {
                MaterialProperties::new(MapColor::Foliage).set_mobility(BlockMobility::NotPushable)
            }
            Material::Portal => {
                MaterialProperties::new_portal(MapColor::Air).set_mobility(BlockMobility::Immovable)
            }
            Material::Cake => {
                MaterialProperties::new(MapColor::Air).set_mobility(BlockMobility::NotPushable)
            }
            Material::Web => MaterialProperties::new(MapColor::Cloth),
        }
    }
}
