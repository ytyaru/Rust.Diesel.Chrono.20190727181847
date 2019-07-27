extern crate diesel;
extern crate diesel_hello;

use self::diesel::prelude::*;
use self::diesel_hello::models::*;
use self::diesel_hello::*;

fn main() {
    use diesel_hello::schema::posts::dsl::*;

    let connection = establish_connection();
    let results = posts
        .filter(is_published.eq(true))
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("----------\n");
        println!("{}", post.body);
    }
}
