pub mod svg_editor {
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;
    use std::f64::consts::FRAC_PI_2;
    use crate::turtle::turtle::*;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Commands {
        Cls,
        Stop,
        Segment(f64, f64, f64, f64, Color), 
        Label(String, f64, f64, u32, f64, Color), 
        Wait(f64),
    }

    fn svg_line(e: Commands) -> String {
        match e {
            Commands::Segment(x1, y1, x2, y2, color) => 
                format!("<line x1='{}' y1='{}' x2='{}' y2='{}' stroke='{}' />", x1, y1, x2, y2, color).to_string(),
            Commands::Label(text, x, y, size, rotate, color) => 
                format!("<text style='font: {}px serif; fill: {};transform:translate({}px, {}px) rotate({}rad)'>{}</text>",
                        size, color, x, y, rotate - FRAC_PI_2, text).to_string(),
            _ => String::new()
        }
    }

    pub fn get_commands(commands: Vec<Commands>) -> Vec<String> {
    commands
        .into_iter().rev()
        .take_while(|x| *x != Commands::Cls) // bierzemy od tylu do momentu ostatniego czyszczenia
        .collect::<Vec<_>>().into_iter()
        .rev()
        .map(|x| svg_line(x))
        .filter(|x| x != "")
        .collect()
    }

    pub fn to_svg<P: AsRef<Path>>(path: P, commands: Vec<Commands>, size: i32) -> std::io::Result<()>{
        let mut lines = vec![
            format!("<svg viewBox='{} {} {} {}' xmlns='http://www.w3.org/2000/svg'>", -size / 2, -size / 2, size, size).to_string()]; // naglowek
        lines.append(&mut get_commands(commands));
        lines.push("</svg>".to_string()); // koniec pliku

        let mut file = File::create(path)?;
        file.write_all(lines.join("\n").as_bytes())?;
        Ok(())
    }
}