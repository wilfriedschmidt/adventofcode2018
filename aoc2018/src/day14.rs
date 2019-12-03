//
//  Advent of Code Challenge, Solution for Day 14:
//  https://adventofcode.com/2018/day/14
//
//
//  MIT License
//
//  Copyright (c) 2019 Wilfried Schmidt
//
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
//
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
//
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.
//

use std::collections::HashMap;
use util::*;

fn print_recipes(recipes:&Vec<u8>, numrecipes:usize, elf0:usize, elf1:usize)
{
  let mut outstr = String::new();
  for i in 0..numrecipes
  {
    if i==elf0 
    { 
      outstr.push('(');
      outstr.push_str(&recipes[i].to_string());
      outstr.push(')'); 
    }
    else if i==elf1 
    { 
      outstr.push('[');
      outstr.push_str(&recipes[i].to_string());
      outstr.push(']'); 
    }
    else
    {
      outstr.push(' ');
      outstr.push_str(&recipes[i].to_string());
      outstr.push(' ');
    }
  }

  println!("{}", outstr);
}

pub fn go(numiterations:usize)
{

  let mut recipes:Vec<u8> = Vec::new();
  recipes.resize(1000*1000*100,0);

  let mut elf0=0;
  let mut elf1=1;  

  recipes[elf0] = 3;
  recipes[elf1] = 7;
  let mut numrecipes = 2;

  print_recipes(&recipes, numrecipes, elf0,elf1);


  let mut searchvec:Vec<u8> = Vec::new();
  let mut num = numiterations;  
  loop
  {
    searchvec.push((num%10) as u8);
    num/=10;
    if num==0 {break;}
  }
  searchvec.reverse();

  let mut temp = String::new();
  for i in 0..searchvec.len()
  {
    temp.push_str(&searchvec[i].to_string());
  }
  println!("{}", temp);


  let mut acc = 0;

  loop
  {

    if acc%10000==0 
    {
      println!("{}", numrecipes);
    }
    acc+=1;

    let mut sum = recipes[elf0];
    sum+=recipes[elf1];

    if sum<10
    {
      recipes[numrecipes] = sum % 10;
      numrecipes+=1;
    }
    else
    {
      recipes[numrecipes+1] = sum % 10;
      sum/=10;        
      recipes[numrecipes] = sum % 10;
      numrecipes+=2;
    }

    elf0 = ((recipes[elf0] + 1) as usize + elf0) % numrecipes;
    elf1 = ((recipes[elf1] + 1) as usize + elf1) % numrecipes;


    if acc>1000*1000*50
    {
      println!("scanning whole array");
      let mut found=true;
      for j in 0..numrecipes
      {
        found = true;  
        for i in 0..searchvec.len()
        {
          if recipes[j+i] != searchvec[i]
          {
            found=false;
            break;
          }
        }
      
        if found
        {
          println!("num recipes {}", j);
          break;  
        }
      }

      if found { break; }
    }

    if numrecipes > searchvec.len()+2
    {
      let mut found=true;
      for i in 0..searchvec.len()
      {
        if recipes[numrecipes - searchvec.len() + i + 1] != searchvec[i]
        {
          found=false;
          break;
        }
      }
    
      if found
      {
        println!("num recipes {}", numrecipes);
        break;  
      }

      
      found=true;
      for i in 0..searchvec.len()
      {
        if recipes[numrecipes - searchvec.len() + i] != searchvec[i]
        {
          found=false;
          break;
        }
      }
    
      if found
      {
        println!("num recipes {}", numrecipes - searchvec.len());
        break;  
      }     
    }
  }
}
