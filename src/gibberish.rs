/**
 * This is just a gibberish Rust file
 * that will allow us to learn about Rust.
 */

struct Closure<F> {
    data: (u8, u16),
    func: F,
}

impl<F> Closure<F>
where
    F: Fn(&(u8, u16)) -> &u8,
{
    fn call(&self) -> &u8 {
        (self.func)(&self.data)
    }
}

struct Testing {
    data: &dyn Fn(u8, u16) -> u8,
}

impl Testing {
    pub fn call(&self, a: u8, b: u16) -> u8 {
        &self.data((a, b))
    }
}

fn do_it(data: &(u8, u16)) -> &u8 {
    &data.0
}

fn main() {
    let clo = Closure {
        data: (0, 1),
        func: do_it,
    };
    let o_clo = Closure {
        data: (0, 1),
        func: do_it,
    };
    println!("{}", clo.call());
}
