pub const euler_value: String = "x"

pub type V0 {
  Number(value: String, inner: Bool)
  Cv1(value: Bool)
  Record(List(Int), value: List(Int))
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn export(v2: Int) -> Float {
case #(100.0, False) {
    #(0.1, True as whole) -> case 2.0 {
      b -> {
        0.0
      } /. {
        0.5
      }
      whole -> {
        0.1
      } -. whole
    }
    #(2.0, False) -> 1.0
    _ -> {
      fn(v3) { 0.25 }("x")
    } +. {
      {
        let item = v2
        100.0
      }
    }
  }
}

fn new(new: Int, y: String, arguments: V0) -> Float {
{
    2.0
  } -. {
    {
      let y = [1]
      {
        0.0
      } -. {
        0.0
      }
    }
  }
}

fn f2(delete: V0, new: String) -> Bool {
{
    case delete {
      Cv1(False) -> 1.5
      new -> fn(v4, v5) { v5 }(False, 0.25)
    }
  } == {
    case delete {
      Cv1(inner) if !inner -> 10.0
      a -> {
        0.1
      } *. {
        0.25
      }
      item -> 1.0
    }
  }
}

pub fn main() {
  echo {
    let m = [1]
    let delete = m
    case delete, [1] {
      [delete, _, ..], [8, 3, ..] -> fn(v6, v7) { [1, 0] }(100.0, True)
      [5, 4, ..], [_, ..rest] -> delete
      [_, ..rest], [b, 2, ..] -> rest
      _, _ -> [2, 0]
    }
  }
  echo "a"
}
