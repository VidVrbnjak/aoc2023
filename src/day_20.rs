use std::collections::{HashMap, HashSet, VecDeque};

pub fn p1(input: &str) {
    let mut modules = parse_modules(input);
    let mut low_pulses = 0;
    let mut high_pulses = 0;

    for _ in 0..1_000 {
        let mut q: VecDeque<(String, String, Pulse)> = match modules.get("button").unwrap() {
            Module::Button { targets } => targets
                .iter()
                .map(|target| ("button".to_string(), target.to_string(), Pulse::Low))
                .collect::<VecDeque<_>>(),
            _ => unreachable!(),
        };
        while let Some((sender, target, in_pulse)) = q.pop_front() {
            match in_pulse {
                Pulse::Low => low_pulses += 1,
                Pulse::High => high_pulses += 1,
            }
            let module = modules.get_mut(&target);
            if module.is_none() {
                continue;
            }
            let module = module.unwrap();
            if let Some(out_pulse) = pulse_module(sender, in_pulse, module) {
                match module {
                    Module::Undefined => {}
                    Module::FlipFlop { on, targets } => {
                        targets
                            .iter()
                            .for_each(|t| q.push_back((target.clone(), t.clone(), out_pulse)));
                    }
                    Module::Conjunction { inputs, targets } => {
                        targets
                            .iter()
                            .for_each(|t| q.push_back((target.clone(), t.clone(), out_pulse)));
                    }
                    Module::Broadcast { targets } => {
                        targets
                            .iter()
                            .for_each(|t| q.push_back((target.clone(), t.clone(), out_pulse)));
                    }
                    Module::Button { targets } => {
                        targets
                            .iter()
                            .for_each(|t| q.push_back((target.clone(), t.clone(), out_pulse)));
                    }
                }
            }
        }
    }

    println!(
        "LOW x HIGH = {} x {} = {}",
        low_pulses,
        high_pulses,
        low_pulses * high_pulses
    );
}
pub fn p2(input: &str) {
    let mut modules = parse_modules(input);
    let mut button_presses = 0;

    'ml: loop {
        let mut q: VecDeque<(String, String, Pulse)> = match modules.get("button").unwrap() {
            Module::Button { targets } => targets
                .iter()
                .map(|target| ("button".to_string(), target.to_string(), Pulse::Low))
                .collect::<VecDeque<_>>(),
            _ => unreachable!(),
        };
        while let Some((sender, target, in_pulse)) = q.pop_front() {
            if sender.as_str() == "button" {
                button_presses += 1;
                if button_presses % 100_000 == 0 {
                    dbg!(button_presses);
                }
            }

            if target.as_str() == "rx" {
                if in_pulse == Pulse::Low {
                    break 'ml;
                }
            }

            let module = modules.get_mut(&target);
            if module.is_none() {
                continue;
            }
            let module = module.unwrap();
            if let Some(out_pulse) = pulse_module(sender, in_pulse, module) {
                match module {
                    Module::Undefined => {}
                    Module::FlipFlop { on, targets } => {
                        targets
                            .iter()
                            .for_each(|t| q.push_back((target.clone(), t.clone(), out_pulse)));
                    }
                    Module::Conjunction { inputs, targets } => {
                        targets
                            .iter()
                            .for_each(|t| q.push_back((target.clone(), t.clone(), out_pulse)));
                    }
                    Module::Broadcast { targets } => {
                        targets
                            .iter()
                            .for_each(|t| q.push_back((target.clone(), t.clone(), out_pulse)));
                    }
                    Module::Button { targets } => {
                        targets
                            .iter()
                            .for_each(|t| q.push_back((target.clone(), t.clone(), out_pulse)));
                    }
                }
            }
        }
    }

    println!("Min button presses required to get a low pulse on rx in {button_presses}");
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Pulse {
    Low,
    High,
}

#[derive(Clone, Debug)]
enum Module {
    Undefined,
    FlipFlop {
        on: bool,
        targets: Vec<String>,
    },
    Conjunction {
        inputs: HashMap<String, Pulse>,
        targets: Vec<String>,
    },
    Broadcast {
        targets: Vec<String>,
    },
    Button {
        targets: Vec<String>,
    },
}

fn parse_modules(input: &str) -> HashMap<String, Module> {
    let mut modules = HashMap::new();
    modules.insert(
        "button".to_string(),
        Module::Button {
            targets: vec!["broadcaster".to_string()],
        },
    );

    for line in input.lines().take_while(|l| !l.is_empty()) {
        let targets = line
            .split("->")
            .skip(1)
            .next()
            .map(|s| {
                s.split(",")
                    .map(|s| s.trim())
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let module = match line.chars().next().unwrap() {
            '%' => Module::FlipFlop { on: false, targets },
            '&' => Module::Conjunction {
                inputs: HashMap::new(),
                targets,
            },
            _ => {
                if line.starts_with("broadcaster") {
                    Module::Broadcast { targets }
                } else {
                    Module::Undefined
                }
            }
        };

        let module_name = line
            .split(" ")
            .next()
            .unwrap()
            .trim_start_matches(&['%', '&'])
            .trim()
            .to_string();

        modules.insert(module_name, module);
    }

    for (name, module) in modules.clone().iter() {
        match module {
            Module::Undefined => {}
            Module::FlipFlop { on, targets } => {
                for target in targets.iter() {
                    match modules.get_mut(target).unwrap() {
                        Module::Conjunction { inputs, targets } => {
                            inputs.insert(name.clone(), Pulse::Low);
                        }
                        _ => {}
                    }
                }
            }
            Module::Conjunction { inputs, targets } => {
                for target in targets.iter() {
                    match modules.get_mut(target) {
                        Some(Module::Conjunction { inputs, targets }) => {
                            inputs.insert(name.clone(), Pulse::Low);
                        }
                        _ => {}
                    }
                }
            }
            Module::Broadcast { targets } => {
                for target in targets.iter() {
                    match modules.get_mut(target).unwrap() {
                        Module::Conjunction { inputs, targets } => {
                            inputs.insert(name.clone(), Pulse::Low);
                        }
                        _ => {}
                    }
                }
            }
            Module::Button { targets } => {
                for target in targets.iter() {
                    match modules.get_mut(target).unwrap() {
                        Module::Conjunction { inputs, targets } => {
                            inputs.insert(name.clone(), Pulse::Low);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    modules
}

fn pulse_module(sender: String, pulse: Pulse, module: &mut Module) -> Option<Pulse> {
    match module {
        Module::Undefined => None,
        Module::FlipFlop { on, targets } => {
            if pulse == Pulse::Low {
                *on = !*on;
                if *on {
                    Some(Pulse::High)
                } else {
                    Some(Pulse::Low)
                }
            } else {
                None
            }
        }
        Module::Conjunction { inputs, targets } => {
            *inputs.get_mut(&sender).unwrap() = pulse;
            if inputs.values().all(|p| *p == Pulse::High) {
                Some(Pulse::Low)
            } else {
                Some(Pulse::High)
            }
        }
        Module::Broadcast { targets } => Some(pulse),
        Module::Button { targets } => Some(Pulse::Low),
    }
}
