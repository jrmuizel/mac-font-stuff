/*
 * Copyright 2006 The Android Open Source Project
 *
 * Use of this source code is governed by a BSD-style license that can be
 * found in the LICENSE file.
 */

extern crate core_graphics;
extern crate core_text;
extern crate core_foundation;
use core_graphics::base::{kCGImageAlphaNoneSkipFirst};
use core_foundation::string::UniChar;
use core_foundation::base::CFIndex;
use core_graphics::color_space::CGColorSpace;
use core_graphics::context::CGContext;
use core_graphics::font::{CGFont, CGGlyph};
use core_graphics::geometry::CGPoint;
use core_text::font;

#[test]
fn it_works() {
    let k = supports_LCD();
    assert!(k);
}

fn supports_LCD() -> bool {
    let mut cg_context = CGContext::create_bitmap_context(1, 1, 8, 4,
                                                          &CGColorSpace::create_device_rgb(),
                                                          kCGImageAlphaNoneSkipFirst |
                                                          (2 << 12) //kCGBitmapByteOrder32Little
                                                          );
    let ct_font = font::new_from_name("Helvetica", 16.).unwrap();
    cg_context.set_should_smooth_fonts(true);
    cg_context.set_should_antialias(true);
    cg_context.set_allows_font_smoothing(true);
    //cg_context.set_text_drawing_mode(
    cg_context.set_rgb_fill_color(1.0, 1.0, 1.0, 1.0);
    let point = CGPoint {x: -1., y: 0.};
    let characters: [UniChar; 1] = ['|' as UniChar];
    let mut glyphs: [CGGlyph; 1] = [0 as CGGlyph];
    let count: CFIndex = 1;
    ct_font.get_glyphs_for_characters(&characters[0], &mut glyphs[0], count);
    // XXX: it would be nice if we didn't have to clone cg_context here
    ct_font.draw_glyphs(&glyphs, &[point], cg_context.clone());
    let data = cg_context.data();
    let rgb : u32 = unsafe { std::mem::transmute::<[u8; 4], u32>([data[0],data[1],data[2],data[3]]) };
    let r = (rgb >> 16) & 0xFF;
    let g = (rgb >>  8) & 0xFF;
    let b = (rgb >>  0) & 0xFF;
    let supports_LCD = r != g || r != b;
    supports_LCD
}
