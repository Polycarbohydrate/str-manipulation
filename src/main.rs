use std::io;

fn main() {
    loop{
        println!("-------------------------------------------------------------------------------------");
        println!("Press '1' for concatenation of 5 strings from the user.");
        println!("Press '2' for finding the string length of a string.");
        println!("Press '3' for splitting a string given by the user at a point identified by the user.");
        println!("-------------------------------------------------------------------------------------");
    
        let mut a = String::new();
    
        io::stdin().read_line(&mut a).expect("Could not read line.");
    
        let a: u32 = match a.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if a == 1   {
            concatenate();
            break;
        }
        else if a == 2  {
            println!("------------------------------------------------");
            println!("Enter you string you want to find the length of.");
            println!("------------------------------------------------");
            let mut a = String::new();
            io::stdin().read_line(&mut a).expect("Could not read line.");
            let a = a.trim();
            let find_len = str_len(&a);
            println!("----------------------");
            println!("The string: {a}");
            println!("The length: {find_len}");
            println!("----------------------");
            break;
        }
        else if a == 3  {
            splitstr();
            break;
        }
        else    {
            println!("---------------------------------------------------------");
            println!("Press either '1','2', or '3'. These are the only options.");
            println!("---------------------------------------------------------");
            continue
        }
    }
}

fn concatenate()    {
    println!("-------------------------------------------------------------------------------------------");
    println!("Input the five sets of strings. Start a new line for the next set by pressing enter/return.");
    println!("-------------------------------------------------------------------------------------------");
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    let mut d = String::new();
    let mut e = String::new();
    
    io::stdin().read_line(&mut a).expect("Could not read line.");
    io::stdin().read_line(&mut b).expect("Could not read line.");
    io::stdin().read_line(&mut c).expect("Could not read line.");
    io::stdin().read_line(&mut d).expect("Could not read line.");
    io::stdin().read_line(&mut e).expect("Could not read line.");
    
    let a = a.trim();
    let b = b.trim();
    let c = c.trim();
    let d = d.trim();
    let e = e.trim();

    let mut concat = String::from("");
    
    concat.push_str(a);
    concat.push_str(b);
    concat.push_str(c);
    concat.push_str(d);
    concat.push_str(e);
    
    println!("------------------------------------");
    println!("The concatenated string is: {concat}");
    println!("------------------------------------");
}

fn str_len(a: &str) -> usize  {
    a.len()
}

fn splitstr()   {
    println!("------------------------------------");
    println!("Enter your string you want to split.");
    println!("------------------------------------");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Could not read line.");
    let a = &a.trim();
    let a_len = a.len();
    loop{
        println!("------------------------------------------------------------------------------------------------------");
        println!("Enter the index of the point you want to be the center of the split. (Index 0 is the first character.)");
        println!("------------------------------------------------------------------------------------------------------");
        let mut b = String::new();
        io::stdin().read_line(&mut b).expect("Could not read line.");
        let c: usize = match b.trim().parse()   {
            Ok(num) => num,
            Err(_) => continue,
        };
        if c > a_len    {
            println!("----------------------------------------------");
            println!("Enter an index within the range of the string.");
            println!("----------------------------------------------");
            continue
        }   else    {
                let start_cut = &a[..c];
                let end_cut = &a[c..];
                println!("---------------------------");
                println!("The first part: {start_cut}");
                println!("The last part: {end_cut}");
                println!("---------------------------");
                break;
        }
    }
}
