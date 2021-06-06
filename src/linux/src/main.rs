use std::env::args;
use std::process::exit;
use colored::Colorize;
fn main() {
    let mut arg_vector: Vec<String> = args().collect();
    arg_vector.remove(0);
    let mut arg_vector_copy: Vec<String> = Vec::new();
    let arg_vector = arg_vector;
    let mut full_mode:bool = false;
    let mut bare_mode:bool = false;
    let help_closure = |err_num| {
        let opts_help_txt = String::from("OPTIONS").green().bold();
        let usage_help_txt = String::from("USAGE").cyan().bold(); 
        let x_help_txt = String::from("X COORDINATE").yellow().bold();
        let z_help_txt = String::from("Z COORDINATE").blue().bold();
        let err_help_txt = String::from("ERRORS").red().bold();
        print!("\
            \n Get Minecraft chunk boundary coordinates.\
            \n E.g. Provide coordinates X: 7 and Z: 4 to get\
            \n the boundary of 0,0 to 15,15.\
            \n\n @{}:\n\tmcchunkget [{}...] <{}> <{}>\
            \n\tmcchunkget <{}> <{}>\
            \n\n @{}:\n\t-h,--help\tThis HELP message.\
            \n\t-f,--full\tAll four corners.\
            \n\t-b,--bare\tReturn coordinates only.\
            \n\n @{}:\n\tInteger\t\tX coordinate inside of chunk.\
            \n\n @{}:\n\tInteger\t\tZ coordinate inside of chunk.\
            \n\n @{}: Integers - Exit Codes.\
            \n\t1\t\tNot enough parameters passed.\
            \n\t2\t\tParameter provided for X is not an integer.\
            \n\t3\t\tParameter provided for Z is not an integer.\
            \n\n",usage_help_txt,opts_help_txt
                ,x_help_txt,z_help_txt
                ,x_help_txt,z_help_txt
                ,opts_help_txt
                ,x_help_txt,z_help_txt,
                err_help_txt);
        exit(err_num);
    };
    for arg_iter_item in &arg_vector {
        let arg_case_conv = &arg_iter_item.to_lowercase();
        if arg_case_conv == "--help" || arg_case_conv == "-h" {
            help_closure(0);
        } else if arg_case_conv == "--full" || arg_case_conv == "-f" {
            full_mode = true;
        } else if arg_case_conv == "--bare" || arg_case_conv == "-b" {
            bare_mode = true;
        } else {
            arg_vector_copy.push(arg_iter_item.to_string());
        }
    }
    if arg_vector_copy.len() < 2 {
        help_closure(1);
    }
    let x_coord_int_test = arg_vector_copy[0].trim().parse::<i32>();
    match x_coord_int_test {
        Ok(_) => {},
        Err(_) => {help_closure(2);}
    }
    let z_coord_int_test = arg_vector_copy[1].trim().parse::<i32>();
    match z_coord_int_test {
        Ok(_) => {},
        Err(_) => {help_closure(3);}
    }
    let x_coord_int:i32 = x_coord_int_test.unwrap();
    let z_coord_int:i32 = z_coord_int_test.unwrap();
    let nearest = |mut i:i32| {
        while i%16!=0 {
            i -= 1;
        }
        return i;
    };
    let farthest = |mut i:i32| {
        i += 15;
        return i;
    };
    let x_coord_near = nearest(x_coord_int);
    let x_coord_far = farthest(x_coord_near);
    let z_coord_near = nearest(z_coord_int);
    let z_coord_far = farthest(z_coord_near);
    match full_mode {
        false => {
            match bare_mode {
                false => {
                    println!(" \
                    Chunk boundaries of X:{} , Z:{}:\n {},{} to {},{}"
                        ,x_coord_int,z_coord_int
                        ,x_coord_near,z_coord_near
                        ,x_coord_far,z_coord_far);                
                },
                true => {
                    println!("{},{} {},{}"
                        ,x_coord_near,z_coord_near
                        ,x_coord_far,z_coord_far);
                }
            }        
        },
        true => {
            match bare_mode {
                false => {
                    println!(" \
                        Chunk boundaries of X:{} , Z:{}:\
                        \n {},{} to {},{} to {},{} to {},{}"
                        ,x_coord_int,z_coord_int
                        ,x_coord_near,z_coord_near
                        ,x_coord_near,z_coord_far
                        ,x_coord_far,z_coord_far
                        ,x_coord_far,z_coord_near);
                },
                true => {
                    println!("{},{} {},{} {},{} {},{}"
                        ,x_coord_near,z_coord_near
                        ,x_coord_near,z_coord_far
                        ,x_coord_far,z_coord_far
                        ,x_coord_far,z_coord_near);
                }
            }              
        }
    }
}
