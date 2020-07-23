use proconio::input;
use std::str::FromStr;

fn main() {
    input! {
        n: usize,
        mut cases: [String; n],
    }

    let mut ac = 0;
    let mut wa = 0;
    let mut tle = 0;
    let mut re = 0;

    for case in cases {
        let j = JudgeResult::from_str(case.trim()).unwrap();
        match j {
            JudgeResult::AC => ac += 1,
            JudgeResult::WA => wa += 1,
            JudgeResult::TLE => tle += 1,
            JudgeResult::RE => re += 1,
        }
    }

    println!("AC x {}", ac);
    println!("WA x {}", wa);
    println!("TLE x {}", tle);
    println!("RE x {}", re);
}

enum JudgeResult {
    AC,
    WA,
    TLE,
    RE,
}

impl FromStr for JudgeResult {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AC" => Ok(JudgeResult::AC),
            "WA" => Ok(JudgeResult::WA),
            "TLE" => Ok(JudgeResult::TLE),
            "RE" => Ok(JudgeResult::RE),
            _ => Err(()),
        }
    }
}
