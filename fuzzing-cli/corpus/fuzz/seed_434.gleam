pub const seed_value: Int = 3
pub const euler_value: String = "abc"
pub const pi_value: Bool = True

pub type Map {
  Record
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn static(v0: String) -> Bool {
True
}

fn f1(constructor: List(Int)) -> Float {
{
    {
      let constructor = 0
      2.0
    }
  } +. {
    {
      100.0
    } -. {
      0.0
    }
  }
}

fn f2(v1: Int, v2: Float, arguments: List(Int)) -> Int {
{
    let v1 = v1
    let v2 = True
    case "bc" {
      _ -> v1
      item -> fn(v3) { v1 }("constructor")
      _ | "data" -> 3
    }
  }
}

pub fn main() {
  let item = False
  let value = {
    let length = seed_value * 0
    euler_value
  }
  echo {
    seed_value + {
      seed_value - seed_value
    }
  } - {
    case f2(3, 0.0, [1]), seed_value {
      0 as whole, 9 if whole <= 3 -> 4 + seed_value
      _, _ -> 42
      z, 3 as whole -> whole
    }
  }
  echo case fn(v4) { [4] }(0.25), <<"bc":utf8, "res":utf8, 3:8>> {
    [6, 7, ..], <<_:utf8>> -> "x"
    [constructor, ..rest], <<10:1>> -> euler_value <> {
      fn(v5) { euler_value }(10)
    }
    [], _ -> {
      euler_value <> euler_value
    } <> "abc"
    v6, v7 -> "a" <> "a"
  }
  echo case "" <> euler_value {
    item -> [4, 7]
    "bc" <> constructor -> case {
        let z = item
        let n = "constructor"
        value
      } {
      "ab" -> {
        let acc = "ab"
        []
      }
      inner -> []
    }
  }
  echo f2(case value <> "constructor" {
    item -> fn(v8) { 10 }(10.0)
    "b" -> 2
    "x" <> _ as whole -> 10 - 4
  }, case "x", "b" {
    _, _ -> fn(v9) { v9 }(0.0)
    "abc", "constructor" -> {
      1.5
    } +. {
      0.0
    }
    _, _ -> [] |> f1()
  }, [])
}
