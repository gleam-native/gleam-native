pub type V0 {
  Some(value: String, inner: List(Int))
}

pub type Object {
  Cv1(Bool)
  Cv2(value: Int, inner: String)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn new(v3: Float, l: Bool, v4: Int) -> List(Int) {
{
    let l = case <<"a":utf8, "ab":utf8>>, "x" {
      <<_:utf8, 3:8>>, "" <> rest if rest != "constructor" || rest == "ab" -> v3
      _, "a" -> 0.5
      _, v5 -> {
        0.0
      } -. v3
    }
    [7]
  }
}

fn f1(x: #(Bool, Bool)) -> List(Int) {
case "res" {
    inner | "" <> inner -> case inner {
      a -> [42]
      "res" -> {
        let v = 0
        let this_ = 0.5
        [1, 5]
      }
    }
    "res" <> rest -> case rest {
      "b" <> rest | "constructor" <> rest -> new(0.1, True, 10)
      l -> []
      "bc" <> _ | "b" -> [7]
    }
  }
}

pub fn main() {
  echo case "a", [100] {
    "x", [] -> [0, 7] |> walk(3)
    "abc", [3, ..rest] -> 42
    v6, v7 -> 0
  }
}
