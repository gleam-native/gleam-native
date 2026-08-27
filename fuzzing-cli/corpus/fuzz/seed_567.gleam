pub type V0 {
  Cv1(value: List(Int))
  Cv2(Float)
  Some(value: Int, inner: List(Int))
}

pub type V3 {
  Cv4(Float, value: Bool)
  Cv5(List(Int))
}

pub type V6 {
  Cv7(value: String, inner: Bool)
  Record
  Cv8(value: Bool)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn yield(v9: String) -> Bool {
case v9 <> "constructor" {
    v10 -> False
    b | "res" <> b -> {
      {
        let l = True
        2.0
      }
    } <. {
      0.25
    }
  }
}

fn new(s: List(Int)) -> Int {
3 + 4
}

pub fn main() {
  let z = {
    {
      3.14
    } -. {
      2.0
    }
  } == {
    {
      1.0
    } *. {
      3.14
    }
  }
  echo [5, 42]
  echo case {
      let constructor = "b"
      0.5
    }, [42, 4] {
    1.5, [5, ..rest] -> {
      100.0
    } -. {
      fn(v11, v12) { 0.1 }(True, 1)
    }
    v13, [2, ..rest] -> {
      fn(v14, v15) { 100.0 }(100, "")
    } +. {
      {
        3.14
      } *. {
        0.5
      }
    }
    _, v16 -> case "abc" {
      b -> 3.14
      v16 | "a" <> v16 -> 0.1
    }
  }
}
