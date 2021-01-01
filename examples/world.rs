#![allow(unused_variables)]

use ecstasy::prelude::*;

#[derive(Debug)]
struct A;

#[derive(Debug)]
struct B;

#[derive(Debug)]
struct C;

#[derive(Debug)]
struct D;

#[derive(Debug)]
struct E;

type WorldLayout = (((A, B), (A, B, C)), ((D, E),));

fn main() {
    let mut world = World::new::<WorldLayout>();
    world.register::<A>();
    world.register::<B>();
    world.register::<C>();
    world.register::<D>();
    world.register::<E>();

    let e0 = world.create((A, B));
    let e1 = world.create((A, B, C));
    let e2 = world.create((A, B, C, D, E));

    let (mut a, mut b, mut c, d, e) = unsafe {
        <(CompMut<A>, CompMut<B>, CompMut<C>, CompMut<D>, CompMut<E>)>::get_from_world(&world)
    };

    {
        let i1 = (&mut a, &mut b, &mut c).join();
    }

    // TODO: Fix borrow-checker erors
    // let i2 = (&mut a, &mut b, &mut c).join();
}
