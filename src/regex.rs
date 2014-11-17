use std::rc::Rc;
use util::BinSetu8;

#[deriving(Clone)]
pub enum Regex {
    // binary operators
    Or(Box<Regex>, Box<Regex>),
    Cat(Box<Regex>, Box<Regex>),

    // unary operators
    Maybe(Box<Regex>),
    Closure(Box<Regex>),

    // constants
    Class(Box<BinSetu8>),
    NotClass(Box<BinSetu8>),
    Var(Rc<Regex>),
    Char(u8),
    Any
}

pub fn string(string: &str) -> Option<Box<Regex>> {
    let mut it = string.bytes();
    let mut reg = box Char(match it.next() {
        Some(ch) => ch,
        None => return None
    });

    for ch in it {
        reg = box Cat(reg, box Char(ch));
    }

    Some(reg)
}

impl Regex {
    #[allow(dead_code)]
    // prints the AST for debugging purposes
    pub fn show(&self, span: &str) {
        match self {
            &Or(ref l, ref r) => {
                println!("{:s} Or of: ", span);
                l.show(format!("  {:s}", span).as_slice());
                r.show(format!("  {:s}", span).as_slice());
            }

            &Cat(ref l, ref r) => {
                println!("{:s} Cat of: ", span);
                l.show(format!("  {:s}", span).as_slice());
                r.show(format!("  {:s}", span).as_slice());
            }

            &Maybe(ref reg) => {
                println!("{:s} Optionnally the regex:", span);
                reg.show(span);
            }

            &Closure(ref reg) => {
                println!("{:s} The eclosure of", span);
                reg.show(format!("  {:s}", span).as_slice())
            }

            &Var(ref reg) => {
                (**reg).show(span);
            }

            &Char(ref c) => println!("{:s} The char {:c}", span, *c as char),
            &Any => println!("Anything"),
            _ => ()
        }
    }
}