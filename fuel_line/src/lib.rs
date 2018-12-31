// For tests
#[allow(unused_imports)]
extern crate bytes;
#[allow(unused_imports)]
#[macro_use] extern crate fuel_line_derive;
#[allow(unused_imports)]
extern crate uuid;

pub trait Render {
  fn render(&self) -> String;
}

#[macro_export]
macro_rules! templatify {
  ( $head_template:expr $(;$key:expr; $template:expr)* ) => {
    {
      let mut total_length = 0;
      total_length = total_length + $head_template.len();

      $(
        total_length = total_length + $key.len() + $template.len();
      )*

      let mut output_string = String::with_capacity(total_length);
      output_string.push_str($head_template);

      $(
        output_string.push_str($key);
        output_string.push_str($template);
      )*

      output_string
    }
  }
}

#[macro_export]
macro_rules! templatify_buffer {
  ( $buffer:ident, $head_template:expr $(;$key:expr; $template:expr)* ) => {
    {
      let mut total_length = 0;
      total_length = total_length + $head_template.len();

      $(
        total_length = total_length + $key.len() + $template.len();
      )*

      $buffer.reserve(total_length);
      $buffer.put($head_template);

      $(
        $buffer.put($key);
        $buffer.put($template);
      )*
    }
  }
}

#[cfg(test)]
mod tests {
  use bytes::{BytesMut, BufMut};
  use Render;
  use uuid::Uuid;

  #[test]
  fn templatify_should_work() {
    let world = "world";
    let results: String = templatify! { "hello, "; world ;"!" };
    assert!(results == "hello, world!");
  }

  #[test]
  fn templatify_buffer_should_work() {
    let mut buf = BytesMut::new();

    let world = "world";
    templatify_buffer! { buf, "hello, "; world ;"!" };
    assert!(buf == "hello, world!");
  }

  #[test]
  fn render_derive_should_work() {

    #[derive(Render)]
    #[TemplateName = "./fuel_line/test_data/test.html"]
    struct TestStruct {
      a: String,
      b: String
    };

    let t = TestStruct {
      a: "a_value".to_owned(),
      b: "b_value".to_owned()
    };

    assert!(t.render() == "<h1>b_value</h1>\n<p>a_value</p>\n");
  }
}

