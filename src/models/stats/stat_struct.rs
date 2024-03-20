use std::rc::Rc;

use crate::models::data::listens::UserListen;

pub trait StatStruct {
    fn push(&mut self, value: Rc<UserListen>);

    fn get_mbid(&self) -> &str;

    fn new(mbid: String) -> Self;
}
