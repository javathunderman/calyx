use crate::errors::Error;
use sexpy::Sexpy;
use std::fs;
use std::path::PathBuf;

// Abstract Syntax Tree for Futil. See link below for the grammar
// https://github.com/cucapra/futil/blob/master/grammar.md

pub type Id = String;

pub fn parse_file(file: &PathBuf) -> Result<Namespace, Error> {
    let content = &fs::read(file)?;
    let string_content = std::str::from_utf8(content)?;
    match Namespace::parse(string_content) {
        Ok(ns) => Ok(ns),
        Err(msg) => Err(Error::ParseError(msg)),
    }
}

#[derive(Clone, Debug, Hash, Sexpy)]
#[sexpy(head = "define/namespace")]
pub struct Namespace {
    pub name: String,
    pub components: Vec<Component>,
}

#[derive(Clone, Debug, Hash, Sexpy)]
#[sexpy(head = "define/component")]
pub struct Component {
    pub name: String,
    #[sexpy(surround)]
    pub inputs: Vec<Portdef>,
    #[sexpy(surround)]
    pub outputs: Vec<Portdef>,
    #[sexpy(surround)]
    pub structure: Vec<Structure>,
    pub control: Control,
}

#[derive(Clone, Debug, Hash, Sexpy)]
#[sexpy(head = "port")]
pub struct Portdef {
    pub name: String,
    pub width: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Sexpy)]
#[sexpy(head = "@")]
pub enum Port {
    Comp {
        component: Id,
        port: String,
    },
    #[sexpy(head = "this")]
    This {
        port: String,
    },
}

#[derive(Clone, Debug, Hash, Sexpy, PartialEq)]
#[sexpy(nohead)]
pub struct Compinst {
    pub name: String,
    pub params: Vec<i64>,
}

// ===================================
// Data definitions for Structure
// ===================================

#[derive(Clone, Debug, Hash, Sexpy, PartialEq)]
#[sexpy(head = "new", nosurround)]
pub struct Decl {
    pub name: Id,
    pub component: String,
}

#[derive(Clone, Debug, Hash, Sexpy, PartialEq)]
#[sexpy(head = "new-std", nosurround)]
pub struct Std {
    pub name: Id,
    pub instance: Compinst,
}

#[derive(Clone, Debug, Hash, Sexpy, PartialEq)]
#[sexpy(head = "->", nosurround)]
pub struct Wire {
    pub src: Port,
    pub dest: Port,
}

#[derive(Clone, Debug, Hash, Sexpy, PartialEq)]
#[sexpy(nohead)]
pub enum Structure {
    Decl { data: Decl },
    Std { data: Std },
    Wire { data: Wire },
}

#[allow(unused)]
impl Structure {
    pub fn decl(name: Id, component: String) -> Structure {
        Structure::Decl {
            data: Decl { name, component },
        }
    }

    pub fn std(name: Id, instance: Compinst) -> Structure {
        Structure::Std {
            data: Std { name, instance },
        }
    }

    pub fn wire(src: Port, dest: Port) -> Structure {
        Structure::Wire {
            data: Wire { src, dest },
        }
    }
}

// ===================================
// Data definitions for Control Ast
// ===================================
// Need Boxes for recursive data structure
// Cannot have recursive data structure without
// indirection

#[derive(Debug, Clone, Hash, Sexpy)]
#[sexpy(nosurround)]
pub struct Seq {
    pub stmts: Vec<Control>,
}

#[derive(Debug, Clone, Hash, Sexpy)]
#[sexpy(nosurround)]
pub struct Par {
    pub stmts: Vec<Control>,
}

#[derive(Debug, Clone, Hash, Sexpy)]
#[sexpy(nosurround)]
pub struct If {
    pub port: Port,
    #[sexpy(surround)]
    pub cond: Vec<String>,
    pub tbranch: Box<Control>,
    pub fbranch: Box<Control>,
}

#[derive(Debug, Clone, Hash, Sexpy)]
#[sexpy(nosurround)]
pub struct Ifen {
    pub port: Port,
    #[sexpy(surround)]
    pub cond: Vec<String>,
    pub tbranch: Box<Control>,
    pub fbranch: Box<Control>,
}

#[derive(Debug, Clone, Hash, Sexpy)]
#[sexpy(nosurround)]
pub struct While {
    pub port: Port,
    #[sexpy(surround)]
    pub cond: Vec<String>,
    pub body: Box<Control>,
}

#[derive(Debug, Clone, Hash, Sexpy)]
#[sexpy(nosurround)]
pub struct Print {
    pub var: String,
}

#[derive(Debug, Clone, Hash, Sexpy)]
#[sexpy(nosurround)]
pub struct Enable {
    pub comps: Vec<String>,
}

#[derive(Debug, Clone, Hash, Sexpy)]
#[sexpy(nosurround)]
pub struct Disable {
    pub comps: Vec<String>,
}

#[derive(Debug, Clone, Hash, Sexpy)]
#[sexpy(nosurround)]
pub struct Empty {}

#[derive(Debug, Clone, Hash, Sexpy)]
#[sexpy(nohead)]
pub enum Control {
    Seq { data: Seq },
    Par { data: Par },
    If { data: If },
    Ifen { data: Ifen },
    While { data: While },
    Print { data: Print },
    Enable { data: Enable },
    Disable { data: Disable },
    Empty { data: Empty },
}

#[allow(unused)]
impl Control {
    pub fn seq(stmts: Vec<Control>) -> Control {
        Control::Seq {
            data: Seq { stmts },
        }
    }

    pub fn par(stmts: Vec<Control>) -> Control {
        Control::Par {
            data: Par { stmts },
        }
    }

    // pub fn c_if(cond: Port, tbranch: Control, fbranch: Control) -> Control {
    //     Control::If {
    //         data: If {
    //             cond,
    //             tbranch: Box::new(tbranch),
    //             fbranch: Box::new(fbranch),
    //         },
    //     }
    // }

    // pub fn ifen(cond: Port, tbranch: Control, fbranch: Control) -> Control {
    //     Control::Ifen {
    //         data: Ifen {
    //             cond,
    //             tbranch: Box::new(tbranch),
    //             fbranch: Box::new(fbranch),
    //         },
    //     }
    // }

    // pub fn c_while(cond: Port, body: Control) -> Control {
    //     Control::While {
    //         data: While {
    //             cond,
    //             body: Box::new(body),
    //         },
    //     }
    // }

    pub fn print(var: String) -> Control {
        Control::Print {
            data: Print { var },
        }
    }

    pub fn enable(comps: Vec<String>) -> Control {
        Control::Enable {
            data: Enable { comps },
        }
    }

    pub fn disable(comps: Vec<String>) -> Control {
        Control::Disable {
            data: Disable { comps },
        }
    }

    pub fn empty() -> Control {
        Control::Empty { data: Empty {} }
    }
}
