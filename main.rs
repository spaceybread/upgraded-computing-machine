use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::os;
use std::fs::File;
use std::fs;

fn fib(n: i32) -> i32 {
    if n == 0 || n == 1{
        return 1
    }

    return fib(n - 1) + fib(n - 2);

}

fn main() -> std::io::Result<()> {
    loop {
        let p = env::current_dir()?;
        print!("{} > ", p.display());
        stdout().flush().unwrap();

        let mut linein: String = String::new(); 
        stdin().read_line(&mut linein).unwrap();
   
        let mut sequence = linein.trim().split(" || ").peekable();

        while let Some(seq) = sequence.next() {
            let mut splt = seq.trim().split_whitespace();
            let mut seq = splt.next().unwrap();
            let mut args = splt;

            
            match seq {
                "sum" => {
                    let mut out: i32 = 0; 
                    while let Some(dig) = args.next() {
                        out += dig.parse::<i32>().unwrap();
                    }
                    println!("{out}");
                }

                "prod" => {
                    let mut out: i32 = 1; 
                    while let Some(dig) = args.next() {
                        out *= dig.parse::<i32>().unwrap();
                    }
                    println!("{out}");
                }

                "exp" => {
                    let Some(ne): Option<&str> = args.next() else {panic!()};
                    let mut base: i32 = ne.parse::<i32>().unwrap();
                    let Some(ne): Option<&str> = args.next() else {panic!()};
                    let mut ex: i32 = ne.parse::<i32>().unwrap();
                    let mut prod = 1;
                    while ex > 0 {
                        prod *= base;
                        ex += -1;
                    }

                    println!("{prod}");
                }

                "mod" => {
                    let Some(ne): Option<&str> = args.next() else {panic!()};
                    let mut n: i32 = ne.parse::<i32>().unwrap();
                    let Some(ne): Option<&str> = args.next() else {panic!()};
                    let mut m: i32 = ne.parse::<i32>().unwrap();
                    let mut out = n % m;

                    println!("{out}");
                }

                "fib" => {
                    let Some(ne): Option<&str> = args.next() else {panic!()};
                    let mut n: i32 = ne.parse::<i32>().unwrap();

                    let out = fib(n);   
                    println!("{out}");
                }

                "cd" => {
                    let dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(dir);
                    assert!(env::set_current_dir(&root).is_ok());
                    let p = env::current_dir()?;
                    println!("Changed directory to {}", p.display()); 
                }

                "pwd" => {
                    let p = env::current_dir()?;
                    println!("{}", p.display());
                }

                "ls" => {
                    let paths = fs::read_dir("./").unwrap();
                    println!("Files found: ");
                    for path in paths {
                        println!("{}", path.unwrap().path().display());
                    }
                }

                "create" => {
                    //let name = args.peekable().peek();
                    let mut file = File::create("emptyFile")?;
                    //file.write_all(b"Hello, world!")?;
                }

                "rm" => {
                    fs::remove_file("emptyFile")?;
                }

                "exit" => {
                    std::process::exit(0);
                }

                "pi" => {
                    println!("3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679");
                }

                "clear" => {
                    //clearscreen::clear().expect("failed to clear screen");
                    println!("I haven't used cargo yet so have this lousy message!");
                }

                _ => {
                    print!("");
                }
            }   
        }
    }
}