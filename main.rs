extern crate rand;

use rand::{Rng, thread_rng};
use std::num::Wrapping;

#[derive(Debug, Clone, Copy)]
struct Node {
    op: Operator,
    val: Wrapping<i64>,
    mut_rate: f32,
    mut_rate2: f32
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

fn main() {
    let mut rng = thread_rng();
    let mut ai = std::iter::repeat(vec![]).take(1024).collect::<Vec<_>>();
    for nodes in ai.iter_mut() {
        for _ in 0..rng.gen_range(2, 16) {
            let random: f32 = rng.gen();
            nodes.push(Node {
                op: if random < 0.25 {
                    Operator::Add
                } else if random < 0.5 {
                    Operator::Sub
                } else if random < 0.75 {
                    Operator::Mul
                } else {
                    Operator::Div
                },
                
                val: Wrapping(rng.gen_range(-2048, 2048)),
                mut_rate: rng.gen(),
                mut_rate2: rng.gen()
            });
        }
    }
    
    let mut best = None;
    let mut best_of_the_best = 0;
    for _ in 0..1024 {
        let mut closest = None;
        let mut best_ai = None;
        
        for (i, nodes) in ai.iter().enumerate() {
            let mut res = Wrapping(rng.gen_range(-2048, 2048));
            for node in nodes.iter() {
                match node.op {
                    Operator::Add => res += node.val,
                    Operator::Sub => res -= node.val,
                    Operator::Mul => res *= node.val,
                    Operator::Div => if node.val != Wrapping(0) {
                        res /= node.val
                    }
                }
            }
            
            let diff = if res > Wrapping(1337) {
                res - Wrapping(1337)
            } else {
                Wrapping(1337) - res
            };
            
            match closest {
                None => {
                    closest = Some(diff);
                    best_ai = Some(i);
                },
                
                Some(ref mut closest) => if diff < *closest || (diff == *closest && nodes.len() < ai[best_ai.unwrap()].len()) {
                    *closest = diff;
                    best_ai = Some(i);
                }
            }
        }
        
        match best {
            None => best = Some(closest.unwrap()),
            Some(ref mut best) => if closest.unwrap() < *best {
                *best = closest.unwrap();
            }
        }
        
        println!("Output: {}", closest.unwrap());
        
        let mut i = 0;
        while i < ai.len() {
            if i != best_ai.unwrap() {
                let mut j = 0;
                while j < ai[i].len() {
                    if rng.gen::<f32>() < ai[i][j].mut_rate {
                        let best = ai[best_ai.unwrap()].clone();
                        let random = rng.gen_range(0, best.len());
                        
                        ai[i][j] = best[random];
                        
                        if j < best.len() {
                            let random: f32 = rng.gen();
                            ai[i].push(Node {
                                op: if random < 0.25 {
                                    Operator::Add
                                } else if random < 0.5 {
                                    Operator::Sub
                                } else if random < 0.75 {
                                    Operator::Mul
                                } else {
                                    Operator::Div
                                },
                                
                                val: Wrapping(rng.gen_range(-2048, 2048)),
                                mut_rate: rng.gen(),
                                mut_rate2: rng.gen()
                            });
                        } else if j > 0 {
                            ai[i].pop();
                        }
                    } else if rng.gen::<f32>() < ai[i][j].mut_rate2 {
                        let random: f32 = rng.gen();
                        ai[i][j].op = if random < 0.25 {
                            Operator::Add
                        } else if random < 0.5 {
                            Operator::Sub
                        } else if random < 0.75 {
                            Operator::Mul
                        } else {
                            Operator::Div
                        };
                        
                        ai[i][j].val = Wrapping(rng.gen_range(-2048, 2048));
                        ai[i][j].mut_rate = rng.gen();
                        ai[i][j].mut_rate2 = rng.gen();
                        
                        if rng.gen() {
                            let random: f32 = rng.gen();
                            ai[i].push(Node {
                                op: if random < 0.25 {
                                    Operator::Add
                                } else if random < 0.5 {
                                    Operator::Sub
                                } else if random < 0.75 {
                                    Operator::Mul
                                } else {
                                    Operator::Div
                                },
                                
                                val: Wrapping(rng.gen_range(-2048, 2048)),
                                mut_rate: rng.gen(),
                                mut_rate2: rng.gen()
                            });
                        } else if j > 0 {
                            ai[i].pop();
                        }
                    }
                    
                    j += 1;
                }
            }
            
            i += 1;
        }
        
        best_of_the_best = best_ai.unwrap();
    }
    
    println!("\nFinal Output: {}", best.unwrap());
    
    let mut res = Wrapping(rng.gen_range(-2048, 2048));
    for node in ai[best_of_the_best].iter() {
        match node.op {
            Operator::Add => {
                print!("{} + {} = ", res, node.val);
                res += node.val;
                println!("{}", res);
            },
            Operator::Sub => {
                print!("{} - {} = ", res, node.val);
                res -= node.val;
                println!("{}", res);
            },
            Operator::Mul => {
                print!("{} * {} = ", res, node.val);
                res *= node.val;
                println!("{}", res);
            },
            Operator::Div => if node.val != Wrapping(0) {
                print!("{} / {} = ", res, node.val);
                res /= node.val;
                println!("{}", res);
            }
        }
    }
}