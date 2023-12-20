use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt::Debug,
    ops::Not,
    rc::Rc,
};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn process(input: &str) -> u64 {
    let (_, modules_and_receivers) = modules(input).expect("should parse");

    #[allow(clippy::type_complexity)]
    let (module_receiver_map, mut name_module_map): (
        HashMap<String, Vec<&str>>,
        HashMap<String, Rc<RefCell<Module>>>,
    ) = modules_and_receivers
        .into_iter()
        .map(|(module, receivers)| {
            (
                (module.name.clone(), receivers),
                (module.name.clone(), Rc::new(RefCell::new(module))),
            )
        })
        .unzip();

    for (module_name, receiver_names) in module_receiver_map.into_iter() {
        for receiver_name in receiver_names {
            let receiver = name_module_map.get(receiver_name);
            match receiver {
                Some(value) => name_module_map[&module_name.clone()]
                    .borrow_mut()
                    .add_receiver(value.clone()),
                None => {
                    {
                        let new_module = Rc::new(RefCell::new(Module {
                            name: module_name.clone(),
                            receivers: vec![],
                            module_type: ModuleType::Conjunction {
                                state: HashMap::new(),
                            },
                        }));
                        name_module_map.insert(String::from(receiver_name), new_module.clone());
                    }
                    name_module_map[&module_name.clone()]
                        .borrow_mut()
                        .add_receiver(name_module_map[receiver_name].clone());
                }
            };
        }
    }

    let first_module = &name_module_map["broadcaster"].clone();
    let mut destination_module_names = vec![String::from("rx")];
    while destination_module_names.len() <= 1 {
        destination_module_names = get_upstream_machine_names(&name_module_map, destination_module_names[0].clone());
    } 

    dbg!(&destination_module_names);

    let loop_counts = destination_module_names.into_iter()
        .map(|destination_machine_name| {
            for (_, module) in name_module_map.iter() {
                module.borrow_mut().reset()
            }
            get_button_presses(first_module, destination_machine_name.clone())
        })
        .collect::<Vec<_>>();
    dbg!(&loop_counts);
    loop_counts.into_iter().map(u64::from).reduce(lcm).unwrap()
    
}

fn lcm(first: u64, second: u64) -> u64 {
    first * second / gcd(first, second)
}

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut min, &mut max);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn get_upstream_machine_names(name_module_map: &HashMap<String, Rc<RefCell<Module>>>, destination_module_name: String) -> Vec<String> {
    let destination_module = name_module_map[&destination_module_name].borrow();
    let ModuleType::Conjunction {
        state: ref destination_machine_names,
    } = destination_module.module_type
    else {
        panic!("Cannot find the '{}' module.", destination_module_name);
    };
    destination_machine_names.keys().cloned().collect()
}

fn get_button_presses(
    first_module: &Rc<RefCell<Module>>,
    destination_machine_name: String,
) -> u32 {
    let mut queue = VecDeque::new();
    let mut button_presses = 1;
    'outer: loop {
        queue.push_back(Signal {
            pulse: Pulse::Low,
            receiver: first_module.clone(),
            transmitter_name: String::from("button"),
        });
        while !queue.is_empty() {
            let signal = &queue
                .pop_front()
                .expect("Queue can't be empty, as while loop would have exited");

            if signal.receiver.borrow().name == destination_machine_name
                && signal.pulse == Pulse::Low
            {
                break 'outer;
            }

            let new_signals = signal.receiver.borrow_mut().receive(signal.clone());
            for signal in new_signals {
                queue.push_back(signal);
            }
        }
        button_presses += 1;
    }
    button_presses
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
enum Pulse {
    High,
    Low,
}

impl Not for Pulse {
    type Output = Pulse;

    fn not(self) -> Self::Output {
        match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High,
        }
    }
}

#[derive(Clone)]
struct Signal {
    pulse: Pulse,
    receiver: Rc<RefCell<Module>>,
    transmitter_name: String,
}

