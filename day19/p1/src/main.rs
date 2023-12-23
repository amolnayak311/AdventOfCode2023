use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct Condition {
    attribute: char,
    is_less_than: bool,
    threshold: u32
}
#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

impl Part {
    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

impl Condition {

    fn eval(&self, part: &Part) -> bool {
        let lhs = match self.attribute  {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _  => panic!("Unexpected attribute")
        };
        if self.is_less_than {
            lhs < self.threshold
        } else {
            lhs > self.threshold
        }
    }
}
#[derive(Debug)]
struct WorkflowStep {
    condition: Condition,
    workflow_if_successful: String
}

impl WorkflowStep {

    fn apply(&self, part: &Part) -> Option<&str> {
        if self.condition.eval(part) {
            Some(&self.workflow_if_successful)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Workflow {
    steps: Vec<WorkflowStep>,
    default_workflow: String
}

impl <'a> Workflow {

    fn eval_rec(&'a self, part: &'a Part, steps: &'a [WorkflowStep]) -> &'a str {
        if steps.is_empty() {
            &self.default_workflow
        } else {
            let next_workflow = steps.first().unwrap();
            match next_workflow.apply(part) {
                Some(next_workflow)  => next_workflow,
                None                       => self.eval_rec(part, &steps[1..])
            }
        }
    }
    fn eval_workflow(&'a self, part: &'a Part) -> &'a str {
        self.eval_rec(part, &self.steps)
    }
}

fn eval_workflows_rec(part: &Part, current_workflow: &str, workflow_map: &HashMap<String, Workflow>) -> String {

    let wf = workflow_map.get(current_workflow).unwrap();
    let next_wf = wf.eval_workflow(part);
    if next_wf == "R" || next_wf == "A" {
        String::from(next_wf)
    } else {
        eval_workflows_rec(part, next_wf, workflow_map)
    }
}

fn main() {
    let (_, workflows, parts) = read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .fold((false, HashMap::new(), vec![]),
        |(parse_part, mut workflows, mut parts), line| {

            if line.is_empty() {
                (true, workflows, parts)
            } else if parse_part {
                // example,  {x=787,m=2655,a=1222,s=2876}
                let part = line[1..line.len() - 1]
                    .split(",")
                    .fold((0, 0, 0, 0), |(x, m, a, s), value| {
                        match value.chars().next().unwrap() {
                            'x' => (value[2..].parse::<u32>().unwrap(), m, a, s),
                            'm' => (x, value[2..].parse::<u32>().unwrap(), a, s),
                            'a' => (x, m, value[2..].parse::<u32>().unwrap(), s),
                            's' => (x, m, a, value[2..].parse::<u32>().unwrap()),
                            _  => panic!("Unexpected attribute in part")
                        }
                    });
                parts.push(Part{x: part.0, m: part.1, a: part.2, s: part.3});
                (parse_part, workflows, parts)
            } else {
                // Example px{a<2006:qkq,m>2090:A,rfg}
                let workflow_name = &line[0..line.find("{").unwrap()];
                let conditions = &line[workflow_name.len() + 1..line.len() - 1];
                // Condition will look like  a<2006:qkq,m>2090:A,rfg, now split by ,
                let (steps, default_workflow) = conditions
                    .split(",")
                    .fold((vec![], String::new()),
                      |(mut workflow_steps, default_workflow), condition_string| {
                          if condition_string.contains(":") {
                              let colon_idx = condition_string.find(":").unwrap();
                              let workflow_if_successful = String::from(&condition_string[colon_idx + 1..]);
                              let condition = &condition_string[0..colon_idx];
                              let mut iter = condition.chars();
                              let attribute = iter.next().unwrap();
                              let is_less_than = iter.next().unwrap() == '<';
                              let threshold = condition[2..].parse::<u32>().unwrap();
                              let condition = Condition{ attribute, is_less_than, threshold};
                              workflow_steps.push(WorkflowStep{condition, workflow_if_successful});
                              (workflow_steps, default_workflow)
                          } else {
                              (workflow_steps, String::from(condition_string))
                          }
                      });
                workflows.insert(String::from(workflow_name), Workflow{steps, default_workflow});
                (parse_part, workflows, parts)

            }
        });

    let accepted_parts_sum =
        parts.into_iter().filter(|part| eval_workflows_rec(part, "in", &workflows) == "A")
        .fold(0, |acc, part| acc + part.sum());
    println!("{}", accepted_parts_sum);

}
