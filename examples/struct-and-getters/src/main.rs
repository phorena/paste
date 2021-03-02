use paste::paste;
use std::time::{Duration, Instant};
use std::thread::sleep;

// Returns the name of the current function this macro is called.
// For debugging.
// err_string = format!("{} [{}:{}])", function!(), file!(), line!());
macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}

// A macro for other macros to generate an enum
// create_enum! (enum_name (field1, field2, ...));
//
// pub enum enum_name {
//      field1 (u32),
//      field2 (u32),
//      ...
//}
macro_rules! create_enum {
    ($name:ident { $($field:ident),* }) => {
        paste! {
            #[derive (Debug)]
            #[allow(non_camel_case_types)]
            pub enum [<$name>] {
            $(
                $field (u32),
            )*
            }
        }
    }
}

// Generate a trait for transition functions
// make_trans_trait!(M1 {a, b, c});
//
// pub trait TransitionM1 {
//      fn trans_a (&mut self, state:State, input:Input) -> Output;
//      fn trans_b (&mut self, state:State, input:Input) -> Output;
//      fn trans_c (&mut self, state:State, input:Input) -> Output;
//
macro_rules! make_trans_trait {
    ($name:ident { $($field:ident),* }) => {
        paste! {
            pub trait [<Transition $name>] {
                $(
                    fn [<trans_ $field>](&mut self, 
                       state: State, input: Input) -> Output;
                )*
            }
        }
    }
}

create_enum!(State {a, b, c, d});
create_enum!(Input {a, b, c, d});
create_enum!(Output {a, b, c, d});

make_trans_trait!(M1 {a, b, c});

struct M1 {
    i:u32,
    state: State,
    input: Input,
    output: Output,
}

impl TransitionM1 for M1 {
    fn trans_a(&mut self, state: State, input: Input) -> Output {
        Output::a(11)
    }
    fn trans_b(&mut self, state: State, input: Input) -> Output {
        Output::a(51)
    }
    fn trans_c(&mut self, state: State, input: Input) -> Output {
        Output::a(61)
    }
}

// TODO write a macro to append the check_exp to every loop, while, for
// expression.
//
//      loop {
//      ...
//      }
//
//      loop { check_exp(exp1);
//      ...
//      }

// result<> enum for experation error
#[derive(Debug)]
enum LoopError {
    TimerExpired(String),
}

// struct for experation
pub struct LoopExpire {
    start:Instant,
    duration:Duration,
    err_string:String,
    // TODO
    // err: LoopError,
}

// is expired for the struct var
macro_rules! is_exp {
    ($var:ident) => {
        $var.start.elapsed() >= $var.duration
    }
}

// check for experation of var
// if expired:
//      if 1st time:
//          copy function_name, file, line number
//          reset the duration to 0
//      break out of the loop
// it works for inside loops too
macro_rules! check_exp {
    ($var:ident) => {
        if is_exp!($var) {
            if $var.err_string.is_empty() {
                $var.err_string = format!("{} [{}:{}])", function!(), file!(), line!());
                $var.duration = Duration::from_secs(0);
            }
            break;
        }
    }
}

// initialize the timer structure
// init_exparation(var, num_sec);
//
// let mut var = LoopExpire {
//            start: Instant::now(),
//            duration: Duration::from_secs($dur),
//            err_string:"".to_string(),
//        };
macro_rules! init_exparation {
    ($var:ident, $dur:expr) => {
        let mut $var = LoopExpire{
            start: Instant::now(),
            duration: Duration::from_secs($dur),
            err_string:"".to_string(),
        };
    }
}

// return the err enum with the err_string
macro_rules! err_exparation {
    ($var:ident) => {
        Err(LoopError::TimerExpired($var.err_string))
    }
}

