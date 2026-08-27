pub const limit_value: Int = 0

pub type V0 {
  Cv1
}

pub type V2 {
  Cv3(Bool)
}

fn arguments(pair: V0, this_: Int) -> List(Int) {
case [2, 0], {
      let pair = True
      Cv3(True)
    } {
    [4, _, ..], acc -> [3, 5]
    [7, ..rest] as whole, Cv3(_) -> [7, 42]
    [a, ..rest], Cv3(_) as whole -> [1]
    _, _ -> []
  }
}

fn f1(value: V0, acc: V2) -> Bool {
case "bc" {
    _ -> True
    s -> case fn(v4) { [] }("constructor") {
      [5, _, ..] -> False
      [a, ..rest] -> True
      v5 -> 3 >= 7
    }
    "data" -> True
  }
}

pub fn main() {
  let default = fn(v6, v7) { {
    let acc = 3.14
    let limit_value = "x"
    [100]
  } }(True, True)
  echo limit_value + {
    4 * 5
  }
  echo "x"
  echo [0, 42]
}
