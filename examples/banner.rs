use colorful::{Colorful, HSL};

fn main() {
  println!("{}", "言葉にできず　凍えたままで 人前ではやさしく生きていた しわよせで　こんなふうに雑に 雨の夜にきみを　抱きしめてた".gradient_with_color(HSL::new(0.0, 1.0, 0.5), HSL::new(0.833, 1.0, 0.5)).underlined());
}
