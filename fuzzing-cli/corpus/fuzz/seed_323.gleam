pub type V0 {
  Cv1(value: List(Int), inner: Int)
  Cv2
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn delete(v3: Bool) -> String {
{
    case [10, 7], [] {
      [], [0, 0, ..] -> "bc"
      [7, ..rest], [v3] -> "ab"
      [7], [0, ..rest] -> "ab"
      _, _ -> "abc"
    }
  } <> ""
}

fn f1(v4: Bool, new: Int) -> Bool {
{
    let length = case #(True, True), "ab" {
      #(_, new), "ab" <> rest if rest != "x" && new -> rest <> rest
      #(v5, True), "abc" <> rest -> "" <> "bc"
      #(_, True), "ab" -> v4 |> delete()
      v6, v7 -> "x" <> "ab"
    }
    True && False
  }
}

pub fn main() {
  echo {
    let v = spin(7, 42)
    case {
        let v = True
        let v = "ab"
        Cv2
      } {
      _ -> {
        0.0
      } -. {
        0.25
      }
      Cv1([], _) -> {
        100.0
      } +. {
        0.25
      }
      item -> {
        0.5
      } +. {
        0.1
      }
    }
  }
  echo case 0 {
    inner -> {
      let m = fn(v8) { [10] }(0.0)
      let rest = ""
      "ab"
    }
    3 -> case "ab" {
      "abc" -> "bc" <> "data"
      inner -> "abc"
      "constructor" | "a" -> "x"
    }
  }
  echo "a" <> {
    {
      let m = fn(v9) { 100.0 }(1)
      let value = 3.14
      "res" <> "constructor"
    }
  }
  echo {
    case Cv1([7, 100], 2), Cv2 {
      Cv2, x -> 0.25
      Cv1([5, 1, ..], s), Cv2 as whole -> {
        0.25
      } +. {
        0.5
      }
      v10, _ -> 0.1
    }
  } == {
    case "bc" <> "a" {
      "" <> _ | "data" -> 0.0
      a -> 1.5
    }
  }
}
