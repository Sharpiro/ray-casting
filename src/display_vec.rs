#[derive(Debug, Clone)]
pub struct DisplayVec<T>(Vec<T>);

impl<T> DisplayVec<T> {
    pub fn new() -> DisplayVec<T> {
        DisplayVec(Vec::new())
    }
}

impl<T: std::fmt::Display> std::fmt::Display for DisplayVec<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for v in &self.0 {
            write!(f, "{}, ", v)?;
        }
        Ok(())
    }
}

impl<T> std::ops::Deref for DisplayVec<T> {
    type Target = Vec<T>;
    fn deref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T> std::ops::DerefMut for DisplayVec<T> {
    fn deref_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}
