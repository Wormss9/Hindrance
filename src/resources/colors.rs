use crate::components::ColorId;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct Colors {
    pub materials: HashMap<(ColorId, Variant), Handle<ColorMaterial>>,
}

#[allow(clippy::approx_constant)]
impl FromWorld for Colors {
    fn from_world(world: &mut World) -> Self {
        use ColorId::*;
        use Variant::*;
        let mut assets = world.resource_mut::<Assets<ColorMaterial>>();
        let mut materials = HashMap::new();

        let own = Color::srgb(0.518, 0.318, 0.851);

        insert_trio(&mut materials, assets.as_mut(), Own, own, 0.7);
        insert_trio(
            &mut materials,
            assets.as_mut(),
            Foe1,
            Color::srgb(0.651, 0.851, 0.318),
            0.7,
        );
        insert_trio(
            &mut materials,
            assets.as_mut(),
            Foe2,
            Color::srgb(0.318, 0.651, 0.851),
            0.7,
        );
        insert_trio(
            &mut materials,
            assets.as_mut(),
            Misc,
            Color::srgb(0.8, 0.8, 0.2),
            0.7,
        );
        insert_trio(
            &mut materials,
            assets.as_mut(),
            Exit,
            Color::srgb(1.0, 0.2, 0.2),
            0.7,
        );
        insert_trio(&mut materials, assets.as_mut(), ReachableTile, own, 0.3);

        materials.insert(
            (Tile, Normal),
            assets.add(Color::srgb(1., 1., 1.).with_luminance(0.2)),
        );
        materials.insert(
            (Wall, Normal),
            assets.add(Color::srgb(0.851, 0.518, 0.318).with_luminance(0.7)),
        );
        materials.insert(
            (Curtain, Normal),
            assets.add(Color::srgba(0.0, 0.0, 0.0, 0.9)),
        );

        Self { materials }
    }
}

fn insert_trio(
    materials: &mut HashMap<(ColorId, Variant), Handle<ColorMaterial>>,
    assets: &mut Assets<ColorMaterial>,
    color_id: ColorId,
    color: Color,
    luminance: f32,
) {
    use Variant::*;
    materials.insert(
        (color_id, Normal),
        assets.add(color.with_luminance(luminance)),
    );
    materials.insert(
        (color_id, Light),
        assets.add(color.with_luminance(luminance * 1.5)),
    );
    materials.insert(
        (color_id, Dark),
        assets.add(color.with_luminance(luminance / 1.5)),
    );
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum Variant {
    Normal,
    Light,
    Dark,
}
