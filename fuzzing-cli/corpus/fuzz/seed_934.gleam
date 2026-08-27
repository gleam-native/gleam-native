pub type V0 {
  Number(value: String, inner: String)
  Cv1
  Cv2(value: Int, inner: List(Int))
}

fn f0(v3: Float) -> Int {
{
    let v3 = case fn(v4, v5) { Number("", "res") }(100, "ab") {
      v6 -> 4
      constructor -> 1 - 100
      Cv2(item, _) -> {
        let v3 = 0.25
        let n = True
        100
      }
    }
    case fn(v7, v8) { 2.0 }(True, 100.0), "" <> "abc" {
      10.0, "bc" -> v3
      100.0, "bc" <> rest as whole -> v3 % 1
      _, v9 -> v3
    }
  }
}

fn f1(v10: List(Int)) -> Float {
{
    case {
        let item = 4
        Cv1
      } {
      Cv2(v11, [_]) if v11 % 2 == 0 -> {
        0.25
      } +. {
        0.0
      }
      Number("a", "data" as whole) -> {
        0.0
      } -. {
        1.0
      }
      _ -> 10.0
    }
  } +. {
    case v10 {
      [] -> 1.5
      [v10] -> {
        let length = "abc"
        let l = 4
        2.0
      }
      [_, ..rest] -> {
        0.25
      } +. {
        1.0
      }
      _ -> 0.5
    }
  }
}

fn f2(v12: Int, n: Int, y: String) -> String {
y
}

pub fn main() {
  echo f1([1])
}
