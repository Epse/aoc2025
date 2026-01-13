use super::Vector3;

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Link {
    lesser: Vector3,
    greater: Vector3,
}

#[allow(dead_code)]
impl Link {
    pub fn new(a: Vector3, b: Vector3) -> Self {
        if a.abs() > b.abs() {
            Self {
                lesser: b,
                greater: a,
            }
        } else {
            Self {
                lesser: a,
                greater: b,
            }
        }
    }

    pub fn contains(&self, other: &Vector3) -> bool {
        self.lesser == *other || self.greater == *other
    }

    pub fn lesser(&self) -> &Vector3 {
        &self.lesser
    }

    pub fn greater(&self) -> &Vector3 {
        &self.greater
    }

    pub fn split(self) -> (Vector3, Vector3) {
        (self.lesser, self.greater)
    }
}
