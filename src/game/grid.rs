use super::DIM;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Grid<T>([T; DIM * DIM]);

impl<T: Default + Copy> Grid<T> {
    pub fn new() -> Self {
        Self([T::default(); DIM * DIM])
    }

    pub fn get(&self, x: i8, y: i8) -> T {
        self.0[x as usize + y as usize * DIM]
    }

    pub fn get_opt(&self, x: i8, y: i8) -> Option<T> {
        if x < 0 || x >= DIM as i8 || y < 0 || y >= DIM as i8 {
            None
        } else {
            Some(self.0[x as usize + y as usize * DIM])
        }
    }

    pub fn set(&mut self, x: i8, y: i8, value: T) {
        self.0[x as usize + y as usize * DIM] = value;
    }

}

impl<T: ToString> ToString for Grid<T> {
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(DIM * (DIM + 1));
        for i in 0..self.0.len() {
            if i > 0 && i % DIM == 0 {
                s.push('\n');
            }
            s.push_str(&self.0[i].to_string());
        }
        s
    }
}
