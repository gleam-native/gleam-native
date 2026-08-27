fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(constructor: Bool, rest: Int, m: Int) -> Bool {
False
}

fn extends(default: Bool, length: Int) -> List(Int) {
[10]
}

fn f2(v0: Bool, arguments: String, item: String) -> List(Int) {
{
    let v0 = {
      fn(v1) { 1.0 }(True)
    } -. {
      3.14
    }
    [1]
  }
}

pub fn main() {
  echo case {
      let length = [10]
      "constructor"
    }, "b" <> "a" {
    _, "" <> _ -> 2.0
    "res", m if m == "" && m != "b" -> {
      {
        1.0
      } *. {
        1.5
      }
    } -. {
      0.25
    }
    "abc" <> rest, default -> {
      fn(v2, v3) { 3.14 }("a", 0)
    } *. {
      2.0
    }
    _, _ -> 100.0
  }
  echo {
    case <<3:4, "b":utf8>>, spin(4, 4) {
      <<_:utf8, 42:8, "b":utf8>>, 2 -> spin(100, 42)
      <<_:utf8>>, 4 -> 42
      _, 3 -> 4 - 10
      v4, v5 -> 7
    }
  } % 2
  echo case #(1.0, True) {
    a -> case spin(5, 5) {
      a -> {
        let a = "res"
        let a = "bc"
        [10, 5]
      }
      7 -> []
      item -> [2, 0]
    }
    #(_, _) as whole -> case "a" <> "constructor" {
      item -> [100, 4]
      constructor -> []
      "bc" as whole -> f2(False, "data", "")
    }
    #(_, True) -> case #(0.5, True) {
      #(0.25, _) -> [2, 5]
      inner -> fn(v6, v7) { [0] }("ab", "bc")
      constructor -> [7]
    }
  }
  echo "abc" <> "res"
}
