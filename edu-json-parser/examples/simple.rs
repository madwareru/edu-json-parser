use edu_json_parser::{parse_json};

fn parse_card(card: &str) {
    println!("My card is:\n{}\n", card);
    let json = parse_json(card);
    match json {
        Ok(json_data) => {
            println!(
                "My name: {}",
                match json_data.get_string("name") {
                    Ok(s) => s.to_string(),
                    Err(cause) => format!(
                        "it appears that I don't have a name! To be precisely, {}",
                        cause.to_string()
                    ),
                }
            );
            println!(
                "My last_name: {}",
                match json_data.get_string("last_name") {
                    Ok(s) => s.to_string(),
                    Err(cause) => format!(
                        "it appears that I don't have a last_name! To be precisely, {}",
                        cause.to_string()
                    ),
                }
            );
            println!(
                "My age: {}",
                match json_data.get_as_string("age") {
                    Ok(s) => s.to_string(),
                    Err(cause) => format!(
                        "it appears that I don't have an age! To be precisely, {}",
                        cause.to_string()
                    ),
                }
            );
            println!(
                "My weight: {}",
                match json_data.get_as_string("weight") {
                    Ok(s) => s.to_string(),
                    Err(cause) => format!(
                        "it appears that I don't have an weight! To be precisely, {}",
                        cause.to_string()
                    ),
                }
            );
            if let Ok(sizes) = json_data.get("sizes") {
                if sizes.is_array() {
                    if sizes.len() != 3 {
                        println!("Something wrong with sizes, they must be exactly 3 in length")
                    } else {
                        let breast_size = sizes[0].to_string()
                            .unwrap_or("???".to_string());

                        let belly_size = sizes[1].to_string()
                            .unwrap_or("???".to_string());

                        let booty_size = sizes[2].to_string()
                            .unwrap_or("???".to_string());

                        println!(
                            "My sizes are: breast={}, belly={}, booty={}",
                            breast_size,
                            belly_size,
                            booty_size
                        );
                    }
                } else {
                    println!("It looks that somehow sizes aren't array!")
                }
            } else {
                println!("It looks that I don't have sizes somehow. Maybe I don't even exist?")
            }
        },
        Err(error_string) => println!("Error! {}", error_string)
    }
    println!()
}

const CARDS: [&'static str; 11] = [
    r#"{"name": "Santa", "last_name": "Clous", "age": 99, "weight": 150, "sizes":[120, 120, 120] }"#,
    r#"{"name": "Robin", "last_name": "Hood", "sizes":[90, 60, 90] }"#,
    r#"{"name": "Non existing", "last_name": "Substance"}"#,
    r#"{"name": "S", "last_name": "Expr", "age": "(99)", "weight": "(150)"}"#,
    // ^^^ this is tricky. get_as_string() would do too well and get string data where it shouldn't,
    // so this is where we should say "hey, get_as_string should return error when it sees a string"
    // this is why this method has such strange semantic, so yeah, I'm not so stupid here
    "[]",
    "{}",
    "null",
    "false",
    "42",
    r#""Who am I?""#,
    "Что я такое?" // It fails. But it should. So it's good
];

fn main() {
    for card in CARDS.iter() {
        parse_card(card);
    }
}