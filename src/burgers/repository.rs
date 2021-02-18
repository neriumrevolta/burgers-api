#![allow(proc_macro_derive_resolution_fallback)]

use crate::burgers::Burger;
use crate::burgers::InsertableBurger;
use crate::schema::burgers;
use diesel;
use diesel::prelude::*;
use rand::prelude::*;

pub fn all(connection: &PgConnection) -> QueryResult<Vec<Burger>> {
    burgers::table.load::<Burger>(&*connection)
}

pub fn get(id: i32, connection: &PgConnection) -> QueryResult<Burger> {
    burgers::table.find(id).get_result::<Burger>(connection)
}

pub fn find_by_name(name: String, connection: &PgConnection) -> QueryResult<Burger> {
    let all = burgers::table.load::<Burger>(&*connection);

    match all {
        Ok(all) => {
            let first_matching = all
                .into_iter()
                .filter(|b| b.name == name)
                .collect::<Vec<Burger>>()
                .first()
                .unwrap()
                .clone();
            return Ok(first_matching);
        }
        Err(e) => Err(e),
    }
}

pub fn rand(connection: &PgConnection) -> QueryResult<Burger> {
    let mut rng = rand::thread_rng();
    let all = burgers::table.load::<Burger>(&*connection);

    match all {
        Ok(all) => {
            let len = all.len();
            let x: usize = rng.gen_range(0..len);
            return Ok(all[x].clone());
        }
        Err(e) => Err(e),
    }
}

pub fn insert(burger: InsertableBurger, connection: &PgConnection) -> QueryResult<Burger> {
    diesel::insert_into(burgers::table)
        .values(burger)
        .get_result(connection)
}

pub fn update(id: i32, burger: InsertableBurger, connection: &PgConnection) -> QueryResult<Burger> {
    diesel::update(burgers::table.find(id))
        .set(&burger)
        .get_result(connection)
}

pub fn delete(id: i32, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(burgers::table.find(id)).execute(connection)
}
