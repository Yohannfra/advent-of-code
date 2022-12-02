use crate::utils;

fn rock_paper_scisor_win(my: &str, op: &str) -> i32 { // 0:loose 6: win 3:tie
    if my == "Z" && op == "C" ||  my == "Y" && op == "B" || my == "X" && op == "A" {
        return 3; // tie
    }

    if my == "Y" && op == "A" { // Paper vs Rock
        return 6;
    }
    if my == "X" && op == "C" {
        return 6;
    }

    if my == "Z" && op == "B" {
        return 6;
    }

    0
}

pub fn part1(fp: &str) -> Option<String> {
    let fc = utils::read_file_to_vec(fp).unwrap();

    let mut score = 0;

    for l in fc {
        let mut sp = l.split(" ");
        let op = sp.next().unwrap();
        let my = sp.next().unwrap();

        let shape_score = match my {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!()
        };
        score += match rock_paper_scisor_win(my, op) {
            3 => 3 + shape_score,
            6 => 6 + shape_score,
            _ => 0 + shape_score,
        };
    }
    Some(score.to_string())
}

pub fn part2(fp: &str) -> Option<String> {
    let fc = utils::read_file_to_vec(fp).unwrap();

    let mut score = 0;

    for l in fc {
        let mut sp = l.split(" ");
        let op = sp.next().unwrap();
        let res = sp.next().unwrap();

        let my = match (op, res) {
            ("A", "X") => "Z",
            ("A", "Y") => "X",
            ("A", "Z") => "Y",

            ("B", "X") => "X",
            ("B", "Y") => "Y",
            ("B", "Z") => "Z",

            ("C", "X") => "Y",
            ("C", "Y") => "Z",
            ("C", "Z") => "X",
            _ => panic!(),
        };

        let shape_score = match my {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => panic!()
        };
        score += match rock_paper_scisor_win(my, op) {
            3 => 3 + shape_score,
            6 => 6 + shape_score,
            _ => 0 + shape_score,
        };

    }
    Some(score.to_string())
}
