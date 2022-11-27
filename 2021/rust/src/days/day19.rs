pub fn solve(part: u32, input: &Vec<&str>, sample: &Vec<&str>) -> i64 {
    assert_eq!(part1(&sample), 79, "Error, sample problem is not solved");
    //assert_eq!(part2(&sample), 900, "Error, sample problem is not solved");
    match part {
        1 => part1(&input),
        2 => part2(&input),
        _ => panic!("Illegal value as input. Valid inputs are 1 and 2, recieved {}", part),
    }
}

fn part2(data: &Vec<&str>) -> i64 {0}

fn part1(data: &Vec<&str>) -> i64 {
    let mut sensors: Vec<Sensor> = vec![];
    for sensor in data {
        let mut beacons: Vec<Beacon> = vec![];
        for line in sensor.lines().skip(1) {
            beacons.push(get_new_beac(line));
        }
        sensors.push(Sensor{beacons});
    }

    println!("{:?}", sensors);
    for sensor in sensors {
        print_sensor(sensor);
    }

    println!("");
    
    /* TODO
     *  1. implement rotation of beacon
     *  2. implement rotation of all beacons on sensor
     *  3. create matching
     */

    0
}

fn print_sensor(sensor: Sensor) {
    for beacon in sensor.beacons {
        println!("{},{},{}", beacon.x, beacon.y, beacon.z);
    }
}

/*
 * total of 24 rotations
 *
 * Cord order:
 * x y z
 * x z y
 * y x z
 * z x y
 * y z x
 * z y x
 *
 * each cord order can have following signs
 *  x  y  z
 *  x -y -z
 * -x  y -z
 * -x -y  z
 */
#[derive(Debug)]
struct Sensor {
    beacons: Vec<Beacon>,
}

#[derive(Debug)]
struct Beacon {
    x: i64,
    y: i64,
    z: i64,
}

fn get_new_beac(line: &str) -> Beacon {
    let values: Vec<&str> = line.split(",").collect::<Vec<&str>>();

    if values.len() != 3 {
        println!("Error wrong number of values");
    }
    let x = values[0].parse().unwrap();
    let y = values[1].parse().unwrap();
    let z = values[2].parse().unwrap();
    Beacon{x,y,z}
}


