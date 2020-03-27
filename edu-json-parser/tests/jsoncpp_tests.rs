#[cfg(test)]
mod tests {
    use edu_json_parser::{parse_json};

    const FAILS: [&'static str; 33] = [
        include!("fail01.json"), // <- arguable case so commented in asserts
        include!("fail02.json"),
        include!("fail03.json"),
        include!("fail04.json"),
        include!("fail05.json"),
        include!("fail06.json"),
        include!("fail07.json"),
        include!("fail08.json"),
        include!("fail09.json"),
        include!("fail10.json"),
        include!("fail11.json"),
        include!("fail12.json"),
        include!("fail13.json"),
        include!("fail14.json"),
        include!("fail15.json"),
        include!("fail16.json"),
        include!("fail17.json"),
        include!("fail18.json"), // <- arguable case so commented in asserts
        include!("fail19.json"),
        include!("fail20.json"),
        include!("fail21.json"),
        include!("fail22.json"),
        include!("fail23.json"),
        include!("fail24.json"),
        include!("fail25.json"),
        include!("fail26.json"),
        include!("fail27.json"),
        include!("fail28.json"),
        include!("fail29.json"),
        include!("fail30.json"),
        include!("fail31.json"),
        include!("fail32.json"),
        include!("fail33.json")
    ];

    const PASSES: [&'static str; 3] = [
        include!("pass01.json"),
        include!("pass02.json"),
        include!("pass03.json")
    ];

    #[test]
    fn json_cpp_fails() {
        //assert!(parse_json(FAILS[0]).is_err());
        //^^^ arguable case
        assert!(parse_json(FAILS[1]).is_err());
        assert!(parse_json(FAILS[2]).is_err());
        assert!(parse_json(FAILS[3]).is_err());
        assert!(parse_json(FAILS[4]).is_err());
        assert!(parse_json(FAILS[5]).is_err());
        assert!(parse_json(FAILS[6]).is_err());
        assert!(parse_json(FAILS[7]).is_err());
        assert!(parse_json(FAILS[8]).is_err());
        assert!(parse_json(FAILS[9]).is_err());
        assert!(parse_json(FAILS[10]).is_err());
        assert!(parse_json(FAILS[11]).is_err());
        assert!(parse_json(FAILS[12]).is_err());
        assert!(parse_json(FAILS[13]).is_err());
        assert!(parse_json(FAILS[14]).is_err());
        assert!(parse_json(FAILS[15]).is_err());
        assert!(parse_json(FAILS[16]).is_err());
        //assert!(parse_json(FAILS[17]).is_err());
        //^^^ arguable case
        assert!(parse_json(FAILS[18]).is_err());
        assert!(parse_json(FAILS[19]).is_err());
        assert!(parse_json(FAILS[20]).is_err());
        assert!(parse_json(FAILS[21]).is_err());
        assert!(parse_json(FAILS[22]).is_err());
        assert!(parse_json(FAILS[23]).is_err());
        assert!(parse_json(FAILS[24]).is_err());
        assert!(parse_json(FAILS[25]).is_err());
        assert!(parse_json(FAILS[26]).is_err());
        assert!(parse_json(FAILS[27]).is_err());
        assert!(parse_json(FAILS[28]).is_err());
        assert!(parse_json(FAILS[29]).is_err());
        assert!(parse_json(FAILS[30]).is_err());
        assert!(parse_json(FAILS[31]).is_err());
        assert!(parse_json(FAILS[32]).is_err());
    }

    #[test]
    fn json_cpp_passes() {
        assert!(parse_json(PASSES[0]).is_ok());
        assert!(parse_json(PASSES[1]).is_ok());
        assert!(parse_json(PASSES[2]).is_ok());
    }
}