fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(constructor: #(String, String), v0: #(Float, Float), this_: Int) -> Int {
walk([], 100)
}

fn f1(x: Int, length: Bool) -> Bool {
case "a" <> "data" {
    "" <> _ -> fn(v1, v2) { fn(v3, v4) { True }(7, 1.0) }(2, True)
    item -> case item {
      "data" as whole if whole != "a" -> length
      m -> False
    }
  }
}

fn f2(s: String, v5: Int, z: Bool) -> Int {
case 2 {
    v6 -> 2
    inner -> 1
  }
}

pub fn main() {
  let this_ = {
    10 * 100
  } != 10
  let acc = {
    let s = walk([3, 10], 5)
    {
      1.5
    } /. {
      2.0
    }
  }
  echo f0(case "data", "abc" {
    "ab" <> _, "" <> rest if rest != "ab" -> {
      let v = rest
      let acc = this_
      #("res", "data")
    }
    "a", acc -> {
      let delete = [5]
      let delete = []
      #("data", "")
    }
    v7, _ -> #("res", "bc")
  }, #(1.5, 1.5), {
    {
      let acc = "b"
      acc
    }
  } |> f2(7, this_))
  echo 7
  echo {
    let new = "a"
    let new = case "a" <> "bc" {
      inner -> new <> new
      "a" -> "bc" <> new
    }
    {
      0.5
    } *. {
      10.0
    }
  }
}
