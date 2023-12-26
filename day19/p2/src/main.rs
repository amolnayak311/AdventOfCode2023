use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct Condition {
    attribute: char,
    is_less_than: bool,
    threshold: u64
}
#[derive(Debug, Copy, Clone)]
struct Part {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64)
}


impl <'a> Part {
    fn new() -> &'a Part {
        &Part{x: (1, 4001), m: (1, 4001), a: (1, 4001), s: (1, 4001)}
    }

    fn combinations(&self) -> u64 {
        (self.x.1 - self.x.0) * (self.m.1 - self.m.0)  * (self.a.1 - self.a.0)  * (self.s.1 - self.s.0)
    }
}

impl Condition {

    // First one matches the condition, second one that doesnt match the condition
    fn decompose(&self, part: &Part) -> (Option<Part>, Option<Part>) {
        if self.is_less_than {
            match self.attribute {
                'x' =>  (if part.x.0 < self.threshold {
                    Some(Part{x: (part.x.0, self.threshold), m: part.m, a: part.a, s: part.s})
                } else {
                    None
                },
                         if part.x.1 >= self.threshold {
                             Some(Part{x: (self.threshold, part.x.1), m: part.m, a: part.a, s: part.s})
                         } else {
                             None
                         }
                ),
                'm' =>  (if part.m.0 < self.threshold {
                    Some(Part{x: part.x, m: (part.m.0, self.threshold), a: part.a, s: part.s})
                } else {
                    None
                },
                         if part.m.1 >= self.threshold {
                             Some(Part{x: part.x, m: (self.threshold, part.m.1), a: part.a, s: part.s})
                         } else {
                             None
                         }
                ),
                'a' =>  (if part.a.0 < self.threshold {
                    Some(Part{x: part.x, m: part.m, a: (part.a.0, self.threshold), s: part.s})
                } else {
                    None
                },
                         if part.a.1 >= self.threshold {
                             Some(Part{x: part.x, m: part.m, a: (self.threshold, part.a.1),  s: part.s})
                         } else {
                             None
                         }
                ),
                's' =>  (if part.s.0 < self.threshold {
                    Some(Part{x: part.x, m: part.m, a: part.a, s: (part.s.0, self.threshold)})
                } else {
                    None
                },
                         if part.s.1 >= self.threshold {
                             Some(Part{x: part.x, m: part.m, a: part.a, s: (self.threshold, part.s.1)})
                         } else {
                             None
                         }
                ),
                _  => panic!("Unexpected attribute")
            }
        } else {
            match self.attribute {
                'x' =>  (
                    if part.x.1 > self.threshold {
                        Some(Part{x: (self.threshold + 1, part.x.1), m: part.m, a: part.a, s: part.s})
                    } else {
                        None
                    },
                    if part.x.0 <= self.threshold {
                        Some(Part{x: (part.x.0, self.threshold + 1), m: part.m, a: part.a, s: part.s})
                    } else {
                        None
                    }
                ),
                'm' =>  (
                    if part.m.1 > self.threshold {
                        Some(Part{x: part.x, m: (self.threshold + 1, part.m.1), a: part.a, s: part.s})
                    } else {
                        None
                    },
                    if part.m.0 <= self.threshold {
                        Some(Part{x: part.x, m: (part.m.0, self.threshold + 1), a: part.a, s: part.s})
                    } else {
                        None
                    }
                ),
                'a' =>  ( if part.a.1 > self.threshold {
                    Some(Part{x: part.x, m: part.m, a: (self.threshold + 1, part.a.1),  s: part.s})
                } else {
                    None
                },
                          if part.a.0 <= self.threshold {
                              Some(Part{x: part.x, m: part.m, a: (part.a.0, self.threshold + 1), s: part.s})
                          } else {
                              None
                          },

                ),
                's' =>  (if part.s.1 > self.threshold {
                    Some(Part{x: part.x, m: part.m, a: part.a, s: (self.threshold + 1, part.s.1)})
                } else {
                    None
                },
                         if part.s.0 <= self.threshold {
                             Some(Part{x: part.x, m: part.m, a: part.a, s: (part.s.0, self.threshold + 1)})
                         } else {
                             None
                         },

                ),
                _ => panic!("Unexpected attribute")
            }
        }

    }
}
#[derive(Debug)]
struct WorkflowStep {
    condition: Condition,
    workflow_if_successful: String
}


#[derive(Debug)]
struct Workflow {
    steps: Vec<WorkflowStep>,
    default_workflow: String
}

fn dfs(part: &Part, wf: &str, wf_map: &HashMap<String, Workflow>) -> u64 {
    if wf == "A" {
        part.combinations()
    } else if wf != "R" {
        // get the current workflow and decompose the steps for each condition
        let wf = wf_map.get(wf).unwrap();
        let (sum, remaining_parts) = wf.steps.iter().fold((0, Some(part.clone())), |(acc, part), wf_step| {
            match part {
                Some(defined)   => {
                    let (if_true, if_false) = wf_step.condition.decompose(&defined);
                    (acc + if if_true.is_some() {
                        dfs(&if_true.unwrap(), &wf_step.workflow_if_successful, wf_map)
                    } else {
                        0
                    }, if_false)
                },
                None                   => (acc, part)
            }
        });

        // Get the default wf
        let other_combinations = match remaining_parts {
            Some(part)    =>   if wf.default_workflow == "A" {
                part.combinations()
            } else if wf.default_workflow == "R" {
                0
            } else {
                dfs(&part, &wf.default_workflow, wf_map)
            },
            None          =>   0
        };
        sum + other_combinations
    } else {
        0
    }
}

fn main() {
    let (_, workflows) = read_to_string("./src/input.txt")
        .unwrap()
        .lines()
        .fold((false, HashMap::new()),
              |(parse_part, mut workflows), line| {

                  if line.is_empty() || parse_part {
                      (true, workflows)
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
                                        let threshold = condition[2..].parse::<u64>().unwrap();
                                        let condition = Condition{ attribute, is_less_than, threshold};
                                        workflow_steps.push(WorkflowStep{condition, workflow_if_successful});
                                        (workflow_steps, default_workflow)
                                    } else {
                                        (workflow_steps, String::from(condition_string))
                                    }
                                });
                      workflows.insert(String::from(workflow_name), Workflow{steps, default_workflow});
                      (parse_part, workflows)
                  }
              });


    println!("{} ", dfs(&Part::new(), "in", &workflows));
    // let accepted_parts_sum =
    //     parts.into_iter().filter(|part| eval_workflows_rec(part, "in", &workflows) == "A")
    //         .fold(0, |acc, part| acc + part.sum());

    // let (o1, o2) = Condition{attribute: 'x', is_less_than: true, threshold: 1000}.decompose(Part::new());
    // println!("{:?}, {:?}", o1, o2);
    // let (o1, o2) = Condition{attribute: 'm', is_less_than: true, threshold: 1000}.decompose(Part::new());
    // println!("{:?}, {:?}", o1, o2);
    // let (o1, o2) = Condition{attribute: 'a', is_less_than: true, threshold: 1000}.decompose(Part::new());
    // println!("{:?}, {:?}", o1, o2);
    // let (o1, o2) = Condition{attribute: 's', is_less_than: true, threshold: 1000}.decompose(Part::new());
    // println!("{:?}, {:?}", o1, o2);




}
