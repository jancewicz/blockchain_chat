use std::{cell::RefCell, collections::HashMap};

// Principal can be used as kind of user id
use candid::Principal;
use ic_cdk::caller;

thread_local! {
    static NOTES: RefCell<HashMap<Principal, Vec<String>>> = RefCell::default();
}

#[ic_cdk::query]
fn get_notes(user: Principal) -> Option<Vec<String>> {
    NOTES.with_borrow(|notes| notes.get(&user).cloned())
}

#[ic_cdk::update]
fn add_note(note: String) {
    let user = caller();
    if user == Principal::anonymous() {
        panic!("User is anonymous Principal");
    }

    NOTES.with_borrow_mut(|notes| {
        let mut_notes = notes.get_mut(&user);

        if let Some(notes_vec) = mut_notes {
            notes_vec.push(note);
        } else {
            notes.insert(user, vec![note]);
        }
    })
}
