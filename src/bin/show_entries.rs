use self::models::*;
use diesel::prelude::*;
use daily_retro::*;

fn main() {
    use self::schema::entries::dsl::*;

    let connection = &mut establish_connection();
    let results = entries
        .limit(5)
        .select(Entry::as_select())
        .load(connection)
        .expect("Error loading entries");

    println!("Displaying {} entries", results.len());
    for entry in results {
        println!("{}", entry.id);
        println!("----------\n");
        println!("{}", entry.body);
    }
}
