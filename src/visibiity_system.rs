use super::{Map, Position, Viewshed};
use rltk::{Point, field_of_view};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        //expect a map read , if not fail
        ReadExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (map, mut viewshed, pos) = data;
        // clear visible tiles
        for (viewshed, pos) in (&mut viewshed, &pos).join() {
            viewshed.visible_tiles.clear();
            //provides field of view from position starting point
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map); // deref and ref to unwrap Map from ecs
            //provide lambda to retain method to delete unmatched criteria entries
            viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
        }
    }
}
