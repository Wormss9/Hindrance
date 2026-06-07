use bevy::prelude::*;

pub struct ColorsPlugin;

impl Plugin for ColorsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(Startup, add_colors);
        app.world_mut().resource_scope(
            |world: &mut World, mut materials: Mut<Assets<ColorMaterial>>| {
                let theme = Theme::new(&mut materials);
                world.insert_resource(theme);
            },
        );
    }
}

pub fn add_colors(mut commands: Commands, mut materials: Mut<Assets<ColorMaterial>>) {
    commands.insert_resource(Theme::new(&mut materials));
}

#[derive(Resource)]
pub struct Theme {
    pub own: ColorSet,
    pub foe1: ColorSet,
    pub foe2: ColorSet,
    pub tile: ColorSet,
    pub reachable_tile: ColorSet,
    pub own_tile: ColorSet,
    pub foe1_tile: ColorSet,
    pub foe2_tile: ColorSet,
    pub misc: ColorSet,
    pub wall: ColorSet,
    pub exit: ColorSet,
    pub curtain: ColorSet,
}

impl Theme {
    #[allow(clippy::approx_constant)]
    pub fn new(materials: &mut Mut<Assets<ColorMaterial>>) -> Self {
        let own = Color::srgb(0.518, 0.318, 0.851);
        let foe1 = Color::srgb(0.651, 0.851, 0.318);
        let foe2 = Color::srgb(0.318, 0.651, 0.851);
        let wall: Color = Color::srgb(0.851, 0.518, 0.318);

        Self {
            own: ColorSet::new(materials, own, 0.7),
            foe1: ColorSet::new(materials, foe1, 0.9),
            foe2: ColorSet::new(materials, foe2, 0.9),
            tile: ColorSet::new(materials, Color::srgb(1., 1., 1.), 0.2),
            reachable_tile: ColorSet::new(materials, own, 0.5),
            own_tile: ColorSet::new(materials, own, 0.3),
            foe1_tile: ColorSet::new(materials, foe1, 0.3),
            foe2_tile: ColorSet::new(materials, foe2, 0.3),
            misc: ColorSet::new(materials, Color::srgb(0.8, 0.8, 0.2), 0.7),
            wall: ColorSet::new(materials, wall, 0.7),
            exit: ColorSet::new(materials, Color::srgb(1.0, 0.2, 0.2), 0.7),
            curtain: ColorSet::dark(materials, Color::srgba(0.0, 0.0, 0.0, 0.9)),
        }
    }
}

pub struct ColorSet {
    pub normal: Handle<ColorMaterial>,
    pub light: Handle<ColorMaterial>,
    pub dark: Handle<ColorMaterial>,
}
impl ColorSet {
    pub fn new(materials: &mut Mut<Assets<ColorMaterial>>, base: Color, luminance: f32) -> Self {
        Self {
            normal: materials.add(base.with_luminance(luminance)),
            light: materials.add(base.with_luminance(luminance * 1.5)),
            dark: materials.add(base.with_luminance(luminance / 1.5)),
        }
    }
    pub fn dark(materials: &mut Mut<Assets<ColorMaterial>>, base: Color) -> Self {
        Self {
            normal: materials.add(base),
            light: materials.add(base),
            dark: materials.add(base),
        }
    }
}

pub trait PointerColorInteraction {
    fn with_color_set(&mut self, color: &ColorSet) -> &mut Self;
}

impl<'w> PointerColorInteraction for EntityCommands<'w> {
    fn with_color_set(&mut self, color: &ColorSet) -> &mut Self {
        self.observe(update_material_on::<Pointer<Over>>(color.light.clone()))
            .observe(update_material_on::<Pointer<Out>>(color.normal.clone()))
            .observe(update_material_on::<Pointer<Press>>(color.dark.clone()))
            .observe(update_material_on::<Pointer<Release>>(color.light.clone()))
            .insert(MeshMaterial2d(color.normal.clone()));

        self
    }
}

fn update_material_on<E: EntityEvent>(
    new_material: Handle<ColorMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial2d<ColorMaterial>>) {
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
        }
    }
}
