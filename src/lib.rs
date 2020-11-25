mod sorted_string_table;
use std::fs::File;
use std::io;
use std::collections::HashMap;

struct Point {
    lat: f64,
    long: f64
}

pub struct Geobase {
    persistence: File,
    data: HashMap<String, Point>
}

impl Geobase {
    pub fn open(path: &str) -> io::Result<Geobase> {
        let persistence = File::create(path)?;
        let data = HashMap::new();
        Ok(Geobase { persistence, data })
    }

    pub fn set(key: &str, pt: &Point) {
        values.insert(key.to_string(), *pt);
    }

    pub fn get(key: &str) -> Option<Point> {
        values.get(key)
    }
}

#[cfg(test)]
mod geobase_tests {
    use super::*;

    #[test]
    fn test_open() {
        let gb: Geobase = match Geobase::open("./datastore") {
            Ok(gb) => gb,
            Err(err) => panic!("Could not open file : {}", err)
        };
    }
}
