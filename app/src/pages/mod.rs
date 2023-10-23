pub mod counter;
pub mod home;
pub mod todo;
pub mod todos;

#[derive(Clone)]
pub struct Page {
    pub link: &'static str,
    pub name: &'static str,
}
