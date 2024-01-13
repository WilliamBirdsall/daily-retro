use self::models::*;
use diesel::prelude::*;
use daily_retro::*;

fn main() {
    use self::schema::entries::dsl::*;

    let connection = &mut establish_connection();

    // Print welcome message
    //     Print key commands, c for create, e for edit, and d for delete, l for list


    let mut went_well = String::new();
    let mut went_poorly = String::new();
    let mut learned = String::new();

    println!("What went well today?");
    io::stdin().read_line(&mut went_well).unwrap();

    println!("What went poorly today?");

    println!("What was learned today?");


    // Create entry with prompt answers
    // Save entry

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
