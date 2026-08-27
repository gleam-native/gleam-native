pub const seed_value: Int = 10
pub const euler_value: String = "ab"

pub type V0 {
  Cv1(value: List(Int))
  Cv2(String)
}

pub type Record {
  Number(value: Float)
  Cv3(List(Int))
  Cv4(Bool)
}

pub type Number {
  Cv5
  Cv6
  Some
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(v7: Bool, class: Float, s: Int) -> String {
"data"
}

pub fn main() {
  let s = case 0 - seed_value, 7 {
    9, 3 as whole if whole == 1 && whole > 8 -> [1]
    _, 4 -> []
    6, 1 -> {
      let euler_value = True
      [7]
    }
    v8, v9 -> [7]
  }
  echo {
    let n = True
    let seed_value = False
    {
      let n = "a"
      0.25
    }
  }
  echo case "b", {
      let euler_value = "b"
      0.25
    } {
    euler_value, 0.25 if euler_value == "" -> case fn(v10) { 42 }(4), {
        100.0
      } -. {
        3.14
      } {
      v11, 2.0 -> "a" == euler_value
      4, 10.0 -> False && True
      2 as whole, _ -> True || True
      _, _ -> True
    }
    "" <> _, _ -> False
    _, v12 -> case 10.0 {
      b -> False
      constructor -> True || False
    }
  }
  echo case "" {
    v13 | "bc" <> v13 -> {
      fn(v14, v15) { "ab" }(3.14, 3.14)
    } <> {
      fn(v16, v17) { "abc" }("abc", True)
    }
    "a" -> euler_value
    "res" -> "res"
  }
  echo case seed_value - seed_value, #(5, True) {
    4, #(1, True) -> {
      s |> walk(seed_value * seed_value)
    } - 7
    5 as whole, #(_, True as it) -> 4
    9, #(7, this_) -> case [1] {
      [9, a, ..] -> seed_value
      [1, ..rest] -> walk(rest, seed_value)
      _ -> walk(s, 100)
    }
    _, _ -> 1 - {
      [3, 4] |> walk(walk([100], 100))
    }
  }
}
