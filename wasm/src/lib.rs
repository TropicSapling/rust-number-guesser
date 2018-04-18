use std::{
	num::Wrapping,
	ffi::CString,
	os::raw::c_char,
	panic
};

#[derive(Debug, Clone, Copy)]
struct Node {
    op: Operator,
    val: Wrapping<isize>,
    mut_rate: f64,
    mut_rate2: f64
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

extern {
	fn print_js(s: *mut c_char, l: usize);
	fn rand_js() -> f64;
	fn rand_bool_js() -> bool;
	fn rand_range_js(min: isize, max: isize) -> isize;
}

fn print(s: String) {
	let length = s.len();
	
	unsafe {
		print_js(CString::new(s).unwrap().into_raw(), length);
	}
}

fn rand() -> f64 {
	unsafe {
		rand_js()
	}
}

fn rand_bool() -> bool {
	unsafe {
		rand_bool_js()
	}
}

fn rand_range(min: isize, max: isize) -> isize {
	unsafe {
		rand_range_js(min, max)
	}
}

#[no_mangle]
pub extern fn run() {
	print(format!("Running..."));
	
	panic::set_hook(Box::new(|info| {
        print(format!("ERROR: {}", info));
    }));
	
    let mut ai = std::iter::repeat(vec![]).take(256).collect::<Vec<_>>();
    for nodes in ai.iter_mut() {
        for _ in 0..rand_range(2, 8) {
            let random = rand();
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
                
                val: Wrapping(rand_range(-2048, 2048)),
                mut_rate: rand(),
                mut_rate2: rand()
            });
        }
    }
    
    let mut best = None;
    let mut best_of_the_best = 0;
    for _ in 0..256 {
        let mut closest = None;
        let mut best_ai = None;
        
        for (i, nodes) in ai.iter().enumerate() {
            let mut res = Wrapping(rand_range(-2048, 2048));
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
        
        print(format!("Output: {}", closest.unwrap()));
        
        let mut i = 0;
        while i < ai.len() {
            if i != best_ai.unwrap() {
                let mut j = 0;
                while j < ai[i].len() {
                    if rand() < ai[i][j].mut_rate {
                        let best = ai[best_ai.unwrap()].clone();
                        let random = rand_range(0, best.len() as isize) as usize;
                        ai[i][j] = best[random];
                        
                        if j < best.len() {
                            let random = rand();
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
                                
                                val: Wrapping(rand_range(-2048, 2048)),
                                mut_rate: rand(),
                                mut_rate2: rand()
                            });
                        } else if j > 0 {
                            ai[i].pop();
                        }
                    } else if rand() < ai[i][j].mut_rate2 {
                        let random = rand();
                        ai[i][j].op = if random < 0.25 {
                            Operator::Add
                        } else if random < 0.5 {
                            Operator::Sub
                        } else if random < 0.75 {
                            Operator::Mul
                        } else {
                            Operator::Div
                        };
                        print(format!("[DEBUG] 6")); // DEBUG
                        ai[i][j].val = Wrapping(rand_range(-2048, 2048));
                        ai[i][j].mut_rate = rand();
                        ai[i][j].mut_rate2 = rand();
                        
                        if rand_bool() {
                            let random = rand();
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
                                
                                val: Wrapping(rand_range(-2048, 2048)),
                                mut_rate: rand(),
                                mut_rate2: rand()
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
    
    print(format!("\nFinal Output: {}", best.unwrap()));
    
    let mut res = Wrapping(rand_range(-2048, 2048));
    for node in ai[best_of_the_best].iter() {
        match node.op {
            Operator::Add => {
                let s = format!("{} + {} = ", res, node.val);
                res += node.val;
                print(s + &format!("{}", res));
            },
            Operator::Sub => {
                let s = format!("{} - {} = ", res, node.val);
                res -= node.val;
                print(s + &format!("{}", res));
            },
            Operator::Mul => {
                let s = format!("{} * {} = ", res, node.val);
                res *= node.val;
                print(s + &format!("{}", res));
            },
            Operator::Div => if node.val != Wrapping(0) {
                let s = format!("{} / {} = ", res, node.val);
                res /= node.val;
                print(s + &format!("{}", res));
            }
        }
    }
}