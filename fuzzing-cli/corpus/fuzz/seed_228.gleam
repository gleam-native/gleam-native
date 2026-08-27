pub const seed_value: Float = 1.0

pub type V0 {
  Cv1(value: List(Int))
  Cv2(Bool)
}

pub type Object {
  Cv3
  Cv4
  Error(value: String, inner: Int)
}

fn f0(this_: String, v5: Float) -> Int {
case 7 {
    item -> item
    _ | 6 -> case {
        let rest = True
        []
      } {
      [8, ..rest] -> {
        let pair = False
        2
      }
      [_, ..rest] -> 42
      [1, ..rest] -> 4
      _ -> 5 - 3
    }
  }
}

fn f1(new: Int, y: #(List(Int), Int), z: Bool) -> Bool {
case fn(v6) { [10, 2] }("bc"), "ab" <> "res" {
    [], new if new != "data" -> z
    [9, ..rest], "x" as whole -> {
      let y = 2
      z && z
    }
    [x, a, ..], _ -> z
    _, _ -> case [2, 5], Cv2(True) {
      [h, 6, ..], Cv1([a, ..rest]) if h <= 1 && a == 0 -> z
      [4, 6, ..], z -> 7 >= 2
      v7, _ -> {
        let prototype = 0
        let prototype = True
        False
      }
    }
  }
}

pub fn main() {
  let m = case seed_value -. {
      0.0
    }, 2 {
    100.0, 4 -> []
    v, _ -> {
      let v = 2
      []
    }
    100.0, 3 -> []
  }
  let seed_value = 100
  echo "bc"
}
