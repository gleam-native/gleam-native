pub const tag_value: Float = 0.25
pub const seed_value: Float = 10.0
pub const golden_value: String = ""

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Int, v0: Int, class: String) -> String {
{
    case "" {
      constructor | "bc" <> constructor -> {
        let x = 1.0
        class
      }
      _ | "b" <> _ -> "ab"
    }
  } <> "abc"
}

fn f1(v1: Bool) -> Bool {
case False {
    True | True -> case spin(1, 4), {
        let v1 = True
        let this_ = [3, 10]
        [1, 100]
      } {
      v1, [9, ..rest] if v1 == 0 || v1 > 2 -> False
      3, [b, ..rest] -> True
      _, [6] -> v1
      _, _ -> {
        10.0
      } <. {
        10.0
      }
    }
    False -> case 0.1 {
      1.5 -> v1
      10.0 -> fn(v2, v3) { False }("bc", 4)
      _ -> fn(v4) { v4 }(True)
    }
    False -> {
      0.1
    } >. {
      {
        100.0
      } *. {
        0.25
      }
    }
  }
}

fn class(n: Int) -> Bool {
10 == {
    case [42, 10] {
      [x, ..rest] -> n
      [h] if h == 9 && h <= 5 -> spin(n, n)
      [] -> 5
      _ -> spin(7, n)
    }
  }
}

pub fn main() {
  let pair = False
  echo [0]
  echo False
  echo True
}
