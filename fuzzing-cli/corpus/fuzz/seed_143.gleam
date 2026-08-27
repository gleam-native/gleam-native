pub const golden_value: Int = 0

pub type V0 {
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(acc: String) -> Int {
42
}

fn f1(v2: V0, v3: V0, v4: V0) -> Bool {
{
    2.0
  } <. {
    {
      10.0
    } -. {
      {
        100.0
      } *. {
        3.14
      }
    }
  }
}

pub fn main() {
  let constructor = case f1(Cv1, Cv1, Cv1) {
    inner -> {
      0.5
    } *. {
      2.0
    }
    _ -> {
      let pair = []
      let prototype = pair
      0.25
    }
    item -> {
      let prototype = 0.5
      1.5
    }
  }
  echo {
    {
      0.0
    } *. constructor
  } >. constructor
}
