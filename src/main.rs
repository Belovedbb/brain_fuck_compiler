use std::io;

fn main() {
    starter("++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.");
}

pub fn starter(string: &str) {
    compile(string);
}

fn compile(string: &str) {
    let mut while_rec_stop: bool = false;
    let mut memory_space: Vec<Option<u32>> = Default::default();
    memory_space.resize(20, None);
    let mut p_ptr: usize = 0;
    let mut ct: usize = 0;
    inner_looping(string, &mut ct, &mut p_ptr, &mut memory_space, &mut while_rec_stop)
}

fn inner_looping(string: &str, ct: &mut usize, p_ptr: &mut usize,
    memory_space: &mut Vec<Option<u32>>, while_rec_stop: &mut bool) {
        if memory_space.len() > 30_000 {
            panic!("error has occurred")
        }
    while *while_rec_stop == false && *ct < (*string).len() {
        functionality(string, ct, p_ptr, memory_space, while_rec_stop);
       *ct += 1;
    }
}

#[allow(mutable_borrow_reservation_conflict)]
fn functionality(string: &str, ct: &mut usize, p_ptr: &mut usize,
      memory_space: &mut Vec<Option<u32>>, while_rec_stop: &mut bool ) {
    let current_char = string.chars().nth(*ct).unwrap();
    match current_char {
        '>' => *p_ptr += 1,
        '<' => {
            if *p_ptr > 0 {
                *p_ptr -= 1
            } else {
                panic!("negative pointer not supported")
            }
        },
        '.' => {
            let val = memory_space.get(*p_ptr).unwrap();
            match val {
                Some(x) => {
                    print!("{}", char::from_u32(*x).unwrap());
                },
                None => panic!("error has occurred")
            }
        },
        ',' => {
            let mut input_ = String::new();
            io::stdin().read_line(&mut input_).unwrap();
            let input_ = input_.chars().nth(0);
            match input_ {
                Some(x) => {
                    memory_space[*p_ptr] = Some(x as u32);
                },
                None => panic!("error has occurred")
            }
        },
        '[' => {
            let condition_ptr_index = *p_ptr;
            while memory_space[condition_ptr_index] != None && memory_space[condition_ptr_index].unwrap() > 0 {
                let x_count = *ct;
                *ct += 1;
                inner_looping(string, ct, p_ptr, memory_space, while_rec_stop);
                *while_rec_stop = false;
                if memory_space[condition_ptr_index] != None && memory_space[condition_ptr_index].unwrap() > 0 {
                    let word = &string[x_count..];
                    let index_option = word.find(']');
                    match index_option {
                        Some(x) => *ct -= x + 1,
                        None => panic!("error has occurred")
                    }
                }else {
                    *ct -= 1;
                }
            }
        },
        ']' => {
            *while_rec_stop = true;
        },
        '+' => {
            let val = memory_space.get(*p_ptr).unwrap();
            match val {
                Some(x) => {
                    memory_space[*p_ptr] = Some(*x + 1);
                },
                None => {
                    memory_space[*p_ptr] = Some(1);
                }
            }
        },
        '-' => {
            let val = memory_space.get(*p_ptr).unwrap();
            match val {
                Some(x) => {
                    if *x == 0 {
                        memory_space[*p_ptr] = None;
                    }else {
                        memory_space[*p_ptr] = Some(x - 1);
                    }
                },
                None => {
                    memory_space[*p_ptr] = None;
                }
            }
        }
       _ => ()
   }
}
