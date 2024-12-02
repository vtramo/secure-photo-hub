use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Page<T> 
where 
    T: Debug + Clone
{
    data: Vec<T>,
    current_page: u32,
    per_page: u32,
}

impl<T> Page<T> where T: Debug + Clone {
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
    pub fn current_page(&self) -> u32 {
        self.current_page
    }
    pub fn per_page(&self) -> u32 {
        self.per_page
    }
    pub fn new(data: Vec<T>, current_page: u32, per_page: u32) -> Self {
        Self { data, current_page, per_page }
    }
}
