use std::fs;
use resvg::usvg;
use resvg::usvg::TreeParsing;

use rayon::prelude::*;

fn main() {
    let far_nearer = fs::read_to_string("far nearer_template.svg")
        .expect("Something went wrong reading the file");

    (0..360).into_par_iter().step_by(10).for_each(|i| {
        let hue1_adjustment = 0;
        let hue2_adjustment = 0;
        let hue3_adjustment = 0;
        // https://colorpicker.me/#ff00ea
        let hsl1 = HSL::new(305, 100, 50);
        // https://colorpicker.me/#fa0060
        let hsl2 = HSL::new(337, 100, 49);
        // https://colorpicker.me/#ff9100
        let hsl3 = HSL::new(34, 100, 50);
        let saturation_adjustment = -10;
        let lightness_adjustment = -25;
        let (hsl1, hsl2, hsl3) = adjust_hues(&hsl1, &hsl2, &hsl3, i, hue1_adjustment, hue2_adjustment, hue3_adjustment);
        let (hsl1, hsl2, hsl3) = adjust_saturations(&hsl1, &hsl2, &hsl3, saturation_adjustment);
        let (hsl1, hsl2, hsl3) = adjust_lightnesses(&hsl1, &hsl2, &hsl3, lightness_adjustment);
        let result = replace_colors(far_nearer.clone(), &hsl1, &hsl2, &hsl3);
        let out_folder = "generated";
        create_out_folder(out_folder);
        let out_file = format!("{}/far-nearer-{}.png", out_folder, i);
        let rtree = get_rtree(result.clone());
        println!("Saving {}", out_file);
        save_png(rtree, &out_file);
    });
}

struct HSL {
    hue: u16,
    saturation: u8,
    lightness: u8,
}

impl HSL {
    fn new(hue: u16, saturation: u8, lightness: u8) -> HSL {
        HSL {
            hue,
            saturation,
            lightness,
        }
    }
}

fn get_rtree(text: String) -> resvg::Tree {
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_str(&text, &opt).unwrap();
    resvg::Tree::from_usvg(&tree)
}

fn save_png(rtree: resvg::Tree, out_file: &str) {
    let pixmap_size = rtree.size.to_int_size();
    let mut pixmap = resvg::tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height()).unwrap();
    rtree.render(resvg::tiny_skia::Transform::default(), &mut pixmap.as_mut());
    pixmap.save_png(&out_file).unwrap();
}

fn replace_colors(text: String, hsl1: &HSL, hsl2: &HSL, hsl3: &HSL) -> String {
    let result = replace_color(text, "COLOR_1", hsl1);
    let result = replace_color(result, "COLOR_2", hsl2);
    let result = replace_color(result, "COLOR_3", hsl3);
    result
}

fn replace_color(text: String, from: &str, to: &HSL) -> String {
    text.replace(from, &format!("hsl({},{}%,{}%)", to.hue, to.saturation, to.lightness))
}

fn create_out_folder(out_folder: &str) {
    if !std::path::Path::new(out_folder).exists() {
        std::fs::create_dir(out_folder).unwrap();
    }
}

fn adjust_lightnesses(hsl1: &HSL, hsl2: &HSL, hsl3: &HSL, amount: i32) -> (HSL, HSL, HSL) {
    (
        HSL::new(hsl1.hue, hsl1.saturation, ((hsl1.lightness as i32) + amount) as u8),
        HSL::new(hsl2.hue, hsl2.saturation, ((hsl2.lightness as i32) + amount) as u8),
        HSL::new(hsl3.hue, hsl3.saturation, ((hsl3.lightness as i32) + amount) as u8)
    )
}

fn adjust_saturations(hsl1: &HSL, hsl2: &HSL, hsl3: &HSL, amount: i32) -> (HSL, HSL, HSL) {
    (
        HSL::new(hsl1.hue, ((hsl1.saturation as i32) + amount) as u8, hsl1.lightness),
        HSL::new(hsl2.hue, ((hsl2.saturation as i32) + amount) as u8, hsl2.lightness),
        HSL::new(hsl3.hue, ((hsl3.saturation as i32) + amount) as u8, hsl3.lightness)
    )
}

// adjusts the hues of the three colors present in the template. x_adjustment variables are used to offset the original hues individually.
// Amount is used to change the hues of all three colors together.
fn adjust_hues(hsl1: &HSL, hsl2: &HSL, hsl3: &HSL, amount: i32, hsl1_adjustment: i32, hsl2_adjustment: i32, hsl3_adjustment: i32) -> (HSL, HSL, HSL) {
    (
        adjust_hue(hsl1, amount + hsl1_adjustment),
        adjust_hue(hsl2, amount + hsl2_adjustment),
        adjust_hue(hsl3, amount + hsl3_adjustment),
    )
}

fn adjust_hue(hsl: &HSL, amount: i32) -> HSL {
    let hue = hsl.hue as i32 + amount;
    let hue = if hue < 0 {
        hue + 360
    } else if hue >= 360 {
        hue - 360
    } else {
        hue
    };
    HSL {
        hue: hue as u16,
        saturation: hsl.saturation,
        lightness: hsl.lightness,
    }
}