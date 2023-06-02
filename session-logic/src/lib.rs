pub mod logic;

use std::cell::RefCell;



thread_local! {
    pub static SESSION_ID: RefCell<String> 
        = RefCell::new("SESSION-ONE".to_string());
}
    
pub fn set_session_id(_session_id: String) {
    SESSION_ID.with(|session| {
    *session.borrow_mut() = _session_id
    });
}

pub fn get_session_id() -> String {
    SESSION_ID.with(|session| session.borrow().clone())
}
