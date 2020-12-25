use regex::Regex;
use std::collections::BTreeMap;
use std::iter::FromIterator;

pub struct Passport {
    pub byr: i32,
    pub iyr: i32,
    pub eyr: i32,
    pub hgt: i32,
    pub hgt_unit: String,
    pub hcl: String,
    pub ecl: String,
    pub pid: String,
    pub pid_num: i32,
    pub cid: String,
    pub cid_num:i32,
}

impl Passport {
    pub fn new(passport: &BTreeMap<String, String>, full_check: bool) -> Result<Passport, &'static str> {


        if !(passport.contains_key("byr")
            && passport.contains_key("iyr")
            && passport.contains_key("eyr")
            && passport.contains_key("hgt")
            && passport.contains_key("hcl")
            && passport.contains_key("ecl")
            && passport.contains_key("pid")) {
            return Err("Invalid passport");
        }

        let mut res: Passport = Passport {byr: 0, iyr: 0, eyr: 0, hgt: 0, hgt_unit: String::from(""), hcl: String::from(""), ecl: String::from(""), pid: String::from(""), pid_num: 0, cid: String::from(""), cid_num: -1};

        if let Some(byr_s) = passport.get("byr") {
            res.byr = byr_s.parse::<i32>().unwrap();

            if full_check && !(res.byr >= 1920 && res.byr <= 2002) {
                return Err("byr not in range");
            }
        } else {
            return Err("no byr");
        }

        if let Some(iyr_s) = passport.get("iyr") {
            res.iyr = iyr_s.parse::<i32>().unwrap();
            if full_check && !(res.iyr >= 2010 && res.iyr <= 2020) {
                return Err("iyr not in range");
            }
        } else {
            return Err("no iyr");
        }

        if let Some(eyr_s) = passport.get("eyr") {
            res.eyr = eyr_s.parse::<i32>().unwrap();
            if full_check && !(res.eyr >= 2020 && res.eyr <= 2030) {
                return Err("eyr not in range");
            }
        } else {
            return Err("no eyr");
        }

        if let Some(hgt_s) = passport.get("hgt") {
            let value_s: String = hgt_s.chars().take(hgt_s.len() - 2).collect();
            res.hgt_unit = hgt_s.chars().skip(hgt_s.len() - 2).collect();

            if let Ok(value) = value_s.parse::<i32>() {
                res.hgt = value;
                
                if full_check {
                    if res.hgt_unit == "in"  || res.hgt_unit == "cm" {
                        if (res.hgt_unit == "in" && !(value >= 59 && value <= 76)) || ( res.hgt_unit == "cm" && !(value >= 150 && value <= 193)) {
                            return Err("hgt not in range");
                        }
                    } else {
                        return Err("hgt unit invalid");
                    }
                }
            }
        } else {
            return Err("no hgt");
        }

        if let Some(hcl_s) = passport.get("hcl") {
            let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
            if full_check && !(re.is_match(&hcl_s) && hcl_s.len()==7) {
                return Err("hcl invalid");
            }
            res.hcl = String::from(hcl_s);
        } else {
            return Err("no hcl");
        }

        if let Some(ecl) = passport.get("ecl") {
            res.ecl = String::from(ecl);
            if full_check && !(ecl == "amb"
                || ecl == "blu"
                || ecl == "brn"
                || ecl == "gry"
                || ecl == "grn"
                || ecl == "hzl"
                || ecl == "oth") {
                    return Err("ecl invalid");
            }
        } else {
            return Err("no ecl");
        }

        if let Some(pid_s) = passport.get("pid") {
            res.pid = String::from(pid_s);
            if full_check && pid_s.len() != 9 {
                return Err("pid invalid length");
            }

            if let Ok(pid) = pid_s.parse::<i32>() {
                res.pid_num = pid;
            } else {
                if full_check {
                    return Err("pid not a number");
                }
            }
        } else {
            return Err("no pid");
        }

        if let Some(cid_s) = passport.get("cid") {
            res.cid = String::from(cid_s);
            res.cid_num = res.cid.parse::<i32>().unwrap();
        }

        Ok(res)
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        println!("-------------------------------");
        println!("Birth year:      {}", self.byr);
        println!("Issue year:      {}", self.iyr);
        println!("Expiration year: {}", self.eyr);
        println!("Height:          {} {}", self.hgt, self.hgt_unit);
        println!("Hair color:      {}", self.hcl);
        println!("Eye color:       {}", self.ecl);
        println!("Passport id:     {}", self.pid);
        println!("Country id:      {}", self.cid);
        println!("-------------------------------");
    }
}

#[allow(dead_code)]
pub fn load_passports(input: &Vec<String>, full_check: bool) -> Vec<Passport> {
    let mut res: Vec<Passport> = vec![];

    let mut current_passport: BTreeMap<String, String> = BTreeMap::new();

    for line in input {
        if line == "" {
            // part 1 check
            if let Ok(valid_pp) = Passport::new(&current_passport, full_check) {
                res.push(valid_pp);
            }

            // start new passport
            current_passport = BTreeMap::new();
        } else {
            let items: Vec<&str> = line.split(' ').collect();
            for item in items {
                let keyvaluepair: Vec<&str> = item.split(':').collect();
                current_passport
                    .insert(String::from(keyvaluepair[0]), String::from(keyvaluepair[1]));
            }
        }
    }
    res
}

#[allow(dead_code)]
pub fn create_country_histogram(passports: &Vec<Passport>) -> Vec<(i32,u32)> {
    let mut cids: BTreeMap<i32, u32> = BTreeMap::new();
    for pp in passports {
        let cid = cids.entry(pp.cid_num).or_insert(0);
        *cid += 1
    }

    let mut v: Vec<(i32,u32)> = Vec::from_iter(cids);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));
    v
}