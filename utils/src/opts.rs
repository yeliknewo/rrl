use getopts::Options;
use std::env;

const H: &'static str = "h";

const R: &'static str = "r";
const W: &'static str = "w";
const G: &'static str = "g";
const T: &'static str = "t";

pub struct Opter {
    r: Option<String>,
    w: Option<String>,
    g: Option<String>,
    t: Option<String>,
}

impl Opter {
    pub fn new() -> Opter {
        let args: Vec<String> = env::args().collect();
        let program = args[0].clone();

        let mut opts = Options::new();

        opts.optopt(R, "", "set file to load for starting network", "READ");
        opts.optopt(W, "", "set file to save network", "WRITE");
        opts.optopt(G, "", "set graphics mode", "GRAPHICS");
        opts.optopt(T, "", "set fixed delta time", "DELTA_TIME");
        opts.optflag(H, "help", "print this help menu");

        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(err) => panic!("Opter Error while parsing: {:?}", err),
        };

        if matches.opt_present(H) {
            Opter::print_usage(&program, opts);
        }

        let r = matches.opt_str(R);

        let w = matches.opt_str(W);

        let g = matches.opt_str(G);

        let t = matches.opt_str(T);

        Opter {
            r: r,
            w: w,
            g: g,
            t: t,
        }
    }

    pub fn get_r(&self) -> Option<&String> {
        self.r.as_ref()
    }

    pub fn get_w(&self) -> Option<&String> {
        self.w.as_ref()
    }

    pub fn get_g(&self) -> Option<&String> {
        self.g.as_ref()
    }

    pub fn get_t(&self) -> Option<&String> {
        self.t.as_ref()
    }

    fn print_usage(program: &str, opts: Options) {
        let brief = format!("Usage: {} FILE [options]", program);
        print!("{}", opts.usage(&brief));
    }
}
