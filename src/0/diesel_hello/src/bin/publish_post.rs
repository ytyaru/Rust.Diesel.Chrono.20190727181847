extern crate diesel;
extern crate diesel_hello;

use self::diesel::prelude::*;
use self::diesel_hello::*;
use std::env::args;

fn main() {
    use diesel_hello::schema::posts::dsl::{posts, is_published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let _ = diesel::update(posts.find(id))
        .set(is_published.eq(true))
        .execute(&connection)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    let post: models::Post = posts
        .find(id)
        .first(&connection)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    println!("Published post {}", post.title);
}
