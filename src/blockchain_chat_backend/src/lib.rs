pub mod user;

use std::{cell::RefCell, collections::HashMap};

// Principal can be used as kind of user id
use candid::Principal;
use ic_cdk::caller;
use user::UserData;

thread_local! {
    static CHAT: RefCell<HashMap<[Principal; 2], Vec<String>>> = RefCell::default();
    static USERS: RefCell<HashMap<Principal, UserData>> = RefCell::default();
}

#[ic_cdk::query]
fn get_chat(chat_path: [Principal; 2]) -> Option<Vec<String>> {
    CHAT.with_borrow(|notes| notes.get(&chat_path).cloned())
}

#[ic_cdk::update]
fn register(nickname: String) {
    let user = caller();

    if user == Principal::anonymous() {
        panic!("User is anonymous Principal");
    }
    USERS.with_borrow_mut(|users| users.insert(user, UserData::new(nickname)));
}

#[ic_cdk::query]
fn get_users() -> HashMap<Principal, UserData> {
    USERS.with_borrow(|users| users.clone())
}

#[ic_cdk::update]
fn add_chat_message(message: String, user2: Principal) {
    let user1 = caller();

    if user1 == Principal::anonymous() {
        panic!("User is anonymous Principal");
    }

    let is_registered = USERS.with_borrow(|users| users.contains_key(&user1));
    if !is_registered {
        panic!("User not registered")
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
