struct Counter{
    count:u32,
}

trait Iterator {
    fn next(&mut self) -> Option<u32>;
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }
}

impl Iterator for Counter {
    fn next(&mut self) -> Option<u32> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new();

    // while let Some(number) = counter.next() {
    //     println!("{}", number);
    // } 
    loop{
        match counter.next(){
            Some(number) => println!("{}",number),
            None => break,
        }
    }
}
