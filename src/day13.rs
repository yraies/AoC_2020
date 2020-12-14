use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day13)]
pub fn generate_ids(input: &str) -> (u128, Vec<Option<u128>>) {
    let mut lines = input.lines();
    let t_ref = lines.next().unwrap().parse::<u128>().unwrap();
    let dep_times = lines.next().unwrap().split(",")
        .map(|id| id.parse::<u128>().map_or(None, |i| Some(i))).collect::<Vec<_>>();
    //println!("{:?}", dep_times);
    (t_ref, dep_times)
}

#[aoc(day13, part1)]
pub fn get_earliest_dep_time(input: &(u128, Vec<Option<u128>>)) -> String {
    let (id, wait_time) = input.1.iter().flatten().map(|&id| {
        (id, id - input.0.rem_euclid(id))
    }).min_by_key(|(_, rem)| *rem).unwrap();

    format!("id:{} wait:{} result:{}", id, wait_time, id * wait_time)
}

#[aoc(day13, part2)]
pub fn get_timestamp_for_constraint(input: &(u128, Vec<Option<u128>>)) -> String {
    let reqs = input.1.iter().enumerate().filter_map(|val| match val {
        (_, None) => None,
        (dt, Some(id)) => Some((dt as u128, *id))
    }).collect::<Vec<(u128, u128)>>();

    let mut t0 = 0;
    let mut stepsize = 1;
    let mut boundary = 0;
    let mut last = None;
    let reqcount = reqs.len();

    'outer: while boundary != reqcount {
        //println!("Testing t0={}: ", t0);
        let mut i = 0;
        'check: while i <= boundary {
            let remainder = (t0 + reqs[i].0) % reqs[i].1;
            //print!("bus {} (d{}): should depart at {}", reqs[i].1, reqs[i].0, t0 + reqs[i].0);
            if remainder == 0 {
                if i < boundary {
                    //println!(" and does - ");
                    i += 1;
                    continue 'check;
                } else if i == boundary {
                    match last {
                        None => {
                            println!("Adding new constraint! {}\n", boundary);
                            last = Some(t0);
                            t0 += stepsize;
                            continue 'outer;
                        }
                        Some(prev) => {
                            println!("Setting stepsize: {} at {}\n", stepsize, t0);
                            stepsize = stepsize.max(t0 - prev);
                            t0 = t0 % stepsize;
                            last = None;
                            boundary += 1;
                            continue 'outer;
                        }
                    }
                }
            } else {
                //println!("would to wait {}\n", remainder);
                t0 += stepsize;
                continue 'outer;
            }
        }
    }

    format!("{:?}", t0)
}

#[aoc(day13, part2, alt = yesbutno)]
pub fn get_timestamp_for_constraint_yesbutno(input: &(u128, Vec<Option<u128>>)) -> String {
    let _reqs = input.1.iter().enumerate().filter_map(|val| match val {
        (_, None) => None,
        (dt, Some(id)) => Some((dt as u128, *id))
    }).collect::<Vec<(u128, u128)>>();

    let i = 0;
    let max = (0, 0);
    /*
        let max = reqs.iter().max_by_key(|(_, k)| *k).unwrap();
        'search: loop {
            i += 1;
            let t0 = max.1 * i - max.0;
            if i % 100000 == 0 {
                println!("Testing iteration {} with t0={}", i, t0);
            }
            //println!("Testing t0={}: ", t0);
            for (dt, id) in reqs.iter() {
                //print!("bus {}({}): departs at {}", id, dt, t0 + dt);
                if (t0 + dt) % id != 0 {
                    //println!(" but would to wait {}\n", (t0+dt) % id);
                    continue 'search;
                } else {
                    //println!("");
                }
            }
            //println!();
            break 'search;
        }*/

    format!("{:?}", i * max.1 - max.0)
}

#[cfg(test)]
mod tests {
    use crate::day13::{get_timestamp_for_constraint, generate_ids};

    #[test]
    fn it_works_1() {
        assert_eq!(get_timestamp_for_constraint(&generate_ids("1\n1,2,5,11")), "63");
    }

    #[test]
    fn it_works_2() {
        assert_eq!(get_timestamp_for_constraint(&generate_ids("1\n17,x,13,19")), "3417");
    }

    #[test]
    fn it_works_3() {
        assert_eq!(get_timestamp_for_constraint(&generate_ids("1\n67,7,59,61")), "754018");
    }

    #[test]
    fn it_works_4() {
        assert_eq!(get_timestamp_for_constraint(&generate_ids("1\n67,x,7,59,61")), "779210");
    }
}