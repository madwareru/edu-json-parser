use edu_json_parser::parse_json;

fn parse_card(card: &str) {
    println!("My card is:\n{}\n", card);
    let json = parse_json(card);
    match json {
        Ok(json_data) => {
            println!(
                "My name: {}",
                json_data
                    .get("name")
                    .map(|v| v.as_string().unwrap_or("it appears that I have a nonstringy name!".to_string()))
                    .unwrap_or("it appears that I don't have a name!".to_string())
            );
            println!(
                "My last_name: {}",
                json_data
                    .get("last_name")
                    .map(|v| v.as_string().unwrap_or("it appears that I have a nonstringy last_name!".to_string()))
                    .unwrap_or("it appears that I don't have a last_name!".to_string())
            );
            println!(
                "My age: {}",
                json_data
                    .get("age")
                    .map(|v| v.as_number().map(|n| n.to_string()).unwrap_or("it appears that I have a nonnumber age!".to_string()))
                    .unwrap_or("it appears that I don't have an age!".to_string())
            );
            println!(
                "My weight: {}",
                json_data
                    .get("weight")
                    .map(|v| v.as_number().map(|n| n.to_string()).unwrap_or("it appears that I have a nonnumber weight!".to_string()))
                    .unwrap_or("it appears that I don't have an weight!".to_string())
            );
            if let Some(sizes) = json_data.get("sizes") {
                match (*sizes).as_array() {
                    Some(nodes) => {
                        if nodes.len() != 3 {
                            println!("Something wrong with sizes, they must be exactly 3 in length")
                        } else {
                            let breast_size = nodes[0]
                                .as_number()
                                .map(|n| n.to_string())
                                .unwrap_or("???".to_string());

                            let belly_size = nodes[1]
                                .as_number()
                                .map(|n| n.to_string())
                                .unwrap_or("???".to_string());

                            let booty_size = nodes[2]
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
                    },
                    _ => println!("It looks that I don't have a sizes somehow. Maybe they aren't array?")
                }
            } else {
                println!("It looks that I don't have a sizes somehow. Maybe I don't even exist?")
            }
        },
        Err(error_string) => println!("Error! {}", error_string)
    }
    println!()
}

fn main() {
    parse_card(
        r##"{"name": "Santa", "last_name": "Clous", "age": 99, "weight": 150, "sizes":[120, 120, 120] }"##
    );
    parse_card(
        r##"{"name": "Robin", "last_name": "Hood", "sizes":[90, 60, 90] }"##
    );
    parse_card(
        r##"{"name": "Non existing", "last_name": "Substance"}"##
    );
    parse_card(
        r##"{"name": "S", "last_name": "Expr", "age": "(99)", "weight": "(150)"}"##
    );
    parse_card("[]");
    parse_card("{}");
    parse_card("null");
    parse_card("false");
    parse_card("42");
    parse_card("\"Who am I?\"");
    parse_card("Что я такое?"); // It fails
}