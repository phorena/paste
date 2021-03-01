use paste::paste;

macro_rules! make_enum {
    ($name:ident, $class:ident { $($field:ident),* }) => {
        paste! {
            #[derive (Debug)]
            #[allow(non_camel_case_types)]
            enum [<$name _ $class>] {
            $(
                $field (u32),
            )*
            }
        }
    }
}

macro_rules! make_enum2 {
    ($name:ident { $($field:ident),* }) => {
        paste! {
            #[derive (Debug)]
            #[allow(non_camel_case_types)]
            enum [<$name>] {
            $(
                $field (u32),
            )*
            }
        }
    }
}

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

make_enum!(M1, State {a, b, c, d});
make_enum!(M1, Input {a, b, c, d});
make_enum!(M1, Output {a, b, c, d});

make_enum2!(State {a, b, c, d});
make_enum2!(Input {a, b, c, d});
make_enum2!(Output {a, b, c, d});

make_trans_trait!(M1 {a, b, c});

struct M1 {
    i:u32,
    state: State,
    input: Input,
    output: Output,
}

impl TransitionM1 for M1 {
    fn trans_a(&mut self, state: M1_State, input: M1_Input) -> M1_Output {
        M1_Output::a(11)
    }
    fn trans_b(&mut self, state: M1_State, input: M1_Input) -> M1_Output {
        M1_Output::a(51)
    }
    fn trans_c(&mut self, state: M1_State, input: M1_Input) -> M1_Output {
        M1_Output::a(61)
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

/*
fn call_some_getters(s: &S) -> bool {
    s.get_a() == s.get_b() && s.get_c().is_empty()
}
*/

fn main() {
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

    let mut mm:M1;
    // println!("{:?}", mm.trans_a(sa, in_d));


}
