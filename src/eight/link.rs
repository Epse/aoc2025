use super::Vector3;

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Link {
    first: Vector3,
    second: Vector3,
}

// No longer does any guarantees about first and second for perf

#[allow(dead_code)]
impl Link {
    pub fn new(a: Vector3, b: Vector3) -> Self {
        Self {
            first: b,
            second: a,
        }
    }

    pub fn contains(&self, other: &Vector3) -> bool {
        self.first == *other || self.second == *other
    }

    pub fn lesser(&self) -> &Vector3 {
        &self.first
    }

    pub fn greater(&self) -> &Vector3 {
        &self.second
    }

    pub fn split(self) -> (Vector3, Vector3) {
        (self.first, self.second)
    }
}
