pub const tag_value: Int = 4
pub const seed_value: Int = 10

pub type V0 {
  Cv1
  Cv2
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn delete(delete: Float) -> Int {
1
}

fn constructor(v3: Float, v4: V0, v5: Int) -> Int {
{
    case <<42:16>> {
      <<"":utf8>> -> fn(v6, v7) { 0.0 }(1.5, False)
      _ -> v3
    }
  } |> delete()
}

pub fn main() {
  let item = True
  echo {
    let pair = 1.5
    "a"
  }
}
