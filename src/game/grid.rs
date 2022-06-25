#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Grid<T, const D: usize>([T; D * D])
where
    [T; D * D]: Sized;

impl<T: Default + Copy, const D: usize> Grid<T, D>
where
    [T; D * D]: Sized,
{
    pub fn new() -> Self {
        Self([T::default(); D * D])
    }

    pub fn get(&self, x: i8, y: i8) -> T {
        self.0[x as usize + y as usize * D]
    }

    pub fn get_opt(&self, x: i8, y: i8) -> Option<T> {
        if x < 0 || x >= D as i8 || y < 0 || y >= D as i8 {
            None
        } else {
            Some(self.0[x as usize + y as usize * D])
        }
    }

    pub fn set(&mut self, x: i8, y: i8, value: T) {
        self.0[x as usize + y as usize * D] = value;
    }
}

impl<T: ToString, const D: usize> ToString for Grid<T, D>
where
    [T; D * D]: Sized,
{
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(D * (D + 1));
        for i in 0..self.0.len() {
            if i > 0 && i % D == 0 {
                s.push('\n');
            }
            s.push_str(&self.0[i].to_string());
        }
        s
    }
}
