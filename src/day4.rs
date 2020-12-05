use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct CredBuilder {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>,
}

#[derive(Debug)]
pub struct Cred {
    byr: i32,
    iyr: i32,
    eyr: i32,
    hgt: i32,
    hcl: String,
    ecl: String,
    pid: u64,
    cid: Option<u64>,
}

impl CredBuilder {
    pub fn new() -> CredBuilder {
        CredBuilder {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }
    pub fn has_all_fieds(&self) -> bool {
        self.byr.is_some() &&
            self.eyr.is_some() &&
            self.iyr.is_some() &&
            self.hgt.is_some() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some()
    }

    pub fn build(&self) -> Result<Cred, String> {
        let byr = self.byr.as_ref()
            .ok_or("Missing byr".to_string())
            .and_then(|s| {
                s.parse::<i32>()
                    .map_err(|e| format!("Parse byr: {}", e))
                    .and_then(|yr| if 1920 <= yr && 2002 >= yr { Ok(yr) } else { Err("byr not in Range".to_string()) })
            });

        let iyr = self.iyr.as_ref()
            .ok_or("Missing iyr".to_string())
            .and_then(|s| {
                s.parse::<i32>()
                    .map_err(|e| format!("Parse iyr: {}", e))
                    .and_then(|yr| if 2010 <= yr && 2020 >= yr { Ok(yr) } else { Err("iyr not in Range".to_string()) })
            });

        let eyr = self.eyr.as_ref()
            .ok_or("Missing eyr".to_string())
            .and_then(|s| {
                s.parse::<i32>()
                    .map_err(|e| format!("Parse eyr: {}", e))
                    .and_then(|yr| if 2020 <= yr && 2030 >= yr { Ok(yr) } else { Err("eyr not in Range".to_string()) })
            });

        let hgt = self.hgt.as_ref()
            .ok_or("Missing hgt".to_string())
            .and_then(|s| {
                match s[s.len() - 2..].as_ref() {
                    "in" => {
                        if let Ok(height) = s[..s.len() - 2].parse::<i32>() {
                            if height >= 59 && height <= 76 {
                                Ok(height)
                            } else { Err("hgt not in range".to_string()) }
                        } else { Err("hgt not number".to_string()) }
                    }
                    "cm" => {
                        if let Ok(height) = s[..s.len() - 2].parse::<i32>() {
                            if height >= 150 && height <= 193 {
                                Ok(height)
                            } else { Err("hgt not in range".to_string()) }
                        } else { Err("hgt not number".to_string()) }
                    }
                    _ => Err("hgt is missing unit".to_string())
                }
            });

        let hcl = self.hcl.as_ref()
            .ok_or("Missing hcl".to_string())
            .and_then(|s|
                if &s[0..1] == "#" && s[1..].chars().all(|c|['1','2','3','4','5','6','7','8','9','0','a','b','c','d','e','f'].contains(&c)) {
                    Ok(s.to_string())
                } else {
                    Err("ecl invalid".to_string())
                }
            );

        let ecl = self.ecl.as_ref()
            .ok_or("Missing ecl".to_string())
            .and_then(|s|
                if ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&&**s) {
                    Ok(s.to_string())
                } else {
                    Err("ecl invalid".to_string())
                }
            );

        let pid = self.pid.as_ref()
            .ok_or("Missing pid".to_string())
            .and_then(|s| {
                if s.len() == 9 && s.chars().all(|c|['1','2','3','4','5','6','7','8','9','0'].contains(&c)){
                    Ok(s.parse::<u64>().unwrap())
                } else {
                    Err("pid invalid".to_string())
                }
            });

        let pre_cid = self.cid.as_ref().ok_or("".to_string())
            .and_then(|s| s.parse::<u64>().map_err(|e| format!("Parse cid: {}", e)));
        let cid = if let Ok(val) = pre_cid {
            Some(val)
        } else {
            None
        };
        let e = byr.as_ref().err().or(
            iyr.as_ref().err().or(
                eyr.as_ref().err().or(
                    hgt.as_ref().err().or(
                        hcl.as_ref().err().or(
                            ecl.as_ref().err().or(
                                pid.as_ref().err()))))));
        if let Some(err) = e {
            println!("{:?}", &self);
            println!("ERR {:?}", err);
        }
        Ok(Cred {
            byr: byr?,
            iyr: iyr?,
            eyr: eyr?,
            hgt: hgt?,
            hcl: hcl?,
            ecl: ecl?,
            pid: pid?,
            cid,
        })
    }
}

#[aoc_generator(day4)]
pub fn generate_creds(input: &str) -> Vec<CredBuilder> {
    let mut results = vec!();
    let mut builder = CredBuilder::new();
    let mut ctr = 0;
    for line in input.lines() {
        if line.eq("") {
            ctr += 1;
            if builder.has_all_fieds() {
                results.push(builder);
            }
            builder = CredBuilder::new();
        } else {
            //println!("line: {}", line);
            for part in line.split_whitespace() {
                //println!("part: {}", part);
                let mut split = part.split(":");
                if let Some(head) = split.next() {
                    let foo = split.next().unwrap().to_string();
                    //println!("{}: {}", head, foo);
                    match head {
                        "byr" => builder.byr = Some(foo),
                        "iyr" => builder.iyr = Some(foo),
                        "eyr" => builder.eyr = Some(foo),
                        "hgt" => builder.hgt = Some(foo),
                        "hcl" => builder.hcl = Some(foo),
                        "ecl" => builder.ecl = Some(foo),
                        "pid" => builder.pid = Some(foo),
                        "cid" => builder.cid = Some(foo),
                        _ => (),
                    }
                }
            }
        }
    }
    println!("looked at: {}", ctr);
    if builder.has_all_fieds() {
        results.push(builder);
    }
    results
}

#[aoc(day4, part1)]
pub fn count_pseudovalid_creds(creds: &[CredBuilder]) -> usize {
    //println!("{:?}", creds);
    creds.len()
}

#[aoc(day4, part2)]
pub fn count_validated_creds(creds: &[CredBuilder]) -> usize {
    creds.iter().map(|cred| cred.build()).filter(|cred| cred.is_ok()).count()
}