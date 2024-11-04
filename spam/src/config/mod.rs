use std::collections::HashMap;
use tracing::{debug, info, instrument, warn};

#[derive(Debug)]
pub struct Cmap(HashMap<char, Vec<u8>>);

impl Cmap {
    fn new() -> Self {
        debug!("Creating a new Cmap object.");
        Self(HashMap::new())
    }

    pub fn build(s: &str) -> Self {
        let mut h: Cmap = Self::new();

        for line in s.lines() {
            h.0.entry(line.chars().nth(0).unwrap()).or_insert(
                (&line[2..])
                    .split(' ')
                    .map(|x| x.parse::<u8>().expect("Not a u8 number"))
                    .collect(),
            );
        }
        info!("Finished building Cmap.");
        h
    }

    #[instrument(skip(self))]
    fn charcanvas(&self, c: char) -> Option<Vec<String>> {
        self.0.get(&c).map(|v| {
            let n = v.len();
            let mut o = vec![String::with_capacity(n + 2); 7];
            debug!("Created empty canvas string");
            for i in 0..7 {
                o[i].push(' ');
                for j in 0..n {
                    let m = 1u8 << i;
                    o[i].push(match m == (v[j] & m) {
                        true => '*',
                        _ => ' ',
                    })
                }
                o[i].push(' ');
            }
            info!("Finished building canvas string.");
            o
        })
    }

    #[instrument(skip(self))]
    pub fn print(&self, s: &str) {
        let o = s.chars().filter_map(|x| self.charcanvas(x)).fold(
            vec![String::new(); 7],
            |mut a, x| {
                for i in 0..7 {
                    a[i].push_str(&x[i]);
                }
                a
            },
        );
        for s in o {
            println!("{s}")
        }
        info!("Finished canvassing.")
    }

    #[instrument(skip(self))]
    pub fn boolgen(&self, s: &str) -> Vec<bool> {
        let res: Vec<_> = s
            .chars()
            .filter(|x| self.0.contains_key(x))
            .flat_map(|x| {
                self.0.get(&x).unwrap().into_iter().flat_map(|&n| {
                    (0..7).map(move |a| {
                        let m = 1 << a;
                        m == n & m
                    })
                })
            })
            .collect();
        debug!("total {} days", res.len());
        res
    }
}

#[cfg(test)]
mod tests {
    use super::Cmap;

    #[test]
    fn test_increment() {
        // Create an instance of the struct
        let cm = Cmap::build("src/artifacts/testcfg.txt");
        assert_eq!(cm.0[&'a'][3], 55)
    }

    #[test]
    fn test_boolgen() {
        let cm = Cmap::build("src/artifacts/testcfg.txt");
        let mut res = vec![true, true, true, true, true, false, true];
        res.append(&mut res.clone());
        assert_eq!(cm.boolgen("!!"), res);
    }
}
