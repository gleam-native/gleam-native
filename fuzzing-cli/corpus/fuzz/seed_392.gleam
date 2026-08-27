pub const limit_value: Bool = True

pub type V0 {
  Cv1
  Cv2
  Cv3(value: String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(delete: Int) -> Int {
case <<2:1, 10:16, "bc":utf8>> {
    <<_:big-signed-4, _:utf8, 0:8>> -> case Cv1 {
      _ -> fn(v4) { delete }("a")
      Cv2 as whole -> 2
    }
    <<x:1, _:utf8, 2:8>> -> fn(v5) { delete - x }(True)
    _ -> 3
  }
}

fn delete(pair: Int, acc: Float, v6: Bool) -> List(Int) {
{
    let s = case {
        let acc = [5]
        #(False, True)
      } {
      #(True, False) | #(False, True) -> "constructor"
      inner -> fn(v7) { "ab" }(1)
    }
    [0]
  }
}

fn f2(s: Int, v8: #(List(Int), Bool)) -> Int {
fn(v9, v10) { v9 * {
    fn(v11) { s }(True)
  } }(5, "bc")
}

pub fn main() {
  echo {
    let m = [7]
    {
      let item = {
        1.5
      } *. {
        0.1
      }
      let limit_value = "bc" <> "ab"
      "ab" <> limit_value
    }
  }
  echo {
    case [] {
      [a, ..rest] -> {
        let rest = 4
        "constructor"
      }
      [] -> "bc"
      [a] -> "a"
      _ -> "b"
    }
  } <> "bc"
  echo "constructor"
}
