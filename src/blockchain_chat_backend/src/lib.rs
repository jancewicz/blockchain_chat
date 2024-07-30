use std::{cell::RefCell, collections::HashMap};

// Principal can be used as kind of user id
use candid::Principal;
use ic_cdk::caller;

thread_local! {
    static NOTES: RefCell<HashMap<Principal, Vec<String>>> = RefCell::default();
    static CHAT: RefCell<HashMap<[Principal; 2], Vec<String>>> = RefCell::default();
}

#[ic_cdk::query]
fn get_chat(first_user: Principal, second_user: Principal) -> Option<Vec<String>> {
    CHAT.with_borrow(|notes| notes.get(&[first_user, second_user]).cloned())
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

#[ic_cdk::update]
fn add_chat_message(message: String, user2: Principal) {
    let user1 = caller();

    if user1 == Principal::anonymous() {
        panic!("User is anonymous Principal");
    }

    let mut principals = [user1, user2];
    principals.sort();

    CHAT.with_borrow_mut(|chats| {
        let mut_chats = chats.get_mut(&principals);

        if let Some(chat_messages) = mut_chats {
            chat_messages.push(message);
        } else {
            chats.insert(principals, vec![message]);
        }
    })
}
