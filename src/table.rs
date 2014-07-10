#[deriving(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TableElem {
    Empty,
    LStr(String),
    RStr(String),
    Int(int),
    UInt(uint)
}

impl TableElem {
    fn to_string(&self) -> String {
        match *self {
            Int(i) => i.to_string(),
            UInt(u) => u.to_string(),
            Empty => "".to_string(),
            LStr(ref s) => s.to_string(),
            RStr(ref s) => s.to_string()
        }
    }
}

pub struct Table {
    rows : Vec<Vec<TableElem>>
}

impl Table {
    pub fn new(rows:uint, cols:uint) -> Table {
        Table { rows : Vec::from_fn(rows, |_| { Vec::from_fn(cols, |_| Empty) }) }
    }

    pub fn set(&mut self, r:uint, c:uint, v:TableElem) {
        let row = self.rows.get_mut(r);
        *row.get_mut(c) = v
    }

    pub fn get<'a>(&'a self, r:uint, c:uint) -> &'a TableElem {
        self.rows.get(r).get(c)
    }

    pub fn print(&self, caption: &str) {
        if self.rows.len() == 0 { return }
        
        let min_width = 4u; 
        let mut width_tbl = Vec::from_elem(self.rows.get(0).len(), min_width);
        let width = width_tbl.as_mut_slice();
        
        for row in self.rows.iter() {
            for (c, elem) in row.iter().enumerate() {
                let w = elem.to_string().len();
                width[c] = if w > width[c] { w } else { width[c] }
            }
        }
        
        let tot_width = width.iter().fold(0, |a,&b| a + b);
        let line_width = tot_width - caption.len() - 2;
        let left = "=".repeat(line_width/2);
        let rght = "=".repeat((line_width+1)/2);
        println!("{} {} {}", left, caption, rght);
                
        for line in self.rows.iter() {
            for (c, elem) in line.iter().enumerate() {
                match *elem {
                    Int(i)     => print!("{:w$i}", i,           w=width[c]),
                    UInt(u)    => print!("{:w$u}", u,           w=width[c]),
                    Empty      => print!("{:>w$s}", "-",        w=width[c]),
                    LStr(ref s) => print!("{:<w$s}", s.to_string(), w=width[c]),
                    RStr(ref s) => print!("{:>w$s}", s.to_string(), w=width[c])
                }
            }
            print!("\n");
        }
        println!("{}", "=".repeat(tot_width));
        print!("\n");
    }

    pub fn print_tex(&self, caption: &str) {
        if self.rows.len() == 0 { return }
        
        let min_width = 0u; 
        let mut width_tbl = Vec::from_elem(self.rows.get(0).len(), min_width);
        let width = width_tbl.as_mut_slice();
        
        for row in self.rows.iter() {
            for (c, elem) in row.iter().enumerate() {
                let w = elem.to_string().len();
                width[c] = if w > width[c] { w } else { width[c] }
            }
        }

        struct TeX { ind: uint };

        impl TeX { 
            fn indent(&mut self) {
                print!("{}", " ".repeat(self.ind));
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
        tex.begin_opt("tabular", "c".repeat(self.rows.get(0).len()).as_slice());
        
        for (r, line) in self.rows.iter().enumerate() {
            if r == 0 { tex.cmd("toprule") }
            if r == 1 { tex.cmd("midrule") }

            tex.indent();
            
            for (c, elem) in line.iter().enumerate() {
                match *elem {
                    Int(i)     => print!("{:w$i}", i,           w=width[c]),
                    UInt(u)    => print!("{:w$u}", u,           w=width[c]),
                    Empty      => print!("{:>w$s}", "\\cdots",        w=width[c]),
                    LStr(ref s) => print!("{:<w$s}", s.to_string(), w=width[c]),
                    RStr(ref s) => print!("{:>w$s}", s.to_string(), w=width[c])
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
