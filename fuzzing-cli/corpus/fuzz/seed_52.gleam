pub type Record {
  Record
  Cv0(value: Bool)
  Cv1
}

fn walk(xs: List(Int), acc: Int) -> Int {
  case xs {
    [] -> acc
    [x, ..rest] -> walk(rest, acc + x)
  }
}

fn f0(this_: Record, n: #(Int, String)) -> Int {
{
    let n = [7, 7]
    let value = {
      0 - 2
    } * 3
    case <<100:16>>, <<"bc":utf8, "ab":utf8, 3:1>> {
      <<_:little-signed-8, 4:16, "b":utf8>>, <<7:1>> as whole -> value % 5
      <<7:4>>, <<1:16, _:utf8>> -> 7
      _, <<"ab":utf8>> -> value
      _, _ -> value
    }
  }
}

pub fn main() {
  let value = case {
      let this_ = []
      "a"
    }, "data" {
    "x" <> rest, "" <> tail if tail == "bc" -> [4]
    "ab" <> rest, "abc" as whole -> []
    _, _ -> {
      let item = [3, 10]
      [4, 42]
    }
  }
  let value = True
  echo !False
  echo [7, 10]
  echo 1
  echo fn(v2, v3) { "abc" }(2, "b")
}
