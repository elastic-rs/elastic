#[test]
fn object() {
    panic!("implement")
}

#[test]
fn string() {
    panic!("implement")
}

#[test]
fn string_with_single_point() {
    //Try to deserialize a geo_point string with 1 entry
    //Should return Err("malformed: should be '{y},{x}'")
    panic!("implement")
}

#[test]
fn string_with_invalid_nums() {
    //Try to deserialize a geo_point with comma, but non-number
    //Should return Err("malformed: `x` and `y` should be floats")
    panic!("implement")
}

#[test]
fn hash() {
    panic!("implement")
}

#[test]
fn array() {
    panic!("implement")
}
