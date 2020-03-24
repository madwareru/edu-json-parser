use edu_json_parser::parse_json;

fn parse_card(card: &str) {
    println!("My card is:\n{}\n", card);
    let json = parse_json(card);
    match json {
        Ok(json_data) => {
            println!(
                "My name: {}",
                json_data
                    .get_string("name")
                    .unwrap_or("it appears that I don't have a name!".to_string())
            );
            println!(
                "My last_name: {}",
                json_data
                    .get_string("last_name")
                    .unwrap_or("it appears that I don't have a last_name!".to_string())
            );
            println!(
                "My age: {}",
                json_data
                    .get_number("age")
                    .map(|n| n.to_string())
                    .unwrap_or("it appears that I don't have an age!".to_string())
            );
            println!(
                "My weight: {}",
                json_data
                    .get_number("weight")
                    .map(|n| n.to_string())
                    .unwrap_or("it appears that I don't have an weight!".to_string())
            );
            if let Some(sizes) = json_data.get("sizes") {
                if sizes.is_array() {
                    if sizes.len() != 3 {
                        println!("Something wrong with sizes, they must be exactly 3 in length")
                    } else {
                        let breast_size = sizes[0]
                            .as_number()
                            .map(|n| n.to_string())
                            .unwrap_or("???".to_string());

                        let belly_size = sizes[1]
                            .as_number()
                            .map(|n| n.to_string())
                            .unwrap_or("???".to_string());

                        let booty_size = sizes[2]
                            .as_number()
                            .map(|n| n.to_string())
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

fn main() {
    parse_card(
        r#"{"name": "Santa", "last_name": "Clous", "age": 99, "weight": 150, "sizes":[120, 120, 120] }"#
    );
    parse_card(
        r#"{"name": "Robin", "last_name": "Hood", "sizes":[90, 60, 90] }"#
    );
    parse_card(
        r#"{"name": "Non existing", "last_name": "Substance"}"#
    );
    parse_card(
        r#"{"name": "S", "last_name": "Expr", "age": "(99)", "weight": "(150)"}"#
    );
    parse_card("[]");
    parse_card("{}");
    parse_card("null");
    parse_card("false");
    parse_card("42");
    parse_card("\"Who am I?\"");
    parse_card("Что я такое?"); // It fails but it does not breaks. It's fine
}