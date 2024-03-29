// a test file that imports the library

extern mod re;
use std::{libc, io, run, os, path, uint};

use re::UncompiledRegexp;

fn main() {
  println("--Case 0--");
  let mut re = UncompiledRegexp::new("[a-z]d|abc");
  re.run("abc");

  println("--Case 1--");
  let mut re = UncompiledRegexp::new("(?:http(s)?://)?(www.)?([a-zA-Z0-9_.]+).(com|org|net|edu)/?");
  re.run("http://ferristseng.comuASDAFASFASBVZKXJVBKZXBVKJZBXVKBZXV");
  re.run("http://reddit.com/");
  re.run("https://google.com/");
  //re.run("NOT A WEBSITE");
  re.run("http://virginia.edu");
  re.run("www.cnn.com");

  println("--Case 2--");
  let mut re = UncompiledRegexp::new("[^a-zA-Z0-9]*");
  re.run("我是曾繁睿");

  println("--Case 2 (NonGreedy)--");
  let mut re = UncompiledRegexp::new("[^a-zA-Z0-9]*?");
  re.run("我是曾繁睿");

  println("--Case 3--");
  let mut re = UncompiledRegexp::new("(a+?)*");
  re.run("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");

  println("--case 3a--");
  re.findall("aaaaabaaaba");

  println("--Case 4--");
  let mut re = UncompiledRegexp::new("<([^>]+)>");
  re.run("<html><head></head><div></div></html>");

  

  // println("--Case 2--");
  // UncompiledRegexp::new("a|b|c").compile();

  // println("--Case 3--");
  // UncompiledRegexp::new("a|(Bcf)|dez").compile();

  // println("--Case 4--");
  // //UncompiledRegexp::new("abc*|d").parse();

  // println("--Case 5--");
  // //UncompiledRegexp::new("io(ab|c)*zz|(bcd)*").parse();

  // println("--Case 6--");
  // //UncompiledRegexp::new("あ(ab(cd|d)|e)|f").parse();

  /*
  println("--Case 7--");
  //Regexp::new("[[A-Z]0-9(fgh)]]]|[abc]").parse();
  Regexp::new("1\\d2").parse();

  println("--Case 8--");
  UncompiledRegexp::new("(abc){1,}").parse();

  println("--Case 9--");
  UncompiledRegexp::new("abc{3,4}?").parse();
  
  println("--Case 10--");
  UncompiledRegexp::new("a|b{3}").parse();

  println("--Case 11--");
  UncompiledRegexp::new("a{4,3}?").parse();

  println("--Case 12--");
  UncompiledRegexp::new("[C[e-h]arlemange]|bs|c").compile();

  println("--Case 13--");
  UncompiledRegexp::new("[^aA-ZA]").compile();

  println("--Case 14--");
  UncompiledRegexp::new("[^\U00000000-\U0010FFFF]").parse();

  println("--Case 15--");
  UncompiledRegexp::new("[^a-f]").parse();
  
  println("--Case 16--");
  UncompiledRegexp::new("a?").compile();

  println("--Case 17--");
  UncompiledRegexp::new("(ABC)+").compile();

  println("--Case 18--");
  UncompiledRegexp::new("(A|B)*").compile();
  */

  println("OK");
  let args: ~[~str] = os::args();
  if (args.len()>1){
    println(args[1]);
    let contents = load(args[1]);
    static CMD_PROMPT_INIT: &'static str = "Enter your expression > ";
    print(CMD_PROMPT_INIT);
    let line = std::io::stdin().read_line();
    let mut exx = fmt!("%?", line);
    let mut exp=exx.slice(2,line.len()+2);
    println(fmt!("Rule given: %?",exp));
    let mut re = UncompiledRegexp::new(exp);
    let mut exx2 =fmt!("%?", contents);
    let mut exp2=exx2.slice(2,contents.len()+2);
    re.run(exp2);
  }
  else{
    static CMD_PROMPT_INIT: &'static str = "No file given...Enter your expression > ";
    print(CMD_PROMPT_INIT);
    let line = std::io::stdin().read_line();
    let mut exx = fmt!("%?", line);
    let mut exp=exx.slice(2,line.len()+2);
    println(fmt!("Rule given: %?",exp));
    let mut re = UncompiledRegexp::new(exp);
    static CMD_PROMPT: &'static str = "Input string > ";
    loop{
      print(CMD_PROMPT);
      //let mut argv: ~[~str] = line.split_iter(' ').filter(|&x| x != "")
      //                             .transform(|x| x.to_owned()).collect();
      let line = std::io::stdin().read_line();
      println(fmt!("Testing: %?", line));
      re.run(line);
    }
  }
  
}

fn load(filename: ~str) -> ~[~str] {
    let read_result: Result<@Reader, ~str>;
    read_result = io::file_reader(~path::Path(filename));
 
    if read_result.is_ok() {
        let file = read_result.unwrap();
        return file.read_lines();
    }
 
    println(fmt!("Error reading file: %?", read_result.unwrap_err()));
    return ~[];
  }