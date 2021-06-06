use std::fmt;

pub fn select_in_menu<T: fmt::Display + Clone>(
    _description: &String,
    collection: &Vec<T>,
) -> Option<T> {
    if collection.is_empty() {
        return None;
    }
    let mut menu = youchoose::Menu::new(collection.iter());
    let index: usize = menu.show().first().unwrap().clone();
    Some(collection[index].clone())
}
