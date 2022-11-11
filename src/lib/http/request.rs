use std::str;

pub fn triage_request(buffer : &[u8; 1024]) {
    let v = buffer.to_vec();
    let req = str::from_utf8(&v).unwrap();
    let split = req.split(" ");

    let mut e = split.enumerate();
    
    let req_type  = e.next();
    let req_path = e.next();

    println!("{:#?}", req_type);
    println!("{:#?}", req_path);
}

// pub fn triage_path(path : Option<usize, &str>) -> (Request::Status, String) {

// }