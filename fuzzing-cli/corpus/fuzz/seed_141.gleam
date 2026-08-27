pub type V0 {
  Cv1
  Cv2
  Cv3
}

pub type V4 {
  Record(Float, value: Int)
  Ok(String)
}

pub type Map {
  Cv5(Bool, Float)
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn class(v6: Int) -> String {
case v6 % 4, [1] {
    7 as whole, [] -> case Cv1 {
      Cv2 -> "bc" <> "data"
      _ -> fn(v7, v8) { "x" }(10.0, 10)
    }
    6 as whole, [] as it -> "data"
    v9, v10 -> {
      let acc = "abc" <> "abc"
      "x"
    }
  }
}

fn yield(pair: Int) -> String {
fn(v11, v12) { "b" }(2, False)
}

fn static(y: Bool) -> List(Int) {
{
    let l = {
      0.1
    } +. {
      0.5
    }
    let arguments = fn(v13) { [] }("bc")
    fn(v14, v15) { arguments }(2.0, True)
  }
}

pub fn main() {
  let item = ""
  let delete = 3 == 7
  echo item
  echo []
  echo {
    let value = case #([4, 5], "b") {
      item -> 4
      #([8, ..rest], _) -> 7
      constructor -> [7] |> walk(fn(v16) { 10 }(100.0))
    }
    let delete = delete
    case static(delete) {
      [value] -> {
        10.0
      } <. {
        0.1
      }
      [8, 1, ..] -> delete
      [3] -> False
      _ -> {
        let self_ = [7, 1]
        let value = 0.1
        delete
      }
    }
  }
}
