use super::{Map, Player, Position, Viewshed};
use rltk::{Point, field_of_view};
use specs::prelude::*;

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        //expect a map write , if not fail
        WriteExpect<'a, Map>,
        Entities<'a>,
        //expect a map read , if not fail
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;
        // clear visible tiles
        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            viewshed.visible_tiles.clear();
            //provides field of view from position starting point
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map); // deref and ref to unwrap Map from ecs
            //provide lambda to retain method to delete unmatched criteria entries
            viewshed
                .visible_tiles
                .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

            // if this is player, reveal their pov
            let p: Option<&Player> = player.get(ent);
            if let Some(_p) = p {
                for vis in viewshed.visible_tiles.iter() {
                    let idx = map.xy_idx(vis.x, vis.y);
                    map.revealed_tiles[idx] = true
                }
            }
        }
    }
}
