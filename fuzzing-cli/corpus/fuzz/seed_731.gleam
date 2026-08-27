pub const limit_value: String = "abc"
pub const seed_value: Int = 2
pub const golden_value: Int = 100

pub type V0 {
  Error(value: String, inner: List(Int))
  Cv1(Float)
}

pub type V2 {
  Cv3(value: Int, inner: List(Int))
  Ok(Bool, value: Float)
  Cv4(value: Int, inner: List(Int))
}

fn f0(v5: Int, constructor: Int) -> Int {
{
    let constructor = {
      1.0
    } +. {
      fn(v6) { 3.14 }("res")
    }
    v5
  }
}

fn f1(n: Int, v7: Int) -> Float {
{
    {
      let acc = "constructor"
      let v7 = "abc" <> acc
      100.0
    }
  } /. {
    10.0
  }
}

pub fn main() {
  let constructor = case seed_value {
    constructor -> {
      let constructor = limit_value
      let rest = 5
      [5]
    }
    0 -> [42]
  }
  let golden_value = [7, 10]
  echo {
    0.1
  } +. {
    case {
        let this_ = limit_value
        limit_value
      }, 4 |> f0({
        let constructor = 100
        4
      }) {
      "res" <> _, 0 -> 2.0
      "ab" <> rest, 9 if rest != "x" -> {
        100.0
      } +. {
        3.14
      }
      "a", 1 -> {
        let v = 1.5
        let value = "x"
        v
      }
      _, _ -> {
        let limit_value = 0.0
        let default = "a"
        limit_value
      }
    }
  }
  echo 100 - {
    {
      seed_value % 4
    } - {
      {
        let constructor = [100]
        let s = True
        seed_value
      }
    }
  }
  echo seed_value
  echo {
    {
      let acc = golden_value
      let constructor = 1
      {
        0.5
      } -. {
        2.0
      }
    }
  } /. {
    0.5
  }
}
