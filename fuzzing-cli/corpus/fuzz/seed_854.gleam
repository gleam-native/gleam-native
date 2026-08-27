pub const tag_value: Bool = True

pub type Object {
  Cv0(value: String, inner: List(Int))
  Ok(Float, value: String)
  Cv1(String, value: List(Int))
}

fn f0(v2: List(Int)) -> String {
fn(v3) { v3 }("bc")
}

fn static(pair: Int, class: String) -> Float {
case [], Cv0("data", [5]) {
    [8, ..rest], Cv0("bc" <> tail, [_, 7, ..] as whole) if tail == "x" -> 0.0
    [_, ..rest], Cv0("data", [1]) -> case fn(v4, v5) { rest }(7, 3.14), pair {
      [b, ..rest], _ -> {
        0.25
      } -. {
        0.5
      }
      [4, ..rest], 6 -> {
        2.0
      } +. {
        100.0
      }
      [_, _, ..], v6 -> {
        0.0
      } *. {
        0.25
      }
      _, v7 -> 1.5
    }
    v8, _ -> 0.25
  }
}

fn extends(pair: Int, v9: #(String, Int), v10: String) -> String {
case pair, pair % 1 {
    _, _ -> v10 <> v10
    5, 2 -> "abc"
    _, 9 -> {
      fn(v11) { v10 }(42)
    } <> {
      {
        let v10 = [3]
        "a"
      }
    }
  }
}

pub fn main() {
  echo case 3 {
    a -> case 0.1 {
      prototype -> [3]
      item -> {
        let arguments = 5
        []
      }
    }
    1 -> case 1 {
      9 -> [2]
      6 -> {
        let pair = "constructor"
        let pair = []
        pair
      }
      7 -> [10, 100]
      v12 -> {
        let constructor = 100.0
        let class = constructor
        []
      }
    }
    pair -> case <<42:8, "b":utf8>> {
      <<_:utf8, "x":utf8, 42:1>> -> [0]
      <<100:4, y:8>> if y <= 6 -> [5]
      <<3:8>> -> fn(v13, v14) { [5, 0] }("ab", 0.0)
      _ -> [4, 2]
    }
  }
}
