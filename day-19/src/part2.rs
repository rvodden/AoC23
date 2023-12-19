use std::{collections::BTreeMap, ops::RangeInclusive};

use nom::{sequence::{pair, tuple, terminated, delimited}, bytes::complete::tag, IResult, character::complete::{self, alpha1, one_of}, combinator::opt, multi::separated_list1};
use nom::Parser;

pub fn process<'a>(
    input: &'a str,
) -> u64 {
    let (_, workflowlist) = terminated(workflow_list, tag("\n\n"))(input).expect("Should be given a valid input.");

    let initial_part = Part{
        x: 1..=4000,
        m: 1..=4000,
        a: 1..=4000,
        s: 1..=4000,
    };

    workflowlist.apply(initial_part).iter().inspect(|x| { println!("{:?}", x); }).map(Part::count).inspect(|x| { println!("{:?}", x); }).sum()
}


struct WorkflowList<'a>(BTreeMap<&'a str, Workflow<'a>>);

impl<'a> WorkflowList<'a> {
    fn apply(&self, part: Part) -> Vec<Part> {
        
        let workflow = self.get_workflow_by_name("in");
        
        let accepted_parts = self.apply_workflow_to_part(workflow, &part);
        accepted_parts.into_iter().map(|(p, _)| p).collect()
    }

    fn apply_workflow_to_part(&'a self, workflow: &'a Workflow, part: &Part) -> Vec<(Part, Destination<'a>)> {
        let mut accepted_parts = vec![];
        let destinations = workflow.apply(part.clone());
        for (part, destination) in destinations {
            use Destination::*;
            match destination {
                Forward(other_workflow_name) => accepted_parts.extend(
                    self.apply_workflow_to_part(self.get_workflow_by_name(other_workflow_name), &part)
                ),
                Accept => accepted_parts.push((part, destination)),
                Reject => ()
            }
        }
        accepted_parts
    }
    
    fn get_workflow_by_name(&self, name: &str) -> &Workflow {
        self.0.get(name).expect(&format!("Cannot find workflow with name {}", name))
    }
}


struct Workflow<'a>(Vec<Rule<'a>>);

impl<'a> Workflow<'a> {
    fn apply(&self, part: Part) -> Vec<(Part, Destination)> {
        let mut part = Some(part.clone());
        let mut retval = vec![];
        for rule in self.0.iter() {
            let (cont, send, destination) = rule.apply(part.unwrap());
            retval.push((send, destination));
            part = cont;            
        }
        retval
    }
}

enum Operation {
    Above(char, u32),
    Below(char, u32),
    Default
}

struct Rule<'a> {
    operation: Operation,
    destination: Destination<'a>
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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
            operation: Operation::Default,
        }
    }

    fn new (destination: Destination<'a>, operation: Operation) -> Self {
        Self {
            destination,
            operation
        }
    }

    fn apply(&self, part: Part) -> (Option<Part>, Part, Destination) {
        use Operation::*;
        match self.operation {
            Above(c, threshold) => {
                match c {
                    'x' => (Some(Part{x: *part.x.start()..=threshold, ..part.clone()}), Part{x: (threshold+1)..=*part.x.end() , ..part}, self.destination),
                    'm' => (Some(Part{m: *part.m.start()..=threshold, ..part.clone()}), Part{m: (threshold+1)..=*part.m.end() , ..part}, self.destination),
                    'a' => (Some(Part{a: *part.a.start()..=threshold, ..part.clone()}), Part{a: (threshold+1)..=*part.a.end() , ..part}, self.destination),
                    's' => (Some(Part{s: *part.s.start()..=threshold, ..part.clone()}), Part{s: (threshold+1)..=*part.s.end() , ..part}, self.destination),
                    _ => panic!("invalid")
                }
            },
            Below(c, threshold) => {
                match c {
                    'x' => (Some(Part{x: threshold..=*part.x.end() , ..part.clone()}), Part{x: *part.x.start()..=(threshold-1), ..part}, self.destination),
                    'm' => (Some(Part{m: threshold..=*part.m.end() , ..part.clone()}), Part{m: *part.m.start()..=(threshold-1), ..part}, self.destination),
                    'a' => (Some(Part{a: threshold..=*part.a.end() , ..part.clone()}), Part{a: *part.a.start()..=(threshold-1), ..part}, self.destination),
                    's' => (Some(Part{s: threshold..=*part.s.end() , ..part.clone()}), Part{s: *part.s.start()..=(threshold-1), ..part}, self.destination),
                    _ => panic!("invalid")
                }
            },
            Default => (None, part, self.destination)
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct PartList(Vec<Part>);

#[derive(Eq, PartialEq, Debug, Clone)]
struct Part {
    x: RangeInclusive<u32>,
    m: RangeInclusive<u32>,
    a: RangeInclusive<u32>,
    s: RangeInclusive<u32>
}

impl Part {
    fn count(&self) -> u64 {
        len(&self.x) * len(&self.m) * len(&self.a) * len(&self.s)
    }    
}

fn len(range: &RangeInclusive<u32>) -> u64 {
    if range.is_empty() { return 0 };
    (range.end() - range.start() + 1) as u64
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
    let operation = match operation {
        '<' => Operation::Below(parameter, threshold),
        '>' => Operation::Above(parameter, threshold),
        value => panic!("Recieved an invalid character: {}", value),
    };

    let destination = Destination::from_str(destination);

    Rule::new(
        destination,
        operation
    )
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
        assert_eq!(result, 167409079868000);
    }

    #[test]
    fn test_count() {
        let value = Part{
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000
        }.count();
        assert_eq!(value, 256000000000000)
    }

    #[test]
    fn test_count_again() {
        let value = Part{
            x: 21..=30,
            m: 21..=40,
            a: 21..=50,
            s: 21..=60
        }.count();
        assert_eq!(value, 240000)
    }
}