fn loop_exp() -> Result<u64, LoopError> {
    init_exparation!(ex1, 1);
    let mut c:u64 = 0;
    loop { 
        check_exp!(ex1);
        c += 1;
        loop { 
        check_exp!(ex1);
            c += 1;
            if c > 100000000 {
                break;
            }
            // sleep(Duration::new(1, 0));
        }
        sleep(Duration::new(1, 0));
    };
    let mut d:u64 = 0;
    while d < 1000000 {
        check_exp!(ex1);
        d += 1;
        sleep(Duration::new(0, 20000));
    }

    if is_exp!(ex1) {
        err_exparation!(ex1)
    } else {
        Ok(c)
    }
}


fn main() {
    let start__ = Instant::now();
    let expired__ = Duration::from_secs(1);
    println!("start expired {:?} {:?}", start__, expired__);
    let mut s = S { a:1, b:2, c:3, };
    let a = Input_S::a(20);
    println!("{:#?}", a);
    let b = Input_S::b(20);
    println!("{:#?}", b);
    let c = Input_S::c(20);
    println!("{:#?}", c);
    println!("{}", s.a);
    println!("{}", s.get_c());
    println!("{}", s.trans_c(22));
    println!("{:#?}", s);
    let sa = M1_State::a(20);
    println!("{:#?}", sa);
    let b = M1_State::b(20);
    println!("{:#?}", b);
    let c = M1_State::c(20);
    println!("{:#?}", c);
    let d = M1_State::d(20);
    println!("{:#?}", d);
    let in_d = M1_Input::d(20);
    println!("{:#?}", in_d);

    sleep(Duration::new(1, 0));

    let duration = start__.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
    let now__ = Instant::now();
    println!("{:?}", now__.duration_since(start__));
    assert!(start__.elapsed() >= expired__);
    println!("{:?}", loop_exp());
    

    

//    let mut mm = M1{ i:1 };
//    println!("{:?}", mm.trans_a(sa, in_d));


}
macro_rules! make_enum {
    ($name:ident, $class:ident { $($field:ident),* }) => {
        paste! {
            #[derive (Debug)]
            #[allow(non_camel_case_types)]
            pub enum [<$name _ $class>] {
            $(
                $field (u32),
            )*
            }
        }
    }
}

make_enum!(M1, State {a, b, c, d});
make_enum!(M1, Input {a, b, c, d});
make_enum!(M1, Output {a, b, c, d});

/*
macro_rules! make_trans_trait {
    ($name:ident { $($field:ident),* }) => {
        // Generate a trait with transition functions
        paste! {
            pub trait [<Transition $name>] {
                $(
                    fn [<trans_ $field>](&mut self, 
                       state: [<$name _ State>],
                       input: [<$name _ Input>])
                       -> [<$name _ Output>];
                )*
            }
        }

    }
}
*/

macro_rules! make_a_struct_and_getters {
    ($name:ident { $($field:ident),* }) => {
        // Define a struct. This expands to:
        //
        //     pub struct S {
        //         a: String,
        //         b: String,
        //         c: String,
        //     }
        #[derive (Debug)]
        pub struct $name {
            $(
                $field: u32,
            )*
        }

        paste! {
            #[derive (Debug)]
            #[allow(non_camel_case_types)]
            enum [<Input_ $name>] {
            $(
                $field (u32),
            )*
            }
        }
        // Build an impl block with getters. This expands to:
        //
        //     impl S {
        //         pub fn get_a(&self) -> &str { &self.a }
        //         pub fn get_b(&self) -> &str { &self.b }
        //         pub fn get_c(&self) -> &str { &self.c }
        //     }
        paste! {
            impl $name {
                $(
                    pub fn [<get_ $field>](&self) -> u32 {
                        self.$field
                    }
                )*
            }
        }
        // Generate a trait with transition functions
        paste! {
            pub trait [<Transition $name>] {
                $(
                    fn [<trans_ $field>](&mut self, v: u32) -> u32;
                )*
            }
        }

    }
}
make_a_struct_and_getters!(S { a, b, c });
impl TransitionS for S {
    fn trans_a(&mut self, v: u32) -> u32 {
        self.a = v;
        v
    }
    fn trans_b(&mut self, v: u32) -> u32 {
        self.b = v;
        v
    }
    fn trans_c(&mut self, v: u32) -> u32 {
        self.c = v;
        v
    }
}

