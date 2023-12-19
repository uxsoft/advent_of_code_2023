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

#[derive(Debug, Clone)]
struct PartRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl PartRange {
    fn value(&self, c: char) -> Range<usize> {
        match c {
            'x' => self.x.clone(),
            'm' => self.m.clone(),
            'a' => self.a.clone(),
            's' => self.s.clone(),
            other => unreachable!("Should never ask for a part value `{other}`"),
        }
    }

    fn with(&self, c: char, value: Range<usize>) -> PartRange {
        match c {
            'x' => PartRange {
                x: value,
                m: self.m.clone(),
                a: self.a.clone(),
                s: self.s.clone(),
            },
            'm' => PartRange {
                x: self.x.clone(),
                m: value,
                a: self.a.clone(),
                s: self.s.clone(),
            },
            'a' => PartRange {
                x: self.x.clone(),
                m: self.m.clone(),
                a: value,
                s: self.s.clone(),
            },
            's' => PartRange {
                x: self.x.clone(),
                m: self.m.clone(),
                a: self.a.clone(),
                s: value,
            },
            other => unreachable!("Should never ask for a part value `{other}`"),
        }
    }

    /// Splits the range into start..treshold (exclusive) and threshold..end (exclusive)
    fn split(&self, c: char, treshold: usize) -> (PartRange, PartRange) {
        if self.value(c).start >= treshold {
            (self.with(c, 0..0), self.clone())
        } else if self.value(c).end <= treshold {
            (self.clone(), self.with(c, 0..0))
        } else {
            (
                self.with(c, self.value(c).start..treshold),
                self.with(c, treshold..self.value(c).end),
            )
        }
    }
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
        range: PartRange,
        workflows: &HashMap<String, Workflow>,
    ) -> Vec<PartRange> {
        let mut passing: Vec<PartRange> = Vec::new();
        let mut next: Vec<PartRange> = vec![range];

        for rule in &self.rules {
            let mut new_next: Vec<PartRange> = vec![];

            for r in next {
                let test = rule.test_range(r, workflows);

                passing.extend(test.passing);
                new_next.extend(test.next);
            }

            next = new_next;
        }

        return passing;
    }
}

#[derive(Debug)]
enum Rule {
    ConditionLess(char, usize, Box<Rule>),
    ConditionMore(char, usize, Box<Rule>),
    WorkflowRef(String),
    Result(bool),
}

struct RangeTest {
    next: Vec<PartRange>,
    passing: Vec<PartRange>,
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

    fn test_range(&self, range: PartRange, workflows: &HashMap<String, Workflow>) -> RangeTest {
        match self {
            Rule::ConditionLess(char, threshold, if_passing) => {
                let (l, m) = range.split(*char, *threshold);

                let if_passing_test = if_passing.test_range(l, workflows);

                let mut next = Vec::new();
                next.push(m);
                for i in if_passing_test.next {
                    next.push(i);
                }

                RangeTest {
                    passing: if_passing_test.passing,
                    next,
                }
            }
            Rule::ConditionMore(char, threshold, if_passing) => {
                let (l, m) = range.split(*char, *threshold + 1);

                let if_passing_test = if_passing.test_range(m, workflows);

                let mut next = Vec::new();
                next.push(l);
                for i in if_passing_test.next {
                    next.push(i);
                }

                RangeTest {
                    passing: if_passing_test.passing,
                    next,
                }
            }
            Rule::WorkflowRef(wf) => {
                let wf_result = workflows[wf.as_str()].test_range(range, workflows);
                RangeTest {
                    next: vec![],
                    passing: wf_result,
                }
            }
            Rule::Result(true) => {
                return RangeTest {
                    next: vec![],
                    passing: vec![range],
                }
            }
            Rule::Result(false) => {
                return RangeTest {
                    next: vec![],
                    passing: vec![],
                }
            }
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
        x: 1..4001,
        m: 1..4001,
        a: 1..4001,
        s: 1..4001,
    };

    let results = start_wf.test_range(start_range, &workflow_map);

    let result: usize = results
        .iter()
        .map(|r| {
            (r.x.end - r.x.start)
                * (r.m.end - r.m.start)
                * (r.a.end - r.a.start)
                * (r.s.end - r.s.start)
        })
        .sum();

    return result;
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

    #[test]
    fn part2_input() {
        let input = include_str!("input.txt");
        let result = part2(input);
        assert_eq!(result, 116365820987729);
    }
}
