pub const limit_value: Int = 10

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(class: Int) -> List(Int) {
case 100 - 42 {
    0 -> [1, 0]
    5 -> [2]
    v0 -> {
      let v0 = v0
      let class = 0.5
      fn(v1) { [] }(True)
    }
  }
}

pub fn main() {
  let m = {
    {
      let limit_value = []
      0.0
    }
  } -. {
    3.14
  }
  echo 10.0
  echo limit_value
  echo case True {
    constructor -> [4, 100]
    constructor -> [3, 0]
    v2 -> case 1 {
      b -> constructor(limit_value)
      inner -> inner |> constructor()
      _ | 0 -> {
        let item = "data"
        let v2 = v2
        [2, 0]
      }
    }
  }
  echo limit_value
}
