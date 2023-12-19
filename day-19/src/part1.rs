use std::{collections::BTreeMap, cmp::Ordering};

use nom::{sequence::{pair, tuple, terminated, delimited, preceded, separated_pair}, bytes::complete::tag, IResult, character::complete::{self, alpha1, one_of}, combinator::opt, multi::separated_list1};
use nom::Parser;

pub fn process<'a>(
    input: &'a str,
) -> u32 {
    let (_, (workflowlist, partlist)) = separated_pair(workflow_list, tag("\n\n"), part_list)(input).expect("Should be given a valid input.");

    partlist.0.iter().filter_map(|part| match workflowlist.apply(part) {
        Destination::Accept => Some(part.x + part.m + part.a + part.s),
        _ => None
    }).sum()
}


struct WorkflowList<'a>(BTreeMap<&'a str, Workflow<'a>>);

impl<'a> WorkflowList<'a> {
    fn apply(&self, part: &Part) -> &Destination {
        let mut workflow_name = "in";
        loop {
            let workflow = self.0.get(workflow_name).unwrap_or_else(||
                panic!("Cannot find workflow with name 'in'")
            );
            let destination = workflow.apply(part);
            use Destination::*;
            match destination {
                Forward(other_workflow_name) => workflow_name = other_workflow_name,
                Accept => return &Accept,
                Reject => return &Reject
            }
        }
    }
}

struct Workflow<'a>(Vec<Rule<'a>>);

impl<'a> Workflow<'a> {
    fn apply(&self, part: &Part) -> &Destination {
        for rule in self.0.iter() {
            if let Some(destination) = rule.apply(part) {
                return destination
            }
        }
        panic!("No valid destination found!");
    }
}

struct Rule<'a> {
    predicate: Box<dyn Fn(&Part) -> bool + 'a>,
    destination: Destination<'a>
}

enum Destination<'a> {
    Accept,
    Reject,
    Forward(&'a str)
}

impl<'a> Destination<'a> {
    fn from_str(string: &'a str) -> Self {
        match string {
            "A" => Destination::Accept,
            "R" => Destination::Reject,
            value => Destination::Forward(value)
        }
    }
}

impl<'a> Rule<'a> {
    fn default (destination: Destination<'a>) -> Self {
        Self {
            destination,
            predicate: Box::new(|_| true),
        }
    }

    fn new (destination: Destination<'a>, predicate: Box<dyn Fn(&Part) -> bool>) -> Self {
        Self {
            destination,
            predicate
        }
    }

    fn apply(&self, part: &Part) -> Option<&Destination> {
        if (self.predicate)(part) { Some(&self.destination) } else { None }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct PartList(Vec<Part>);

#[derive(Eq, PartialEq, Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

fn workflow_list(input: &str) -> IResult<&str, WorkflowList> {
    let (input, named_workflows) = separated_list1(tag("\n"), pair(
        alpha1,
        delimited(tag("{"), workflow, tag("}"))
    ))(input)?;
    Ok((input, WorkflowList(BTreeMap::from_iter(named_workflows.into_iter()))))
}

// a<2006:qkq,m>2090:A,rfg
fn workflow(input: &str) -> IResult<&str, Workflow> {
    separated_list1(tag(","), rule).map(|x| Workflow(x)).parse(input)
}

// a<2006:qkq
fn rule(input: &str) -> IResult<&str, Rule> {
    let (input, result) = 
        opt(terminated(
            tuple((one_of("xmas"), one_of("<>"), complete::u32)), 
            tag(":")
        )).and(alpha1).parse(input)?;
    
    match result {
        (None, destination) => Ok((input, Rule::default(Destination::from_str(destination)))),
        (Some((parameter, operation, threshold)), destination)=> Ok((input, non_default_rule(parameter, operation, threshold, destination))),
    }
}

fn non_default_rule(parameter: char, operation: char, threshold: u32, destination: &str) -> Rule{
    let comparitor : fn(u32, u32) -> bool = match operation {
        '<' => |part_param, threshold| part_param.cmp(&threshold) == Ordering::Less,
        '>' => |part_param, threshold| part_param.cmp(&threshold) == Ordering::Greater,
        value => panic!("Recieved an invalid character: {}", value),
    };

    let predicate: Box<dyn Fn(&Part) -> bool> = match parameter {
        'x' => Box::new(move |p| comparitor(p.x, threshold)),
        'm' => Box::new(move |p| comparitor(p.m, threshold)),
        'a' => Box::new(move |p| comparitor(p.a, threshold)),
        's' => Box::new(move |p| comparitor(p.s, threshold)),
        value => panic!("Received an invalid charachter: {}", value)
    };

    let destination = Destination::from_str(destination);

    Rule::new(
        destination,
        predicate
    )
}

fn part_list(input: &str) -> IResult<&str, PartList>{
    separated_list1(tag("\n"), part).map(|x| PartList(x)).parse(input)
}

//{a<2006:qkq,m>2090:A,rfg}
fn part(input: &str) -> IResult<&str, Part> {
    let (input, parameter) = delimited(
        tag("{"),
        separated_list1(
            tag(","),
            preceded(
                pair(one_of("xmas"), tag("=")),
                complete::u32
            )
        ),
        tag("}")
    )(input)?;
    Ok((input, Part{
        x: parameter[0],
        m: parameter[1],
        a: parameter[2],
        s: parameter[3]
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_workflow_list() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}";
        let (input, _) = workflow_list(input).unwrap();
        assert_eq!(input, "");
    }

    #[test]
    fn test_part_list() {
        let input = "{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        let (input, partlist) = part_list(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(partlist, 
            PartList(vec![
            Part{x:787, m:2655, a:1222, s:2876},
            Part{x:1679, m:44, a:2067, s:496},
            Part{x:2036, m:264, a:79, s:2244},
            Part{x:2461, m:1339, a:466, s:291},
            Part{x:2127, m:1623, a:2188, s:1013},
        ]))
    }

    #[test]
    fn test_process() {
        let lines = "px{a<2006:qkq,m>2090:A,rfg}
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
        let result = process(lines);
        assert_eq!(result, 19114);
    }
}