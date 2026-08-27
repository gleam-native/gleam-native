pub const euler_value: Int = 7
pub const pi_value: Int = 100

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn constructor(constructor: Int) -> Bool {
case "b" {
    b | "x" <> b -> False && True
    _ -> {
      let delete = [5, 42]
      let z = constructor - 7
      z <= constructor
    }
  }
}

pub fn main() {
  echo fn(v0, v1) { case [7, 5] {
    [8, ..rest] -> {
      let arguments = True
      let m = "data"
      rest
    }
    [] -> [5]
    v2 -> [42]
  } }("a", 1.5)
  echo case {
      let s = 1.0
      0
    }, 3 - 0 {
    2, 4 as whole -> 2.0
    pair, 9 if pair == 3 -> case 3 {
      _ -> 0.25
      item -> {
        100.0
      } +. {
        0.0
      }
    }
    m, 1 -> {
      fn(v3, v4) { 0.25 }(True, False)
    } +. {
      3.14
    }
    v5, _ -> fn(v6) { 100.0 }(False)
  }
  echo [7, 0]
  echo case [2, 7] {
    [0] -> case pi_value, #(2.0, 0.5) {
      v7, #(_, _) -> {
        let euler_value = "data"
        let class = False
        [10, 5]
      }
      2, #(1.0 as whole, prototype) -> []
      v8, #(_, _) -> {
        let value = 1.0
        [4, 42]
      }
      _, _ -> [42, 1]
    }
    [euler_value] -> case 10.0, 3 - pi_value {
      _, _ -> [0, 3]
      100.0, 9 -> [0, 0]
      10.0 as whole, 2 -> []
    }
    [1, ..rest] -> case <<0:1>>, "b" {
      <<"a":utf8>>, "x" -> [5, 3]
      <<_:utf8, "":utf8, "ab":utf8>>, _ -> [3]
      _, "res" as whole -> []
      v9, v10 -> fn(v11) { [] }("x")
    }
    v12 -> v12
  }
}
