extern crate rand;
extern crate regex;
extern crate ansi_term;

use std::process;
use std::env;
use rand::Rng;
use regex::Regex;
use ansi_term::Colour;
use ansi_term::Style;

fn main() {
  #[cfg(target_os = "windows")]
  ansi_term::enable_ansi_support().ok();

  let args: Vec<String> = env::args().collect();
  if args.len() <= 1 {
    println!("{}", Style::new().bold().paint("Dice roll is missing"));
    show_help();
    process::exit(0x0100);
  }
  let roll = &args[1];
  
  let roll_parts = parse_dice(roll);

  println!("Rolling: {}", roll);

  if roll_parts[0] > 0 {
    let mut total: i32 = 0;
    let mut dice_roll: i32 = 0;
    for n in 0..roll_parts[0] {
      dice_roll = roll_dice(roll_parts[1]);
      println!("Dice {:>4} = {}", n+1, Colour::Yellow.paint(dice_roll.to_string()));
      total = total + dice_roll;
    }
    if roll_parts[2] != 0 {
      let mut sign = "";
      if roll_parts[2] > 0 {
        sign = "+";
      }
      println!("Modifier: {} {}{}", "", Colour::Cyan.paint(sign), Colour::Cyan.paint(roll_parts[2].to_string()));
    }
    println!("Total: {:>2} = {}", "", Style::new().bold().paint((total + roll_parts[2]).to_string()));
  }
}

fn roll_dice(mut n: i32) -> i32 {
  if n < 2 { 
    n = 2;
  }
  let mut rng = rand::thread_rng();
  return rng.gen_range(1, n+1);
}

fn parse_dice(input: &str) -> [i32;3] {
  let mut multiplier: i32 = 0;
  let mut dice: i32 = 0;
  let mut modifier: i32 = 0;
  // split die by (n)d(m)+|-(k)
  let re = Regex::new(r"(?P<m>\d+|.*)d(?P<d>\d+)(?P<mod>\+\d+|\-\d+|.*)").unwrap();
  let foo = re.captures(input);
  match foo {
    Some(caps) => {
      let cap1 = caps.name("m").unwrap().as_str();
      match cap1.parse::<i32>() {
        Ok(ok) => { multiplier = ok; },
        Err(e) => { multiplier = 1 }
      }
      let cap2 = caps.name("d").unwrap().as_str();
      match cap2.parse::<i32>() {
        Ok(ok) => { dice = ok; },
        Err(e) => println!("Not a DICE ({:?}))", e)
      }
      let cap3 = caps.name("mod").unwrap().as_str();
      match cap3.parse::<i32>() {
        Ok(ok) => { modifier = ok; },
        Err(e) => { modifier = 0; }
      }
    }
    None => {
      // Not a valid dice roll
      println!("{}", Style::new().bold().paint("Unable to parse dice roll!"));
      show_help();
      process::exit(0x0100);
    }
  }
  return [multiplier, dice, modifier];
}

fn show_help() {
  println!("Usage example: rolldice 2d6+1");
  println!("Number of dice and modifier are optional. Modifier can be + or -.");
}
