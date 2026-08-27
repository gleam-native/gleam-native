pub type V0 {
  Some(value: String, inner: String)
  Record
  Cv1
}

pub type V2 {
  Cv3
  Cv4(value: String)
}

fn delete(v5: String, y: Float, length: Int) -> String {
case {
      let v5 = "ab"
      0
    }, 0.25 {
    7, 0.25 as whole -> case [7] {
      [_, ..rest] -> "x"
      [x, ..rest] if x <= 9 -> v5
      [b] as whole -> "ab"
      _ -> {
        let length = True
        let acc = y
        v5
      }
    }
    6, v5 -> {
      let length = length
      "x" <> "abc"
    }
    _, _ -> v5
  }
}

fn extends(v6: Int, v7: Int, delete: String) -> Int {
case Cv4("bc"), {
      let l = 1.5
      let delete = 10
      Record
    } {
    Cv4("data" <> _), Record -> case delete, delete {
      "constructor", "data" <> _ -> {
        let constructor = delete
        let this_ = [5, 4]
        v6
      }
      v8, "data" -> v7 + v6
      "ab", delete -> 4
      v9, v10 -> v6
    }
    v, Cv1 -> case "data" {
      inner | "ab" <> inner -> v7
      a | "data" <> a -> 10
      inner | "res" <> inner -> v6 * 10
    }
    _, _ -> {
      let v7 = {
        let m = v6
        [10, 4]
      }
      10
    }
  }
}

fn class(x: Float, v11: Bool) -> Int {
{
    4 + 42
  } - 1
}

pub fn main() {
  echo 0.25
  echo []
  echo {
    let s = [2]
    case {
        1.5
      } |> class("constructor" != "x"), 7 + 1 {
      1 as whole, 3 if whole % 2 == 0 -> whole
      4, 7 -> fn(v12) { 10 }(True)
      v13, _ -> {
        0.1
      } |> class(True)
    }
  }
}
