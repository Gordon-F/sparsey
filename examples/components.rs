use sparsey::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Position(i32, i32);

#[derive(Clone, Copy, Debug)]
pub struct Velocity(i32, i32);

#[derive(Clone, Copy, Debug)]
pub struct Immovable;

fn update_velocity(mut vel: CompMut<Velocity>, imv: Comp<Immovable>) {
	println!("[Update velocities]");

	for (e, (mut vel,)) in (&mut vel,).include(&imv).iter().entities() {
		println!("{:?} is immovable; set its velocity to (0, 0)", e);
		*vel = Velocity(0, 0);
	}

	println!();
}

fn update_position(mut pos: CompMut<Position>, vel: Comp<Velocity>) {
	println!("[Update positions]");

	for (e, (mut pos, vel)) in (&mut pos, &vel).iter().entities() {
		pos.0 += vel.0;
		pos.1 += vel.1;

		println!("{:?}, {:?}, {:?}", e, *pos, vel);
	}

	println!();
}

fn main() {
	let mut dispatcher = Dispatcher::builder()
		.add_system(update_velocity.system())
		.add_system(update_position.system())
		.build();

	let mut world = World::default();
	dispatcher.set_up(&mut world);

	world.create((Position(0, 0), Velocity(1, 1)));
	world.create((Position(0, 0), Velocity(2, 2)));
	world.create((Position(0, 0), Velocity(3, 3), Immovable));

	let mut resources = Resources::default();

	for _ in 0..3 {
		dispatcher.run_seq(&mut world, &mut resources).unwrap();
		world.advance_ticks().unwrap();
	}
}
