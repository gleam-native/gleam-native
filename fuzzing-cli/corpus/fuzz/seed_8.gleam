pub type Symbol {
  Record
}

pub type V0 {
  Some(value: String, inner: String)
  Cv1
  Error
}

fn f0(constructor: Int, length: Float) -> List(Int) {
[]
}

fn extends(v2: Float, constructor: Float) -> Bool {
True
}

pub fn main() {
  echo {
    0.25
  } /. {
    2.0
  }
  echo case 42 |> f0({
      let value = [7]
      let delete = 2.0
      3.14
    }) {
    [b] -> case [100] {
      [] -> "abc"
      [b, ..rest] -> "res"
      [_, _, ..] -> fn(v3) { "b" }(False)
      v4 -> ""
    }
    [] -> ""
    _ -> "bc"
  }
  echo []
  echo {
    let n = {
      {
        0.5
      } -. {
        0.1
      }
    } /. {
      3.14
    }
    True
  }
}
