use clap::{App, Arg, SubCommand, AppSettings};

use std::io::{BufReader, BufRead};

use rdag::*;

use termion::color;

fn main() {
    let matches =  App::new("rdag")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .author("Alessandro Bregoli")
        .about("Trivial Rust implementation of the single source shortest/longest path for a DAG")
        .arg(Arg::with_name("n_nodes")
             .short("n")
             .long("n_nodes")
             .value_name("Number of nodes")
             .help("Number of nodes in the DAG")
             .takes_value(true)
             .required(true))
        .arg(Arg::with_name("network_path")
             .short("p")
             .long("path")
             .value_name("Network file path")
             .help("File containing the edge list")
             .takes_value(true)
             .required(true))
        .subcommand(SubCommand::with_name("topological")
                    .about("Return the topological order of the network"))
        .subcommand(SubCommand::with_name("SLpath")
                    .about("Compute the single source shortest-logest path")
                    .arg(Arg::with_name("longest")
                         .short("l")
                         .long("longest")
                         .value_name("Longest")
                         .help("If this flag is present rdag searches for the longest path. Otherwise is searches the shortest path")
                         .takes_value(false))
                    .arg(Arg::with_name("source")
                         .short("s")
                         .long("source")
                         .value_name("Source")
                         .help("Source node for the shortest/longest path algorithm")
                         .takes_value(true)
                         .required(true))).get_matches();
    
    let path = std::path::Path::new(matches.value_of("network_path").unwrap());
    
    if !path.exists() {
        eprintln!("Path: '{}'  does not exists", path.display());
        std::process::exit(1);
    }

    let n_nodes: u64 = match matches.value_of("n_nodes").unwrap().parse(){
    
        Ok(x) => x,
        Err(_) => {
            eprintln!("The number of nodes must be a positive integer");
            std::process::exit(1);
        }
    };

    let dag_file = BufReader::new(std::fs::File::open(path).unwrap());
    let mut edge_list: Vec<(u64, u64, f64)> = Vec::new();

    for (_i,line) in dag_file.lines().enumerate() {
        let splitted = line.unwrap();
        let splitted: Vec<&str> = splitted.split_whitespace().collect();
        edge_list.push(
            (splitted[0].trim().parse().unwrap(),
            splitted[1].trim().parse().unwrap(),
            splitted[2].trim().parse().unwrap()));
    }

    let dag = DAG::init(&n_nodes, &edge_list);

    if let Some(_m) =  matches.subcommand_matches("topological") {
        let tsort = dag.topological_sort();
        print_bicolor_vector(&tsort);
    }
    
    if let Some(SLpath) = matches.subcommand_matches("SLpath") {
        let path_type = if SLpath.is_present("longest") {
            OptimalPathType::Longest}
        else{
            OptimalPathType::Shortest};

        let source:u64 = match SLpath.value_of("source").unwrap().parse(){
            Ok(x) => x,
            Err(_) => {
                eprintln!("The source node must be a positive integer");
                std::process::exit(1);
            }
        };
        let optimal_path = dag.optimal_path(&path_type, source);
        print_optimal_path(&path_type, &optimal_path, source);

    }


}

fn print_optimal_path(path_type: &OptimalPathType, optimal_path: &Vec<Step>, source: u64){
    match path_type {
        OptimalPathType::Shortest => print!("Shortest path"),
        OptimalPathType::Longest => print!("Longest path"),
    }
    println!(" from {}:", source);
    for (i, v) in optimal_path.iter().enumerate() {
        if i%2 == 0 {
            print!("{}", color::Fg(color::Green));
        } else {
            print!("{}", color::Fg(color::Red));
        };
        let predecessor = match v.predecessor {
            Some(x) => x.to_string(),
            None => String::from("None")
        };
        println!("Node: {} \t-\t Predecessor: {} \t\t Distance from source: {}", i, predecessor, v.distance  );
    }
    println!("{}",color::Fg(color::Reset));
    

}

fn print_bicolor_vector(vector: &Vec<u64>) {
    print!("Output: ");
    for (i, v) in vector.iter().enumerate() {
        if i%2 == 0 {
            print!("{}", color::Fg(color::Green));
        } else {
            print!("{}", color::Fg(color::Red));
        };
        print!("{} ", v);
    }
    println!("{}",color::Fg(color::Reset));
}
