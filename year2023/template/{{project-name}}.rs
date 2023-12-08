use crate::{utils, Day};

pub struct {{project-name | capitalize}} {}

impl Day for {{project-name| capitalize}} {
    fn part1(&self, file: String) -> String {
        let lines = utils::read_lines(format!("./src/days/{{project-name}}/files/{file}.txt").as_str());


        "".to_string()
    }

    fn part2(&self, file: String) -> String {
        let lines = utils::read_lines(format!("./src/days/{{project-name}}/files/{file}.txt").as_str());

        "".to_string()
    }
}


#[cfg(test)]
#[test]
fn {{project-name}}_part1_example() {
    let day = {{project-name| capitalize}} {};
    let expected = "";

    let actual = day.part1("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn {{project-name}}_part1() {
    let day = {{project-name| capitalize}} {};
    let expected = "";

    let actual = day.part1("input".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn {{project-name}}_part2_example() {
    let day = {{project-name| capitalize}} {};
    let expected = "";

    let actual = day.part2("test1".to_string());

    assert_eq!(actual, expected);
}

#[test]
fn {{project-name}}_part2() {
    let day = {{project-name| capitalize}} {};
    let expected = "";

    let actual = day.part2("input".to_string());

    assert_eq!(actual, expected);
}