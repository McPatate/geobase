mod sorted_string_table;
use std::fs::File;
use std::io;
use std::collections::HashMap;
use std::fmt;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    lat: f64,
    long: f64
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.long, self.lat)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.lat == other.lat && self.long == other.long
    }
}

impl Eq for Point {}

pub struct Geobase {
    persistence: File,
    data: HashMap<String, Point>
}

impl Geobase {
    pub fn open(path: &str) -> io::Result<Self> {
        let persistence = File::create(path)?;
        let data = HashMap::new();
        Ok(Geobase { persistence, data })
    }

    pub fn set(&mut self, key: &str, pt: &Point) {
        self.data.insert(key.to_string(), *pt);
    }

    pub fn get(&self, key: &str) -> Option<Point> {
        match self.data.get(key) {
            Some(v) => Some(*v),
            None => None
        }
    }
}

#[cfg(test)]
mod geobase_tests {
    use super::*;

    #[test]
    fn test_open() {
        let _gb: Geobase = match Geobase::open("./datastore") {
            Ok(gb) => gb,
            Err(err) => panic!("Could not open file : {}", err)
        };
    }

    #[test]
    fn test_set_get() {
        let mut gb = match Geobase::open("./datastore") {
            Ok(gb) => gb,
            Err(err) => panic!("Could not open file : {}", err)
        };
        let pt1 = Point { lat: 3.5, long: 67.8 };
        gb.set("key", &pt1);
        let pt2 = match gb.get("key") {
            Some(p) => p,
            None => Point { lat: 0., long: 0. }
        };
        assert_eq!(pt1, pt2);
    }
}
