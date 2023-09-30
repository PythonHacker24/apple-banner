
extern crate sysinfo;
extern crate chrono;
extern crate colored;

use sysinfo::{System, SystemExt, CpuExt};
use chrono::Local;
use colored::*;

fn date_and_time() -> String {
    
    let current_datetime = Local::now();
    let formatted_datetime = current_datetime.format("%d-%m-%Y %H:%M:%S");

    return formatted_datetime.to_string();
}

fn logo() {
    
    let owner = String::from("Aditya Patil");
    let time_info = date_and_time(); 
    let logo_string = format!(r#"
                     ⣀⣀⠀⠀⠀⠀⠀⠀
      ⠀⠀⠀⠀⠀⠀⠀⠀⠀ ⠀⠀⢀⣴⣿⣿⡿⠀⠀⠀⠀⠀⠀
      ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣾⣿⣿⠟⠁⠀⠀⠀⠀⠀⠀      Date: {}
      ⠀⠀⠀⢀⣠⣤⣤⣤⣀⣀⠈⠋⠉⣁⣠⣤⣤⣤⣀⡀⠀⠀
      ⠀⢠⣶⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣦⡀      {}
      ⣠⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠟⠋⠀
      ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡏⠀⠀⠀      Device: Apple Macbook Air M2 8GB RAM
      ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀
      ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣧⠀⠀⠀      Designed by Apple in California
      ⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣤⣀
      ⠀⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠁      Developed by: Steve Jobs
      ⠀⠀⠙⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡟⠁⠀                    Steve Wozniak
      ⠀⠀⠀⠈⠙⢿⣿⣿⣿⠿⠟⠛⠻⠿⣿⣿⣿⡿⠋⠀⠀⠀                    Ronald Wayne
            
  "#, time_info, owner).white().bold();

  println!("{}", logo_string);
}

fn return_bar(percent: &mut f32) -> ColoredString {
   if *percent < 25.0 {
        let bar_char = "■".cyan();
        return bar_char;
   } else if *percent > 25.0 && *percent < 80.0 {
       let bar_char = "■".yellow();
       return bar_char;
   } else if *percent > 80.0 {
       let bar_char = "■".red();
       return bar_char;
   }
   return "■".white();
}

fn plot_cpu(percent: f32) {

    let mut percent_mut = percent;
    let bar_char = return_bar(&mut percent_mut); 

    for _ in 0..=((percent/2.0) as i32) {
        print!("{}", bar_char);
    }
    
    for _ in 0..=((50.0 - (percent/2.0)) as i32) {
        print!(" ");
    }
    
    let colored_percent = format!(r#"{}"#, percent).white().bold();
    print!("{}", colored_percent);
}


fn plot_ram(value: &mut u64, total_value: &mut u64) {

    let percentage = (*value as f64 / *total_value as f64 * 100.0) as u64;

    let label = "Memory Usage: ".cyan().bold();
    print!("{}", label);
    
    print!("[ ");
    let bar_char = "█".cyan();
    let bar = &bar_char;

    for _ in 0..=(percentage / 2) {
        print!("{}", *bar);
    }
    for _ in 0..=(50 - (percentage / 2)) {
        print!("-");
    }

    println!(" ] {}%", percentage);
}

fn post_inc(counter: &mut i64) {
    *counter += 1;
}

fn main() {
    
    let mut sys = System::new_all();

    logo();
    let sample_text = "    [+] Apple Banner by Aditya. Developed in RUST\n".bold().cyan();
    println!("{}", sample_text);
    
    sys.refresh_all();

    let mut used_mem = sys.used_memory() / (1024 * 1024);
    let mut total_mem = sys.total_memory() / (1024 * 1024);
    
    print!("    ");
    plot_ram(&mut used_mem, &mut total_mem);
    
    let mem_info = format!(r#"
    Total Memory: {} MB
    Used Memory:  {} MB
    Total Swap:   {} MB
    Used Swap:    {} MB"#, (sys.total_memory() / (1024 * 1024)), (sys.used_memory() / (1024 * 1024)), (sys.total_swap() / (1024 * 1024)), (sys.used_swap() / (1024 * 1024))).magenta().bold();

    let cpu_num = format!(r#"
    Number of CPUs: {}
    "#, sys.cpus().len()).blue().bold();

    let system_info = format!(r#"
    System Name: {:?}
    System Kernel Verson: {:?}
    System OS version: {:?}
    System host name: {:?}"#, sys.name(), sys.kernel_version(), sys.os_version(), sys.host_name()).green().bold();
   
    println!("{}", mem_info);
    println!("{}", system_info);
    println!("{}", cpu_num);

    let mut counter = 1;
    sys.refresh_cpu(); 
    for cpu in sys.cpus() {
        let count = format!(r#"    CPU: {} "#, counter).green().bold();
        print!("{}", count);
        plot_cpu(cpu.cpu_usage());
        print!("\n");
        post_inc(&mut counter);
    }

    print!("\n");
    std::thread::sleep(System::MINIMUM_CPU_UPDATE_INTERVAL);
}
