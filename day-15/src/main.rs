use std::collections::HashSet;

struct Sensor {
    x: i32,
    y: i32,
    ray: i32,
}

impl Sensor {
    fn new((x, y): (i32, i32), beacon: (i32, i32)) -> Sensor {
        Sensor {
            x,
            y,
            ray: dist((x, y), beacon),
        }
    }

    fn covers(&self, p: (i32, i32)) -> bool {
        dist((self.x, self.y), p) <= self.ray
    }

    fn x(&self) -> i32 {
        self.x
    }

    fn x_min(&self) -> i32 {
        self.x - self.ray
    }

    fn x_max(&self) -> i32 {
        self.x + self.ray
    }

    fn perimeter(&self) -> HashSet<(i32, i32)> {
        let mut p = HashSet::new();
        for d in 0..=(self.ray + 1) {
            p.insert((self.x + d, self.y + self.ray - d + 1));
            p.insert((self.x + d, self.y - self.ray + d - 1));
            p.insert((self.x - d, self.y + self.ray - d + 1));
            p.insert((self.x - d, self.y - self.ray + d - 1));
        }
        p
    }
}

fn dist((ax, ay): (i32, i32), (bx, by): (i32, i32)) -> i32 {
    (ax - bx).abs() + (ay - by).abs()
}

fn main() {
    let data = vec![
        ((1638847, 3775370), (2498385, 3565515)),
        ((3654046, 17188), (3628729, 113719)),
        ((3255262, 2496809), (3266439, 2494761)),
        ((3743681, 1144821), (3628729, 113719)),
        ((801506, 2605771), (1043356, 2000000)),
        ((2933878, 5850), (3628729, 113719)),
        ((3833210, 12449), (3628729, 113719)),
        ((2604874, 3991135), (2498385, 3565515)),
        ((1287765, 1415912), (1043356, 2000000)),
        ((3111474, 3680987), (2498385, 3565515)),
        ((2823460, 1679092), (3212538, 2537816)),
        ((580633, 1973060), (1043356, 2000000)),
        ((3983949, 236589), (3628729, 113719)),
        ((3312433, 246388), (3628729, 113719)),
        ((505, 67828), (-645204, 289136)),
        ((1566406, 647261), (1043356, 2000000)),
        ((2210221, 2960790), (2498385, 3565515)),
        ((3538385, 1990300), (3266439, 2494761)),
        ((3780372, 2801075), (3266439, 2494761)),
        ((312110, 1285740), (1043356, 2000000)),
        ((51945, 2855778), (-32922, 3577599)),
        ((1387635, 2875487), (1043356, 2000000)),
        ((82486, 3631563), (-32922, 3577599)),
        ((3689149, 3669721), (3481800, 4169166)),
        ((2085975, 2190591), (1043356, 2000000)),
        ((712588, 3677889), (-32922, 3577599)),
        ((22095, 3888893), (-32922, 3577599)),
        ((3248397, 2952817), (3212538, 2537816)),
    ];

    let sensors: Vec<Sensor> = data.iter().map(|(s, b)| Sensor::new(*s, *b)).collect();

    let beacons: HashSet<(i32, i32)> = data.iter().map(|(_, b)| *b).collect();
    let rightmost = sensors.iter().max_by_key(|s| s.x()).unwrap();
    let leftmost = sensors.iter().min_by_key(|s| s.x()).unwrap();

    let y = 2_000_000;
    let coverage = (leftmost.x_min()..=rightmost.x_max())
        .into_iter()
        .filter(|x| !beacons.contains(&(*x, y)))
        .filter(|x| sensors.iter().any(|s| s.covers((*x, y))))
        .count();

    dbg!(coverage);

    let perimeters = sensors
        .iter()
        .enumerate()
        .fold(HashSet::<(i32, i32)>::new(), |t, (i, s)| {
            println!("#{}", i);
            &t | &s.perimeter()
        });

    let l = 4_000_000;
    let p = perimeters
        .iter()
        .filter(|(x, y)| x >= &0 && x <= &l && y >= &0 && y <= &l)
        .find(|p| !sensors.iter().any(|s| s.covers(**p)))
        .unwrap();

    dbg!(p);
}
