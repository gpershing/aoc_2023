use std::{collections::{HashMap, VecDeque, HashSet}, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pulse {
    Lo, Hi
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MovingPulse {
    pulse: Pulse,
    from: String,
    to: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleState {
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>)
}

impl ModuleState {
    fn pulse<F: FnMut(Pulse)>(&mut self, from: &str, pulse: Pulse, mut send: F) {
        match self {
            ModuleState::FlipFlop(on) => {
                match pulse {
                    Pulse::Lo => {
                        *on = !*on;
                        send(if *on { Pulse::Hi } else { Pulse::Lo });
                    },
                    Pulse::Hi => (),
                }
            },
            ModuleState::Conjunction(memory) => {
                *memory.get_mut(from).unwrap() = pulse;
                send(if memory.values().all(|p| *p == Pulse::Hi) { Pulse::Lo } else { Pulse::Hi });
            },
        }
    }

    fn pulse_printzg<F: FnMut(Pulse)>(&mut self, from: &str, pulse: Pulse, i: usize, to_print: bool, mut send: F) {
        match self {
            ModuleState::FlipFlop(on) => {
                match pulse {
                    Pulse::Lo => {
                        *on = !*on;
                        send(if *on { Pulse::Hi } else { Pulse::Lo });
                    },
                    Pulse::Hi => (),
                }
            },
            ModuleState::Conjunction(memory) => {
                let entry = memory.get_mut(from).unwrap();
                let should_print = *entry != pulse && to_print;
                *entry = pulse;
                if should_print && memory.values().any(|v| *v == Pulse::Hi) {
                    println!("{:?} {:?} {:?} {:?} {i}", memory["jd"], memory["fv"], memory["lm"], memory["vm"]);
                }
                send(if memory.values().all(|p| *p == Pulse::Hi) { Pulse::Lo } else { Pulse::Hi });
            },
        }
    }

    fn worth_printing(&self) -> bool {
        match self {
            ModuleState::FlipFlop(_) => false,
            ModuleState::Conjunction(memory) => memory.values().any(|v| *v == Pulse::Hi),
        }
    }
}

#[derive(Debug)]
pub struct ModuleNet {
    broadcast: Vec<String>,
    state: HashMap<String, ModuleState>,
    module_outputs: HashMap<String, Vec<String>>
}

impl ModuleNet {
    pub fn from_lines(lines: impl Iterator<Item=String>) -> Self {
        let mut broadcast = Vec::new();
        let mut module_outputs = HashMap::new();
        let mut state = HashMap::new();
        lines.for_each(|line| {
            let mut parts = line.split("->");
            let name = parts.next().unwrap().trim();
            let outputs = parts.next().unwrap().trim().split(", ").map(|s| s.to_string()).collect();
            if name == "broadcaster" {
                broadcast = outputs;
            }
            else {
                module_outputs.insert(name[1..].to_string(), outputs);
                let t = name.chars().next().unwrap();
                state.insert(name[1..].to_string(), match t {
                    '%' => ModuleState::FlipFlop(false),
                    '&' => ModuleState::Conjunction(HashMap::new()),
                    _ => panic!()
                });
            }
        });
        module_outputs.iter().for_each(|(key, outputs)| {
            outputs.iter().for_each(|to: &String| match state.get_mut(to) {
                Some(ModuleState::Conjunction(map)) => { map.insert(key.clone(), Pulse::Lo); },
                Some(ModuleState::FlipFlop(_)) => (),
                None => (),
            })
        });

        Self { broadcast, module_outputs, state }
    }

    pub fn current_state(&self) -> &HashMap<String, ModuleState> { &self.state }

    pub fn pulse_broadcast(&mut self) -> (u32, u32) {
        let mut pulses: VecDeque<_> = self.broadcast.iter().map(|to| MovingPulse { pulse: Pulse::Lo, from: "broadcaster".to_string(), to: to.clone() }).collect();
        let mut sent_lo = 1;
        let mut sent_hi = 0;
        while let Some(current) = pulses.pop_front() {
            match current.pulse {
                Pulse::Lo => sent_lo += 1,
                Pulse::Hi => sent_hi += 1,
            }
            if let Some(module) = self.state.get_mut(&current.to) { 
                module.pulse(&current.from, current.pulse, |pulse| {
                    self.module_outputs[&current.to].iter().for_each(|out| {
                        pulses.push_back(MovingPulse { pulse, from: current.to.clone(), to: out.clone() })
                    })
                })
            }
        }
        (sent_lo, sent_hi)
    }

    pub fn pulse_broadcast_print(&mut self, i: usize) {
        let mut pulses: VecDeque<_> = self.broadcast.iter().map(|to| MovingPulse { pulse: Pulse::Lo, from: "broadcaster".to_string(), to: to.clone() }).collect();
        while let Some(current) = pulses.pop_front() {
            if let Some(module) = self.state.get_mut(&current.to) { 
                module.pulse_printzg(&current.from, current.pulse, i, current.to == "zg", |pulse| {
                    self.module_outputs[&current.to].iter().for_each(|out| {
                        pulses.push_back(MovingPulse { pulse, from: current.to.clone(), to: out.clone() })
                    })
                });
            }
        }
    }
}