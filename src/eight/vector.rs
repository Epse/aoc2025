#[allow(dead_code)]
use std::{fmt::Display, ops::BitXor};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vector3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vector3 {
    pub fn abs(&self) -> u64 {
        ((self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as u64).isqrt()
    }
}

impl Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

/// Actually computes euclidian distance
impl BitXor for Vector3 {
    type Output = i64;

    fn bitxor(self, other: Self) -> Self::Output {
        &self ^ &other
    }
}

impl<'a, 'b> BitXor<&'a Vector3> for &'b Vector3 {
    type Output = i64;

    /// Returns squared distance
    fn bitxor(self, other: &Vector3) -> Self::Output {
        (self.x - other.x).pow(2)
            + (self.y - other.y).pow(2)
            + (self.z - other.z).pow(2)
    }
}

impl<'a> TryFrom<&'a str> for Vector3 {
    type Error = &'static str;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let nums: Vec<i64> = value
            .split(',')
            .filter_map(|num| num.parse::<i64>().ok())
            .take(3)
            .collect();

        if nums.len() != 3 {
            Err("Incorrect number of parseable numbers")
        } else {
            Ok(Vector3 {
                x: nums[0],
                y: nums[1],
                z: nums[2],
            })
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_distance() {
        let a = Vector3{x: 0, y: 0, z: 0};
        let b = Vector3{x: 90000, y: 0, z: 0};
        let dist = &a ^ &b;
        assert_eq!(90000i64.pow(2), dist);
        assert_eq!(90000i64.pow(2), b ^ a);
    }
}
