use std::fs;

#[derive(Copy, Clone)]
enum Height {
    Centimeters(usize),
    Inches(usize),
}

struct Passport<'a> {
    byr: usize,
    iyr: usize,
    eyr: usize,
    hgt: Height,
    hcl: &'a str,
    ecl: &'a str,
    pid: &'a str,
}

impl<'a> Passport<'a> {
    fn is_valid(&self) -> bool {
        is_byr_valid(self.byr)
            && is_iyr_valid(self.iyr)
            && is_eyr_valid(self.eyr)
            && is_hgt_valid(self.hgt)
            && is_hcl_valid(self.hcl)
            && is_ecl_valid(self.ecl)
            && is_pid_valid(self.pid)
    }
}

impl<'a> From<&'a str> for Passport<'a> {
    fn from(description: &'a str) -> Self {
        let mut byr: usize = Default::default();
        let mut iyr: usize = Default::default();
        let mut eyr: usize = Default::default();
        let mut hgt: Height = Height::Centimeters(0);
        let mut hcl: &str = Default::default();
        let mut ecl: &str = Default::default();
        let mut pid: &str = Default::default();

        for p in description.split_ascii_whitespace() {
            if p.contains("byr") {
                byr = p.strip_prefix("byr:").unwrap().parse::<usize>().unwrap();
            } else if p.contains("iyr") {
                iyr = p.strip_prefix("iyr:").unwrap().parse::<usize>().unwrap();
            } else if p.contains("eyr") {
                eyr = p.strip_prefix("eyr:").unwrap().parse::<usize>().unwrap();
            } else if p.contains("hgt") {
                if p.contains("cm") {
                    hgt = Height::Centimeters(
                        p.strip_prefix("hgt:")
                            .unwrap()
                            .strip_suffix("cm")
                            .unwrap()
                            .parse::<usize>()
                            .unwrap(),
                    );
                } else if p.contains("in") {
                    hgt = Height::Inches(
                        p.strip_prefix("hgt:")
                            .unwrap()
                            .strip_suffix("in")
                            .unwrap()
                            .parse::<usize>()
                            .unwrap(),
                    );
                }
            } else if p.contains("hcl:#") {
                hcl = p.strip_prefix("hcl:#").unwrap();
            } else if p.contains("ecl") {
                ecl = p.strip_prefix("ecl:").unwrap();
            } else if p.contains("pid") {
                pid = p.strip_prefix("pid:").unwrap();
            }
        }

        Self {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
        }
    }
}

fn is_description_valid(description: &str) -> bool {
    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|w| description.contains(w))
}

fn is_byr_valid(byr: usize) -> bool {
    byr >= 1920 && byr <= 2002
}

fn is_iyr_valid(iyr: usize) -> bool {
    iyr >= 2010 && iyr <= 2020
}

fn is_eyr_valid(eyr: usize) -> bool {
    eyr >= 2020 && eyr <= 2030
}

fn is_hgt_valid(hgt: Height) -> bool {
    match hgt {
        Height::Centimeters(cm) => cm >= 150 && cm <= 193,
        Height::Inches(inch) => inch >= 59 && inch <= 76,
    }
}

fn is_hcl_valid(hcl: &str) -> bool {
    hcl.len() == 6 && hcl.chars().all(|c| c.is_digit(16))
}

fn is_ecl_valid(ecl: &str) -> bool {
    ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter()
        .any(|&c| c == ecl)
}

fn is_pid_valid(pid: &str) -> bool {
    pid.len() == 9 && pid.parse::<usize>().is_ok()
}

fn part_2(valid_descriptions: &[Passport]) -> usize {
    valid_descriptions.iter().filter(|p| p.is_valid()).count()
}

fn main() {
    let content = fs::read_to_string("input").unwrap();
    let passports: Vec<&str> = content.split_terminator("\n\n").collect();

    let valid_descriptions: Vec<Passport> = passports
        .iter()
        .filter(|p| is_description_valid(p))
        .map(|&p| Passport::from(p))
        .collect();

    assert_eq!(valid_descriptions.len(), 228); // part_1
    assert_eq!(part_2(&valid_descriptions), 175);
}
