use colorful::{Colorful,RGB};

pub fn print_banner() {
  
  let banner_txt = " - The Progressive JavaScript Framework";
  println!(
    "\n{}{}\n",
    "Vue.js ".color(RGB::new(66, 211, 146)),
    banner_txt
      .gradient_with_color(RGB::new(66, 211, 146), RGB::new(100, 126, 255))
      
  );
}
