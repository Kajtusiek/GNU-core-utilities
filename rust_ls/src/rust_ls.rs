use std::{env, fs};
use std::path::Path;

fn ls_r(path: String, tabs_nr: u8)
{
    let it = match fs::read_dir(Path::new(&path)){
        Ok(directories) => directories,
        Err(_) => panic!("Path doesn't exist")
    };
    let tabs = "\t".repeat(tabs_nr as usize);
    println!("{}--Names--",tabs);
    for i in it
    {
        let dir_entry = match i{
            Ok(dir) => dir,
            Err(_) => panic!("Directory name error")
        };
        let name :String  = match dir_entry.file_name().into_string() {
            Ok(string) => string,
            Err(_) => panic!("Directory name couldn't be converted into string")
        };
        if name.contains(".")
        {
            println!("{}{}", tabs, name)
        }
        else
        {
            println!("\n{}Printing {} directory:", tabs, name);
            let new_patch: String = format!("{}\\{}", path, name);
            ls_r(new_patch, tabs_nr + 1);
        }
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    let (mut switch_l, mut switch_recursive, mut switch_a, mut switch_h, mut switch_sort_n, mut switch_sort_s, mut switch_sort_t, mut switch_sort_v, mut switch_sort_x, mut switch_sort_reverse) : (bool, bool, bool, bool, bool, bool, bool, bool, bool, bool);
    let mut paths : Vec<String>;

    for arg in args{
        if arg == "-a" || arg == "--all"{
            switch_a = true;
        }
        else if arg == "-l"{
            switch_l = true;
        }
        else if arg == "-R" || arg == "--recursive"{
            switch_recursive = true;
        }
        else if arg == "-h" || arg == "--human-readable"{
            switch_h = true;
        }
        else if arg == "-U" || arg == "--sort=none"{
            switch_sort_n = true;
        }
        else if arg == "-S" || arg == "--sort=size"{
            switch_sort_s = true;
        }
        else if arg == "-t" || arg == "--sort=time"{
            switch_sort_t = true;
        }
        else if arg == "-v" || arg == "--sort=version"{
            switch_sort_v = true;
        }
        else if arg == "-X" || arg == "--sort=extension"{
            switch_sort_x = true;
        }
        else if arg == "-r" || arg == "--reverse"{
            switch_sort_reverse = true;
        }
        else{
            paths.push(arg);
        }
    }


    if args.len() < 2
    {

        path = ".".to_string();
    }
    else
    {
        path = args[1].clone();
    }
    //ls_r(path, 0); tak wywolac ale usunac wszystko co jest na dole
    let it = match fs::read_dir(Path::new(&path)){
        Ok(directories) => directories,
        Err(_) => panic!("Path doesn't exist")
    };
    for i in it
    {

        let dir_entry = match i{
            Ok(dir) => dir,
            Err(_) => panic!("Directory name error")
        };
        println!("{}", match dir_entry.file_name().into_string(){
            Ok(string) => string,
            Err(_) => panic!("Directory name couldn't be converted into string")
        });
    }
}

