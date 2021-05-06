trait Tweet {
  fn tweet(&self);
  fn tweet_twice(&self) {
    self.tweet();
    self.tweet();
  }

  fn shout(&self) {
    println!("Uoooooooooooohhh!!!!!!!!!");
  }
}

struct Dove;
struct Duck;

impl Tweet for Dove {
  fn tweet(&self) {
    println!("Cool!");
  }
}

fn main() {
  let dove = Dove {};

  
}