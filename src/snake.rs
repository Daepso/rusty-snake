use {ARENA_HEIGHT,ARENA_WIDTH,SNAKE_COLOUR, FOOD_COLOUR, FOOD_RADIUS, SNAKE_RADIUS};

use amethyst::ecs::{Entity, Component, VecStorage, World};
use amethyst::core::transform::{LocalTransform,Transform};
use cgmath::Vector3;
use rendering::*;

use amethyst::renderer::{Material, MeshHandle};

pub struct Snake {
    pub tail : Entity,
    pub head : Entity,
    pub growing : bool
}

pub struct SnakePart(pub Option<Entity>);

impl Component for SnakePart {
    type Storage = VecStorage<SnakePart>;
}

pub struct SnakeResource {
    pub mesh : MeshHandle,
    pub material : Material
}

pub fn initialise_snake(world: &mut World) {
    let mut transform = LocalTransform::default();
    transform.translation = Vector3::new(ARENA_WIDTH/2.,ARENA_HEIGHT/2.,0.0);

    let mesh = create_mesh(
        world,
        generate_rectangle_vertices(-SNAKE_RADIUS, -SNAKE_RADIUS, SNAKE_RADIUS, SNAKE_RADIUS),
    );

    let material = create_colour_material(world, SNAKE_COLOUR);
    world.add_resource(SnakeResource {mesh:mesh.clone(),material:material.clone()});

    let head = world.create_entity()
    .with(SnakePart(None))
    .with(mesh.clone())
    .with(material.clone())
    .with(transform.clone())
    .with(Transform::default())
    .build();

    transform.translation.y -= 1.0;

    let mid = world.create_entity()
    .with(SnakePart(Some(head)))
    .with(mesh.clone())
    .with(material.clone())
    .with(transform.clone())
    .with(Transform::default())
    .build();

    transform.translation.y -= 1.0;

    let tail = world.create_entity()
    .with(SnakePart(Some(mid)))
    .with(mesh)
    .with(material)
    .with(transform)
    .with(Transform::default())
    .build();

    world.add_resource(Snake {head:head,tail:tail,growing:false});
}

pub struct Food(pub u8);

impl Component for Food {
    type Storage = VecStorage<Food>;
}

pub struct FoodResource {
    pub mesh : MeshHandle,
    pub material : Material
}

pub fn initialise_food(world: &mut World) {
    let mesh = create_mesh(world, generate_circle_vertices(FOOD_RADIUS, 16));
    let material = create_colour_material(world, FOOD_COLOUR);

    world.add_resource(FoodResource {mesh:mesh,material:material});
}
