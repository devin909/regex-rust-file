use state::*;

// instruction opcodes
//
// more opcodes are probably required, but for now:
//
// Literal      = check if the input matches a literal
// Range        = check if the input is within a range
// Match        = the input matches!
// Jump         = goto a point in the stack
// CaptureStart = start capturing the input
// CaptureEnd   = end capturing the input
// Split        = start a new thread with one jumping to the
//                first uint, and the next jumping to the
//                second.
// InstNoop     = placeholder in most cases

#[deriving(Clone)]
pub enum InstOpCode {
  InstLiteral(char),
  InstRange(char, char),
  InstMatch,
  InstJump(uint),
  InstCaptureStart,
  InstCaptureEnd,
  InstSplit(uint, uint),
  InstNoop
}

#[deriving(Clone)]
pub struct Instruction {
  op: InstOpCode 
}

impl Instruction {
  pub fn new(op: InstOpCode) -> Instruction {
    Instruction { op: op }
  }
}

impl ToStr for Instruction {
  fn to_str(&self) -> ~str {
    match self.op {
      InstLiteral(c)    => fmt!("InstLiteral %c", c), 
      InstRange(s, e)   => fmt!("InstRange %c-%c", s, e),
      InstMatch         => ~"InstMatch", 
      InstJump(i)       => fmt!("InstJump %u", i),
      InstCaptureStart  => ~"InstCaptureStart",
      InstCaptureEnd    => ~"InstCaptureEnd",
      InstSplit(l, r)   => fmt!("InstSplit %u | %u", l, r),
      InstNoop          => ~"InstNoop"
    }
  }
}

fn compile_literal(lit: &Literal, stack: &mut ~[Instruction]) {
  for c in lit.value.iter() {
    stack.push(Instruction::new(InstLiteral(c)));
  }
}
fn compile_charclass(cc: &CharClass, stack: &mut ~[Instruction]) {
  let mut ssize = stack.len();
  let mut rlen = cc.ranges.len();
  let rsize = ssize + rlen * 3;

  for &(start, end) in cc.ranges.iter() {
    if (rlen >= 2) {
      let split = Instruction::new(InstSplit(ssize + 1, ssize + 3));
      stack.push(split);

      ssize += 3;
      rlen  -= 1;
    }

    if (start == end) {
      stack.push(Instruction::new(InstLiteral(start)));
    } else {
      stack.push(Instruction::new(InstRange(start, end)));
    }

    stack.push(Instruction::new(InstJump(rsize - 1)));
  }
}

pub fn compile_recursive(re: &Regexp, stack: &mut ~[Instruction]) {
  _compile_recursive(re, stack);
  stack.push(Instruction::new(InstMatch));

  //debug_stack(stack);
}

fn _compile_recursive(re: &Regexp, stack: &mut ~[Instruction]) {
  // recurse on a sub expression if type is Expression,
  // otherwise just compile
  macro_rules! recurse(
    ($re: expr) => (
      {
        match $re {
          &Some(~ParseStack::Expression(ref x)) => {
            _compile_recursive(x, stack);
          }
          &Some(~ParseStack::Literal(ref lit)) => {
            compile_literal(lit, stack);
          }
          &Some(~ParseStack::CharClass(ref cc)) => {
            compile_charclass(cc, stack);
          }
          _ => { } // unreachable
        }
      }
    );
  )

  // insert a InstNoop...to be replaced later
  macro_rules! placeholder(
    () => (
      stack.push(Instruction::new(InstNoop))
    );
  )

  match &re.op {
    // this should correspond with the case 
    // of the input being only a string (i.e 'abc')
    // or a char class (i.e '[a-zA-Z]')
    &OpNoop => {
      match re.state0 {
        Some(~ParseStack::Literal(ref lit)) => {
          compile_literal(lit, stack);
        }
        Some(~ParseStack::CharClass(ref cc)) => {
          compile_charclass(cc, stack);
        }
        _ => { } // unreachable
      };
    }
    // compile to:
    // ...
    //      Split(L1, L2)
    // L1:  (state0)
    //      Jump(L3)
    // L2:  (state1)
    // L3:  ...
    &OpAlternation => {
      let ptr_split = stack.len();
      placeholder!();
      recurse!(&re.state0);

      let ptr_jmp = stack.len();
      placeholder!();
      recurse!(&re.state1);
      
      let split = Instruction::new(InstSplit(ptr_split + 1, ptr_jmp + 1));
      let jmp = Instruction::new(InstJump(stack.len()));

      stack[ptr_split] = split; 
      stack[ptr_jmp] = jmp; 
    }
    // compile to:
    // ...
    // (state0)
    // (state1)
    // ...
    &OpConcatenation => {
      recurse!(&re.state0);
      recurse!(&re.state1);
    }
    // compile to:
    // ...
    // CaptureStart
    // (state0)
    // CaptureEnd
    &OpCapture => {
      if (re.hasFlag(ParseFlags::NoCapture)) {
        recurse!(&re.state0);
      } else {
        stack.push(Instruction::new(InstCaptureStart));
        recurse!(&re.state0);
        stack.push(Instruction::new(InstCaptureEnd));
      }
    }
    // compile to:
    // ...
    // L1: Split(L2, L3) | Split(L3, L2) (NonGreedy)
    // L2: (state0)
    //     Jump(L1)
    // L3: ...
    // ...
    &OpKleine => {
      let ptr_split = stack.len();
      let nongreedy = re.hasFlag(ParseFlags::NonGreedy);
      placeholder!();

      recurse!(&re.state0);
      let jmp = Instruction::new(InstJump(ptr_split));
      stack.push(jmp);

      let split = {
        if (nongreedy) { 
          Instruction::new(InstSplit(stack.len(), ptr_split + 1))
        } else {
          Instruction::new(InstSplit(ptr_split + 1, stack.len()))
        }
      };
      stack[ptr_split] = split;
    }
    // compile to:
    // ...
    // L1: (state0)
    //     Split(L1, L2) | Split(L2, L1) (NonGreedy)
    // L2: ...
    // ...
    &OpOneOrMore => {
      let ptr_inst = stack.len();
      let nongreedy = re.hasFlag(ParseFlags::NonGreedy);
      recurse!(&re.state0);

      let split = {
        if (nongreedy) {
          Instruction::new(InstSplit(stack.len() + 1, ptr_inst))
        } else {
          Instruction::new(InstSplit(ptr_inst, stack.len() + 1))
        }
      };
      stack.push(split);
    }
    // compile to:
    // ...
    //     Split(L1, L2) | Split(L2, L1) (NonGreedy)
    // L1: (state0)
    // L2: ... 
    &OpZeroOrOne => {
      let ptr_split = stack.len();
      let nongreedy = re.hasFlag(ParseFlags::NonGreedy);
      placeholder!();

      recurse!(&re.state0);

      let split = {
        if (nongreedy) {
          Instruction::new(InstSplit(stack.len(), ptr_split + 1))
        } else {
          Instruction::new(InstSplit(ptr_split + 1, stack.len()))
        }
      };
      stack[ptr_split] = split;
    }
    _ => { }
  }
}

fn debug_stack(stack: &mut ~[Instruction]) {
  let mut count: uint = 0;

  println("--COMPILE STACK--");
  for e in stack.iter() {
    println(fmt!("%u: %s", count, e.to_str()));
    count += 1;
  }
}
