use std::collections::HashMap;

use crate::ast::astree::*;
use crate::eval::eval::*;
use crate::svg_editor::svg_editor::*;
use crate::turtle::turtle::*;

extern crate lalrpop;

pub mod ast;
pub mod eval;
pub mod svg_editor;
pub mod turtle;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

fn build_declarations(func: Vec<Declaration>) -> HashMap<String, Declaration> {
    let mut res = HashMap::new();
    for f in func {
        res.insert(f.name.clone(), f);
    }
    res
}

fn save_as_svg(code: &str, file_name: &str, size: u32) -> Result<(), String> {
    let prog: Program = match parser::ProgramParser::new().parse(code) {
        Ok(l) => l,
        Err(e) => return Err(format!("Parse error {:?}", e)),
    };
    //println!("{:?}", logo);

    let mut turtle = Turtle::new();

    let effects = exec_program(
        prog.instructions,
        &mut HashMap::new(),
        &mut turtle,
        &build_declarations(prog.functions),
    )?;

    match to_svg(file_name, effects, size as i32) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Saving error {:?}", e)),
    }
}

fn main() -> Result<(), String> {
    let pic_folder = "obrazki/";
    let pic_size = 200;

    let star = "TO star
    repeat 5 [ fd 100 rt 144 ]
    END
    clearscreen
    star";

    let kwadraty = "
    TO square :length
    repeat 4 [ fd :length rt 90 ]
    END
    TO randomcolor
    setcolor (pick [ red orange yellow green blue violet ])
    END
    clearscreen
    repeat 36 [ randomcolor square (random 200) rt 10 ]";

    let label = "
    clearscreen window hideturtle
    repeat 144 [
    setlabelheight (repcount)
    penup
    fd (repcount) * (repcount) / 30
    label \"Logo
    bk (repcount) * (repcount) / 30
    pendown
    rt 10
    wait 5
    ]
    showturtle";

    let tree = "
    TO tree :size
        if :size < 5 [forward :size back :size stop]
        forward :size/3
        left 30 tree :size*2/3 right 30
        forward :size/6
        right 25 tree :size/2 left 25
        forward :size/3
        right 25 tree :size/2 left 25
        forward :size/6
        back :size
    END
    clearscreen
    pu bk 100 pd
    tree 150";

    let fern = "
    TO fern :size :sign
        if :size < 1 [ stop ]
        fd :size
        rt 70 * :sign fern :size * 0.5 :sign * (-1) lt 70 * :sign
        fd :size
        lt 70 * :sign fern :size * 0.5 :sign rt 70 * :sign
        rt 7 * :sign fern :size - 1 :sign lt 7 * :sign
        bk :size * 2
    END
    window clearscreen pu bk 150 pd
    fern 25 1
    ";

    save_as_svg(
        star,
        format!("{}gwiazdka.svg", pic_folder).as_str(),
        pic_size,
    )?;
    save_as_svg(
        kwadraty,
        format!("{}kwadraty.svg", pic_folder).as_str(),
        pic_size,
    )?;
    save_as_svg(label, format!("{}label.svg", pic_folder).as_str(), pic_size)?;
    save_as_svg(tree, format!("{}tree.svg", pic_folder).as_str(), pic_size)?;
    save_as_svg(fern, format!("{}fern.svg", pic_folder).as_str(), pic_size)?;
    Ok(())
}
