use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interactable(pub bool);

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pointable {
    pub over: bool,
    pub press: bool,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tile {
    pub id: usize,
    pub x: usize,
    pub y: usize,
}

impl Tile {
    pub fn new(id: usize, x: usize, y: usize) -> Self {
        Self { id, x, y }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SquareGapId {
    pub parent: usize,
    pub location: SquareGapLocation,
    pub wall: Entity,
}

impl SquareGapId {
    pub fn new(parent: usize, location: SquareGapLocation, wall: Entity) -> Self {
        Self {
            parent,
            location,
            wall,
        }
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OwnLocation {
    pub x: usize,
    pub y: usize,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FoeLocation {
    pub x: usize,
    pub y: usize,
}

impl OwnLocation {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl FoeLocation {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SquareGapLocation {
    RU,
    RD,
    DL,
    DR,
}

pub trait PointerInteraction {
    fn with_pointer_interaction(&mut self) -> &mut Self;
}

impl<'w> PointerInteraction for EntityCommands<'w> {
    fn with_pointer_interaction(&mut self) -> &mut Self {
        self.observe(
            move |event: On<Pointer<Over>>, mut query: Query<&mut Pointable, With<Tile>>| {
                let entity = event.event_target();
                if let Ok(mut pointable) = query.get_mut(entity) {
                    pointable.over = true;
                };
            },
        )
        .observe(
            move |event: On<Pointer<Out>>, mut query: Query<&mut Pointable, With<Tile>>| {
                let entity = event.event_target();
                if let Ok(mut pointable) = query.get_mut(entity) {
                    pointable.over = false;
                };
            },
        )
        .observe(
            move |event: On<Pointer<Press>>, mut query: Query<&mut Pointable, With<Tile>>| {
                let entity = event.event_target();
                if let Ok(mut pointable) = query.get_mut(entity) {
                    pointable.press = true;
                };
            },
        )
        .observe(
            move |event: On<Pointer<Release>>, mut query: Query<&mut Pointable, With<Tile>>| {
                let entity = event.event_target();
                if let Ok(mut pointable) = query.get_mut(entity) {
                    pointable.press = false;
                };
            },
        );

        self
    }
}

pub trait OwnMovement {
    fn with_move_own(&mut self) -> &mut Self;
}

impl<'w> OwnMovement for EntityCommands<'w> {
    fn with_move_own(&mut self) -> &mut Self {
        self.observe(            move |event: On<Pointer<Release>>,
                  mut own: Query<(&mut Transform, &mut OwnLocation), With<OwnLocation>>,
                  tile: Query<(&Interactable, &Transform, &Tile),Without<OwnLocation>>| {
                let Ok((interactable, target_transform, tile)) = tile.get(event.event_target())
                else {
                    return;
                };
                let Ok((mut own_transform, mut own_location)) = own.single_mut() else {
                    return;
                };
                if interactable.0 {
                    *own_transform = *target_transform;
                    own_location.x = tile.x;
                    own_location.y = tile.y;
                }
            },
        );

        self
    }
}

fn interactable(event: On<Pointer<Over>>, query: Query<&Interactable, With<Tile>>) -> bool {
    let entity = event.event_target();
    query.get(entity).ok().map(|i| i.0).unwrap_or(false)
}
