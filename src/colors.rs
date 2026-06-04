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
    pub foe: ColorSet,
    pub tile: ColorSet,
    pub reachable_tile: ColorSet,
    pub own_tile: ColorSet,
    pub fore_tile: ColorSet,
    pub misc: ColorSet,
    pub wall: ColorSet,
    pub exit: ColorSet,
}

impl Theme {
    pub fn new(materials: &mut Mut<Assets<ColorMaterial>>) -> Self {
        Self {
            own: ColorSet::new(materials, Color::srgb(0.4, 0.2, 1.0), 0.5),
            foe: ColorSet::new(materials, Color::srgb(0.5, 1.0, 0.5), 0.5),
            tile: ColorSet::new(materials, Color::srgb(1., 1., 1.), 0.3),
            reachable_tile: ColorSet::new(materials, Color::srgb(0.4, 0.3, 0.5), 0.3),
            own_tile: ColorSet::new(materials, Color::srgb(0.7, 0.2, 1.0), 0.3),
            fore_tile: ColorSet::new(materials, Color::srgb(0.5, 1.0, 0.5), 0.3),
            misc: ColorSet::new(materials, Color::srgb(0.8, 0.8, 0.2), 0.5),
            wall: ColorSet::new(materials, Color::srgb(0.2, 0.2, 1.0), 0.5),
            exit: ColorSet::new(materials, Color::srgb(1.0, 0.2, 0.2), 0.5),
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
            .insert( MeshMaterial2d(color.normal.clone()),);

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
