pub const golden_value: String = "x"
pub const limit_value: Bool = False
pub const tag_value: Float = 10.0

fn f0(m: String) -> List(Int) {
case 7 < 1 {
    True | True -> [100, 5]
    inner -> [42, 1]
  }
}

fn f1(v0: String, pair: Int) -> Int {
case pair - 7 {
    v1 -> 5
    _ -> 0
  }
}

fn f2(v2: List(Int), new: Int, v3: Bool) -> Bool {
v3
}

pub fn main() {
  let golden_value = case 10 {
    _ -> [3, 2]
    9 | 5 -> {
      let default = False
      []
    }
    inner -> [1, 7]
  }
  let item = "data"
  echo case {
      let constructor = 10
      False
    } {
    False -> case True {
      inner -> {
        let arguments = []
        golden_value
      }
      True | True -> [2]
      inner -> golden_value
    }
    inner -> case 10.0, item <> item {
      2.0, "res" -> golden_value
      v4, "abc" <> rest -> "x" |> f0()
      _, v5 -> [10, 3]
    }
  }
  echo True
}
