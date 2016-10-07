use std::iter::repeat;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum TableElem {
    Empty,
    LStr(String),
    RStr(String),
    I32(i32),
    U32(u32),
    F32(f32),
    F64(f64)
}

impl TableElem {
    fn to_string(&self) -> String {
        match *self {
            TableElem::I32(i) => i.to_string(),
            TableElem::U32(u) => u.to_string(),
            TableElem::Empty => "".to_string(),
            TableElem::LStr(ref s) => s.to_string(),
            TableElem::RStr(ref s) => s.to_string(),
            TableElem::F32(r) => r.to_string(),
            TableElem::F64(r) => r.to_string()
        }
    }
}

pub fn left(v: &str) -> TableElem { 
    TableElem::LStr(v.to_string())
}

pub fn right(v: &str) -> TableElem { 
    TableElem::RStr(v.to_string())
}


pub struct Table {
    rows : Vec<Vec<TableElem>>
}

impl Table {
    pub fn new(rows :usize, cols :usize) -> Table {
        Table { 
            rows : repeat(repeat(TableElem::Empty).take(cols).collect::<_>())
                .take(rows).collect::<_>()
        }
    }

    pub fn set(&mut self, r :usize, c :usize, v:TableElem) {
        self.rows[r][c] = v
    }

    pub fn get<'a>(&'a self, r :usize, c :usize) -> &'a TableElem {
        &self.rows[r][c]
    }

    pub fn print(&self, caption: &str) {
        if self.rows.len() == 0 { return }
        
        let min_width = 4; 
        let mut width_tbl : Vec<usize> = 
            repeat(min_width).take(self.rows[0].len()).collect();
        let width = &mut width_tbl;
        
        for row in self.rows.iter() {
            for (c, elem) in row.iter().enumerate() {
                let w = elem.to_string().len();
                width[c] = if w > width[c] { w } else { width[c] }
            }
        }
        
        let tot_width = width.iter().fold(0, |a,&b| a + b + 1);
        let line_width = tot_width - caption.len() - 2;
        let left : String = repeat("=").take(line_width/2).collect();
        let rght : String = repeat("=").take((line_width+1)/2).collect();
        println!("{} {} {}", left, caption, rght);
                
        for line in self.rows.iter() {
            for (c, elem) in line.iter().enumerate() {
                match *elem {
                    TableElem::I32(i)      => print!("{:w$} ", i,           w=width[c]),
                    TableElem::U32(u)      => print!("{:w$} ", u,           w=width[c]),
                    TableElem::F32(f)      => print!("{:w$} ", f,           w=width[c]),
                    TableElem::F64(f)      => print!("{:w$} ", f,           w=width[c]),
                    TableElem::Empty       => print!("{:>w$} ", "-",        w=width[c]),
                    TableElem::LStr(ref s) => print!("{:<w$} ", s.to_string(), w=width[c]),
                    TableElem::RStr(ref s) => print!("{:>w$} ", s.to_string(), w=width[c])
                }
            }
            print!("\n");
        }
        println!("{}", repeat("=").take(tot_width).collect::<String>());
        print!("\n");
    }

    pub fn print_tex(&self, caption: &str) {
        if self.rows.len() == 0 { return }
        
        let min_width = 0; 
        let mut width_tbl = 
            repeat(min_width).take(self.rows[0].len()).collect::<Vec<usize>>();
        let width = &mut width_tbl;
        
        for row in self.rows.iter() {
            for (c, elem) in row.iter().enumerate() {
                let w = elem.to_string().len();
                width[c] = if w > width[c] { w } else { width[c] }
            }
        }

        struct TeX { ind: usize };

        impl TeX { 
            fn indent(&mut self) {
                print!("{}", repeat(' ').take(self.ind).collect::<String>());
            }

            fn cmd(&mut self, what: &str) {
                self.indent();
                println!("\\{}", what);
            }
            fn cmd_opt(&mut self, what: &str, opt: &str) {
                self.indent();
                println!("\\{}{{{}}}", what, opt);
            }
            fn cmd_opt2(&mut self, what: &str, opt: &str, opt2: &str) {
                self.indent();
                println!("\\{}{{{}}}{{{}}}", what, opt, opt2);
            }

            fn begin(&mut self, what: &str) { 
                self.cmd_opt("begin", what); 
                self.ind += 2 
            }
            fn begin_opt(&mut self, what: &str, opt: &str) {
                self.cmd_opt2("begin", what, opt);
                self.ind += 2
            }
            fn end(&mut self, what: &str) { 
                self.ind -= 2 ;
                self.cmd_opt("end", what);
            }
        }
        
        let mut tex = TeX { ind : 0 };

        tex.begin("figure");
        tex.begin("center");
        tex.begin_opt("tabular", &(0..self.rows[0].len()).map({|_| "c"}).collect::<String>());
        
        for (r, line) in self.rows.iter().enumerate() {
            if r == 0 { tex.cmd("toprule") }
            if r == 1 { tex.cmd("midrule") }

            tex.indent();
            
            for (c, elem) in line.iter().enumerate() {
                match *elem {
                    TableElem::I32(i)      => print!("{:w$}", i,           w=width[c]),
                    TableElem::U32(u)      => print!("{:w$}", u,           w=width[c]),
                    TableElem::F32(f)      => print!("{:w$}", f,           w=width[c]),
                    TableElem::F64(f)      => print!("{:w$}", f,           w=width[c]),
                    TableElem::Empty       => print!("{:>w$}", "\\cdots",        w=width[c]),
                    TableElem::LStr(ref s) => print!("{:<w$}", s.to_string(), w=width[c]),
                    TableElem::RStr(ref s) => print!("{:>w$}", s.to_string(), w=width[c])
                }
                
                if c < line.len() - 1 {
                    print!(" & ")
                } else {
                    print!(" \\\\\n")
                }
            }        
        }

        tex.cmd("bottomrule");
        tex.end("tabular");
        tex.end("centering");
        tex.cmd_opt("caption", caption);
        tex.end("figure");
        
        print!("\n");
    }
}
