extern crate rand;

use rand::{Rng, thread_rng};

#[derive(Debug, Clone, Copy)]
struct Node {
    op: Operator,
    val: i64
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
    let mut ai = [[Node {op: Operator::Add, val: 0}; 64]; 64];
    for nodes in ai.iter_mut() {
        for node in nodes.iter_mut() {
            let random: f32 = rng.gen();
            node.op = if random < 0.25 {
                Operator::Add
            } else if random < 0.5 {
                Operator::Sub
            } else if random < 0.75 {
                Operator::Mul
            } else {
                Operator::Div
            };
            
            node.val = rng.gen_range(-8, 8);
        }
    }
    
    let mut best = 4294967295;
    for _ in 0..16 {
        let mut closest = 4294967295;
        let mut best_ai = 0;
        
        for (i, nodes) in ai.iter().enumerate() {
            let mut res: i64 = rng.gen_range(-64, 64);
            for node in nodes.iter() {
                match node.op {
                    Operator::Add => res += node.val,
                    Operator::Sub => res -= node.val,
                    Operator::Mul => res *= node.val,
                    Operator::Div => if node.val != 0 {
                        res /= node.val
                    }
                }
            }
            
            let diff = (res - 1337).abs();
            if diff < closest {
                closest = diff;
                best_ai = i;
            }
        }
        
        if closest < best {
            best = closest;
        }
        
        println!("Output: {}", closest);
        
        let mut i = 0;
        while i < ai.len() {
            if i != best_ai {
                let mut j = 0;
                while j < ai.len() {
                    if rng.gen() {
                        let best = ai[best_ai];
                        let random = rng.gen_range(0, best.len());
                        
                        ai[i][j].op = best[random].op;
                        ai[i][j].val = best[random].val;
                    } else {
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
                        
                        ai[i][j].val = rng.gen_range(-8, 8);
                    }
                    
                    j += 1;
                }
            }
            
            i += 1;
        }
    }
    
    println!("\nFinal Output: {}", best);
}
