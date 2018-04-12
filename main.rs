extern crate rand;

use rand::{Rng, thread_rng};

#[derive(Debug, Clone, Copy)]
struct Node {
    op: Operator,
    val: i64,
    mut_rate: f32
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
    let mut ai = [[Node {op: Operator::Add, val: 0, mut_rate: 0.5}; 64]; 64];
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
            node.mut_rate = rng.gen();
        }
    }
    
    let mut best = 4294967295;
    let mut best_of_the_best = 0;
    for _ in 0..256 {
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
                while j < ai[i].len() {
                    if rng.gen::<f32>() < ai[i][j].mut_rate {
                        let best = ai[best_ai];
                        let random = rng.gen_range(0, best.len());
                        
                        ai[i][j].op = best[random].op;
                        ai[i][j].val = best[random].val;
                        ai[i][j].mut_rate = best[random].mut_rate;
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
                        ai[i][j].mut_rate = rng.gen();
                    }
                    
                    j += 1;
                }
            }
            
            i += 1;
        }
        
        best_of_the_best = best_ai;
    }
    
    println!("\nFinal Output: {}", best);
    
    let mut res: i64 = rng.gen_range(-64, 64);
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
            Operator::Div => if node.val != 0 {
                print!("{} / {} = ", res, node.val);
                res /= node.val;
                println!("{}", res);
            }
        }
    }
}
