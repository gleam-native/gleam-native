fn f0(m: String) -> String {
m
}

fn f1(v0: String, v1: String) -> Bool {
True
}

fn f2(v2: Bool) -> Float {
{
    {
      3.14
    } /. {
      1.0
    }
  } +. {
    1.0
  }
}

pub fn main() {
  let n = {
    fn(v3, v4) { 0.25 }("constructor", 100)
  } +. {
    fn(v5, v6) { 0.25 }(3, 1)
  }
  echo case {
      let default = 2
      []
    } {
    [n] if n <= 9 || n == 0 -> case f0("b"), {
        let prototype = [5]
        n
      } {
      v7, n -> {
        1.0
      } +. {
        1.5
      }
      "a" <> _, n -> 0.5
    }
    [_, ..rest] -> f2(f1("res", "abc"))
    [3, ..rest] -> case f0("ab"), [] {
      "res", [6] -> n
      "constructor", [9] -> 10.0
      "ab", [] -> n *. {
        1.5
      }
      v8, v9 -> f2(False)
    }
    v10 -> f2(True) -. n
  }
  echo 0 - 1
  echo case {
      let z = 2
      0.5
    } {
    inner -> case {
        let n = n
        let prototype = True
        [4, 10]
      } {
      [x] -> "b"
      [7, b, ..] as whole -> "data"
      [8] -> "bc" |> f0()
      v11 -> fn(v12) { "abc" }(False)
    }
    3.14 -> "a" <> {
      {
        let default = 0.5
        ""
      }
    }
  }
}
