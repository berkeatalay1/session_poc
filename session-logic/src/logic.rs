use uuid::Uuid;
use std::{thread, time};

use crate::{set_session_id, get_session_id};


pub fn test_thread_local() {

    let started_uuid = get_session_id();

    let new_uuid = Uuid::new_v4();

    println!("---------------------- 1 - {started_uuid} -- new_uuid = {new_uuid}  -------------------------");

    set_session_id(new_uuid.to_string());


    let ten_millis = time::Duration::from_secs(10);

    thread::sleep(ten_millis);

    let new_uuid_getted = get_session_id();

    println!("---------------------- 2 - {started_uuid} -- new_uuid = {new_uuid} -- uuid_on_session; {new_uuid_getted}  -------------------------");

}
