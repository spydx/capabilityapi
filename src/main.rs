// Source: https://web.archive.org/web/20180120000131/http://www.zsck.co/writing/capability-based-apis.html

use capabilityapi::capabilities::{
    display_db_content, handle_delete_user, handle_find_user, handle_save_user, handle_update_user,
    SQLite,
};
use capabilityapi::model::User;

fn main() {
    println!("Hello, world!\n");

    let connection = sqlite::open(":memory:").unwrap();

    connection
        .execute(
            "CREATE TABLE users (name TEXT,password TEXT);
            INSERT INTO users VALUES ('kenneth', 'password');
            INSERT INTO users VALUES ('boisy', 'woof');",
        )
        .unwrap();

    let db = SQLite { db: connection };

    let user = User {
        name: "Ollie".to_string(),
        password: "pffpff".to_string(),
    };

    let u = handle_save_user(&db, user).unwrap();

    println!("Saved:");
    println!("{}\n", u);

    let mut boisy = handle_find_user(&db, "boisy".to_string()).unwrap();
    println!("Found:");
    println!("{}\n", &boisy);

    boisy.password = "WoofWoof".to_string();

    let updated = handle_update_user(&db, boisy).unwrap();
    println!("Updated:");
    println!("{}\n", updated);

    display_db_content(&db);
    handle_delete_user(&db, "kenneth".to_string()).unwrap();
    display_db_content(&db);
}
