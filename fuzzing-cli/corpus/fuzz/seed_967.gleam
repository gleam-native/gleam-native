pub const euler_value: String = "constructor"
pub const limit_value: Int = 1

pub type Record {
  Record
  Cv0(String)
  Cv1(value: String, inner: Int)
}

pub type V2 {
  Cv3
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn constructor(z: Int, l: Float, v4: List(Int)) -> String {
{
    let v4 = {
      z - z
    } == {
      {
        let l = v4
        let l = 42
        7
      }
    }
    let constructor = 7
    case "a" {
      "data" | "b" -> "x"
      _ | "abc" -> fn(v5, v6) { v5 }("constructor", 10.0)
    }
  }
}

fn f1(s: Float, v7: Record, m: Bool) -> Int {
walk({
    let v7 = {
      let s = 7
      0.1
    }
    [5]
  }, 0 - {
    2 % 1
  })
}

fn static(s: List(Int), v8: Bool) -> Float {
{
    case fn(v9, v10) { Cv3 }(3, 42) {
      v11 -> 2.0
      Cv3 -> fn(v12) { 100.0 }("data")
    }
  } /. {
    2.0
  }
}

pub fn main() {
  echo {
    {
      limit_value - limit_value
    } + 2
  } + limit_value
  echo "a"
  echo case {
      let prototype = 0.1
      let acc = prototype
      euler_value
    }, static([], True) {
    "bc" <> rest, 2.0 -> limit_value
    "a", 0.0 -> limit_value
    _, _ -> fn(v13, v14) { walk([], limit_value) }(True, 100.0)
  }
  echo case limit_value + limit_value, {
      let euler_value = 2
      Record
    } {
    6 as whole, _ -> euler_value
    7 as whole, Cv1("data", v15) if v15 > 9 || whole > 6 -> euler_value
    euler_value, Cv0("res") -> constructor(3 + euler_value, static([], False), [5])
    _, v16 -> case #("bc", 0.5) {
      #(_, euler_value) -> fn(v17, v18) { v18 }(0.5, "ab")
      b -> euler_value <> euler_value
      #("b", 10.0 as whole) -> euler_value
    }
  }
}
