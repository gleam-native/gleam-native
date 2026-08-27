pub const seed_value: Float = 0.1
pub const pi_value: Int = 4
pub const euler_value: Bool = False

fn f0(m: String, item: Int, default: Float) -> String {
{
    case <<"":utf8, 4:8>> {
      <<_:big-signed-8, "x":utf8, _:bytes>> -> m
      <<5:16>> -> m
      _ -> "b"
    }
  } <> m
}

pub fn main() {
  let s = "x"
  echo True
}
