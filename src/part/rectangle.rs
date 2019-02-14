use crate::camera::Camera;
use graphics::{Graphics, Transformed};
use nalgebra::Vector2;
use nphysics2d::object::ColliderHandle;
use nphysics2d::world::World;

pub struct Rectangle {
    position: Vector2<f64>,
    handle: ColliderHandle,
    shape: graphics::Rectangle,
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(handle: ColliderHandle, world: &World<f64>, width: f64, height: f64) -> Self {
        let position = world.collider(handle).unwrap().position();
        Rectangle {
            position: position.translation.vector,
            handle,
            shape: graphics::Rectangle::new([1.0, 1.0, 1.0, 1.0]),
            width,
            height,
        }
    }

    pub fn update(&mut self, world: &World<f64>) {
        self.position = world
            .collider(self.handle)
            .unwrap()
            .position()
            .translation
            .vector;
    }

    pub fn draw<G>(&self, camera: &Camera, c: graphics::Context, g: &mut G)
    where
        G: Graphics,
    {
        let position = camera.to_global(&self.position);
        self.shape.draw(
            [0.0, 0.0, self.width, self.height],
            &graphics::DrawState::default(),
            c.trans(position.x, position.y)
                .zoom(camera.zoom())
                .transform,
            g,
        );
    }
}
