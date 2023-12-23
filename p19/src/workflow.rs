use std::collections::HashMap;

use crate::{part::Part, label::Label, part_possibility::PartPossibility};

#[derive(Debug, Clone)]
pub enum WorkflowAction {
    Accept,
    Reject,
    Goto(String)
}

impl WorkflowAction {
    fn from_str(s: &str) -> WorkflowAction {
        match s {
            "A" => WorkflowAction::Accept,
            "R" => WorkflowAction::Reject,
            _ => WorkflowAction::Goto(s.to_string())
        }
    }
}

#[derive(Debug, Clone)]
pub enum WorkflowCondition {
    LessThan(Label, u32),
    GreaterThan(Label, u32)
}

impl WorkflowCondition {
    fn from_str(s: &str) -> WorkflowCondition {
        let mut parts = s.split(&['<', '>']);
        let name = Label::from_char(parts.next().unwrap().chars().next().unwrap());
        let num: u32 = parts.next().unwrap().parse().unwrap();
        if s.contains("<") {
            WorkflowCondition::LessThan(name, num)
        }
        else {
            WorkflowCondition::GreaterThan(name, num)
        }
    }

    fn check(&self, part: &Part) -> bool {
        match self {
            WorkflowCondition::LessThan(c, n) => part.get(c) < *n,
            WorkflowCondition::GreaterThan(c, n) => part.get(c) > *n,
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorkflowItem {
    action: WorkflowAction,
    condition: WorkflowCondition
}

impl WorkflowItem {
    fn from_str(s: &str) -> WorkflowItem {
        let mut parts = s.split(':');
        let condition = WorkflowCondition::from_str(parts.next().unwrap());
        let action = WorkflowAction::from_str(parts.next().unwrap());
        Self { condition, action }
    }

    fn check(&self, part: &Part) -> Option<&WorkflowAction> {
        if self.condition.check(part) {
            Some(&self.action)
        }
        else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Workflow {
    items: Vec<WorkflowItem>,
    or_else: WorkflowAction
}

impl Workflow {
    fn from_str(s: &str) -> Workflow {
        let parts: Vec<_> = s.split(',').collect();
        let items = parts.iter().take(parts.len() - 1).map(|s| WorkflowItem::from_str(s)).collect();
        let or_else = WorkflowAction::from_str(parts.last().unwrap());
        Self { items, or_else }
    }

    fn check(&self, part: &Part) -> &WorkflowAction {
        self.items.iter().find_map(|item| item.check(part)).unwrap_or(&self.or_else)
    }

    fn split(&self, possibility: PartPossibility, states: &mut Vec<(String, PartPossibility)>) -> usize {
        let mut rest = possibility;
        let mut sum = 0;
        self.items.iter().for_each(|item| {
            let matched = match item.condition {
                WorkflowCondition::LessThan(label, part) => rest.separate_lt(label, part),
                WorkflowCondition::GreaterThan(label, part) => rest.separate_gt(label, part),
            };
            match &item.action {
                WorkflowAction::Accept => sum += matched.size(),
                WorkflowAction::Reject => (),
                WorkflowAction::Goto(label) => if matched.size() > 0 { states.push((label.clone(), matched)) },
            }
        });
        match &self.or_else {
            WorkflowAction::Accept => sum += rest.size(),
            WorkflowAction::Reject => (),
            WorkflowAction::Goto(label) => if rest.size() > 0 { states.push((label.clone(), rest)) },
        }
        sum
    }
}

#[derive(Debug)]
pub struct WorkflowNet {
    workflows: HashMap<String, Workflow>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkflowResult {
    Accept,
    Reject
}

impl WorkflowNet {
    pub fn from_lines(lines: &mut impl Iterator<Item = String>) -> Self {
        let mut workflows = HashMap::new();
        while let Some(line) = lines.next() {
            if line == "" { break }

            let mut parts = line.split('{');
            let name = parts.next().unwrap().to_string();
            let workflow_str = parts.next().unwrap();
            let workflow = Workflow::from_str(&workflow_str[0..workflow_str.len()-1]);
            
            workflows.insert(name, workflow);
        }
        Self { workflows }
    }

    fn check(&self, part: &Part) -> WorkflowResult {
        let mut at = "in";
        loop {
            match self.workflows[at].check(part) {
                WorkflowAction::Accept => break WorkflowResult::Accept,
                WorkflowAction::Reject => break WorkflowResult::Reject,
                WorkflowAction::Goto(next) => at = next,
            }
        }
    }

    pub fn sum_if_accepted(&self, part: &Part) -> u32 {
        match self.check(part) {
            WorkflowResult::Accept => part.sum(),
            WorkflowResult::Reject => 0,
        }
    }

    pub fn sum_all(&self) -> usize {
        let mut states = vec![("in".to_string(), PartPossibility::full(1, 4000))];
        let mut sum = 0;
        while let Some((at, possibility)) = states.pop() {
            sum += self.workflows[&at].split(possibility, &mut states);
        }
        sum
    }
}