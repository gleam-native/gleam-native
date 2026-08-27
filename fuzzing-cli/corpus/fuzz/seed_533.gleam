pub const limit_value: String = ""
pub const pi_value: String = ""
pub const seed_value: Int = 7

pub type V0 {
  Ok(value: String, inner: Float)
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(self_: Int, v: String, this_: Bool) -> String {
{
    {
      v <> "res"
    } <> {
      {
        let self_ = [100]
        v
      }
    }
  } <> {
    v <> v
  }
}

fn f1(item: String, z: Int) -> String {
{
    {
      let z = z
      fn(v2) { item }(True)
    }
  } <> "abc"
}

fn extends(v3: Int, delete: Float, self_: Bool) -> String {
"x"
}

pub fn main() {
  echo {
    {
      seed_value - seed_value
    } - {
      seed_value - seed_value
    }
  } - seed_value
}