impl Debug for Signal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Signal")
            .field("pulse", &self.pulse)
            .field("transmitter", &self.transmitter_name)
            .field("receiver", &self.receiver.borrow().name)
            .finish()
    }
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop { state: Pulse },
    Conjunction { state: HashMap<String, Pulse> },
    Broadcaster,
}

struct Module {
    name: String,
    receivers: Vec<Rc<RefCell<Module>>>,
    module_type: ModuleType,
}

impl Debug for Module {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Module")
            .field("name", &self.name)
            .field(
                "receivers",
                &self
                    .receivers
                    .iter()
                    .map(|r| r.borrow().name.clone())
                    .collect::<Vec<_>>(),
            )
            .field("module_type", &self.module_type)
            .finish()
    }
}

impl Module {
    fn receive(&mut self, signal: Signal) -> Vec<Signal> {
        let Signal {
            pulse,
            transmitter_name,
            ..
        } = signal;
        let tx = match self.module_type {
            ModuleType::FlipFlop { ref mut state } => {
                if pulse == Pulse::High {
                    return vec![];
                };
                *state = !*state;
                *state
            }
            ModuleType::Conjunction { ref mut state } => {
                state.insert(transmitter_name, pulse);
                if state.values().all(|s| *s == Pulse::High) {
                    Pulse::Low
                } else {
                    Pulse::High
                }
            }
            ModuleType::Broadcaster { .. } => pulse,
        };

        self.receivers
            .iter()
            .map(|receiver| Signal {
                pulse: tx,
                receiver: receiver.clone(),
                transmitter_name: self.name.clone(),
            })
            .collect()
    }

    fn reset(&mut self) {
        match self.module_type {
            ModuleType::FlipFlop { ref mut state } => *state = Pulse::Low,
            ModuleType::Conjunction { ref mut state } => {
                let module_names = state.keys().cloned().collect::<Vec<_>>();
                for module_name in module_names {
                    state.insert(module_name.clone(), Pulse::Low);
                }
            },
            ModuleType::Broadcaster => (),
        }
    }

    fn register_input(&mut self, receiver_name: String) {
        if let ModuleType::Conjunction { ref mut state, .. } = self.module_type {
            state.insert(receiver_name, Pulse::Low);
        }
    }

    fn add_receiver(&mut self, module: Rc<RefCell<Module>>) {
        module.borrow_mut().register_input(self.name.clone());
        self.receivers.push(module);
    }
}

fn modules(input: &str) -> IResult<&str, Vec<(Module, Vec<&str>)>> {
    let (input, modules_and_receivers) =
        separated_list1(newline, alt((flip_flop, conjunction, broadcaster)))(input)?;

    Ok((input, modules_and_receivers))
}

fn broadcaster(input: &str) -> IResult<&str, (Module, Vec<&str>)> {
    let (input, _) = tag("broadcaster -> ")(input)?;
    let (input, outputs) = separated_list1(tag(", "), alpha1)(input)?;
    Ok((
        input,
        (
            Module {
                name: String::from("broadcaster"),
                receivers: vec![],
                module_type: ModuleType::Broadcaster,
            },
            outputs,
        ),
    ))
}

fn flip_flop(input: &str) -> IResult<&str, (Module, Vec<&str>)> {
    let (input, _) = tag("%")(input)?;
    let (input, (name, outputs)) =
        separated_pair(alpha1, tag(" -> "), separated_list1(tag(", "), alpha1))(input)?;
    Ok((
        input,
        (
            Module {
                name: String::from(name),
                receivers: vec![],
                module_type: ModuleType::FlipFlop { state: Pulse::Low },
            },
            outputs,
        ),
    ))
}

fn conjunction(input: &str) -> IResult<&str, (Module, Vec<&str>)> {
    let (input, _) = tag("&")(input)?;
    let (input, (name, outputs)) =
        separated_pair(alpha1, tag(" -> "), separated_list1(tag(", "), alpha1))(input)?;
    Ok((
        input,
        (
            Module {
                name: String::from(name),
                receivers: vec![],
                module_type: ModuleType::Conjunction {
                    state: HashMap::from([]),
                },
            },
            outputs,
        ),
    ))
}
