use indicatif::ProgressIterator;
use itertools::Itertools;
use lazy_static::*;
use regex::Regex;
use std::{collections::HashMap, ops::Range};

lazy_static! {
    static ref PART_REGEX: Regex = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    static ref DOUBLE_LINE_REGEX: Regex = Regex::new(r"\r?\n\r?\n").unwrap();
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn parse(s: &str) -> Part {
        let captures = PART_REGEX.captures(s).unwrap();
        Part {
            x: captures.get(1).unwrap().as_str().parse().unwrap(),
            m: captures.get(2).unwrap().as_str().parse().unwrap(),
            a: captures.get(3).unwrap().as_str().parse().unwrap(),
            s: captures.get(4).unwrap().as_str().parse().unwrap(),
        }
    }

    fn value(&self, c: char) -> usize {
        match c {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            other => unreachable!("Should never ask for a part value `{other}`"),
        }
    }
}

struct PartRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    fn parse(input: &str) -> Workflow {
        let (name, rules_str) = input.trim_end_matches('}').split_once('{').unwrap();
        let rules: Vec<Rule> = rules_str.split(',').map(Rule::parse).collect();

        Workflow {
            name: name.to_string(),
            rules,
        }
    }

    fn test(&self, part: &Part, workflows: &HashMap<String, Workflow>) -> bool {
        self.rules
            .iter()
            .find_map(|rule| rule.test(part, workflows))
            .expect("Expected a workflow to yield a definite result")
    }

    fn test_range(
        &self,
        range: &PartRange,
        workflows: &HashMap<String, Workflow>,
    ) -> Vec<Range<usize>> {
        vec![]
    }
}

#[derive(Debug)]
enum Rule {
    ConditionLess(char, usize, Box<Rule>),
    ConditionMore(char, usize, Box<Rule>),
    WorkflowRef(String),
    Result(bool),
}

impl Rule {
    fn parse(input: &str) -> Rule {
        use Rule::*;

        match input {
            s if s.contains('<') => {
                let (char, rest) = s.split_once('<').unwrap();
                let (threshold, result) = rest.split_once(':').unwrap();
                ConditionLess(
                    char.chars().next().unwrap(),
                    threshold.parse().unwrap(),
                    Box::new(Rule::parse(result)),
                )
            }
            s if s.contains('>') => {
                let (char, rest) = s.split_once('>').unwrap();
                let (threshold, result) = rest.split_once(':').unwrap();
                ConditionMore(
                    char.chars().next().unwrap(),
                    threshold.parse().unwrap(),
                    Box::new(Rule::parse(result)),
                )
            }
            "R" => Result(false),
            "A" => Result(true),
            wf => WorkflowRef(wf.to_string()),
        }
    }

    fn test(&self, part: &Part, workflows: &HashMap<String, Workflow>) -> Option<bool> {
        match self {
            Rule::ConditionLess(char, threshold, if_passing) => {
                if part.value(*char) < *threshold {
                    if_passing.test(part, workflows)
                } else {
                    None
                }
            }
            Rule::ConditionMore(char, threshold, if_passing) => {
                if part.value(*char) > *threshold {
                    if_passing.test(part, workflows)
                } else {
                    None
                }
            }
            Rule::WorkflowRef(wf) => return Some(workflows[wf.as_str()].test(part, workflows)),
            Rule::Result(result) => return Some(*result),
        }
    }

    fn test_range(
        &self,
        range: &PartRange,
        workflows: &HashMap<String, Workflow>,
    ) -> Vec<PartRange> {
        match self {
            Rule::ConditionLess(char, threshold, if_passing) => {
                if part.value(*char) < *threshold {
                    if_passing.test(part, workflows)
                } else {
                    None
                }
            }
            Rule::ConditionMore(char, threshold, if_passing) => {
                if part.value(*char) > *threshold {
                    if_passing.test(part, workflows)
                } else {
                    None
                }
            }
            Rule::WorkflowRef(wf) => return Some(workflows[wf.as_str()].test(part, workflows)),
            Rule::Result(result) if result => return Some(vec![range]),
            Rule::Result(_) => return Some(vec![]),
        }
    }
}

fn parse(input: &str) -> (Vec<Workflow>, Vec<Part>) {
    let split = DOUBLE_LINE_REGEX.split(input).collect_vec();

    let workflows: Vec<Workflow> = split.get(0).unwrap().lines().map(Workflow::parse).collect();

    let parts: Vec<Part> = split.get(1).unwrap().lines().map(Part::parse).collect();

    return (workflows, parts);
}

pub fn part1(input: &str) -> usize {
    let (workflows, parts) = parse(input);
    let workflow_map: HashMap<String, Workflow> = workflows
        .into_iter()
        .map(|wf| (wf.name.to_string(), wf))
        .collect();
    let start_wf = &workflow_map["in"];

    println!("Workflows: {:?}", workflow_map.values());
    println!("Parts: {parts:?}");

    let passing_parts = parts
        .iter()
        .filter(|p| start_wf.test(*p, &workflow_map))
        .collect_vec();

    println!("Passing Parts: {passing_parts:?}");

    let result = passing_parts.iter().map(|p| p.x + p.m + p.a + p.s).sum();
    return result;
}

pub fn part2(input: &str) -> usize {
    let (workflows, _) = parse(input);
    let workflow_map: HashMap<String, Workflow> = workflows
        .into_iter()
        .map(|wf| (wf.name.to_string(), wf))
        .collect();

    let start_wf = &workflow_map["in"];
    let start_range = PartRange {
        x: 0..=4000,
        m: 0..=4000,
        a: 0..=4000,
        s: 0..=4000,
    };

    let results = start_wf.test_range(&range, &workflows);
    dbg!(results);

    return 0;
}

pub fn process(input: String) {
    use std::time::Instant;
    let now = Instant::now();
    let result = part2(&input);
    println!("Result: {result}");
    println!("Finished in: {:.2?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";

    // #[test]
    fn part1_example() {
        let result = part1(EXAMPLE);
        assert_eq!(result, 19114);
    }

    // #[test]
    fn part1_input() {
        let input = include_str!("input.txt");
        let result = part1(input);
        assert_eq!(result, 362930);
    }

    #[test]
    fn part2_example() {
        let result = part2(EXAMPLE);
        assert_eq!(result, 167409079868000);
    }

    // #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
