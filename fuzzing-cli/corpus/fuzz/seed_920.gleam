pub const tag_value: Int = 0

pub type Symbol {
  Record
}

fn spin(n: Int, acc: Int) -> Int {
  case n <= 0 {
    True -> acc
    False -> spin(n - 1, acc + n)
  }
}

fn f0(delete: Int, v: Int) -> String {
"" <> "b"
}

pub fn main() {
  echo {
    let y = 10
    let m = 0.0
    case Record {
      Record -> y
      this_ -> {
        let tag_value = m
        y
      }
      Record -> 0
    }
  }
  echo {
    case fn(v0) { tag_value }("ab"), Record {
      _, Record -> False && False
      6, Record -> False
      2, Record -> True
      _, _ -> fn(v1, v2) { v2 }(2.0, False)
    }
  } && True
  echo "b" <> {
    case Record, Record {
      Record, Record -> f0(1, tag_value)
      item, Record -> "ab"
      _, _ -> "abc" <> "bc"
    }
  }
  echo {
    spin(2, 2) - {
      3 - tag_value
    }
  } |> f0(tag_value)
}
