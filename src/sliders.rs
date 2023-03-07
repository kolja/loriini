
use crate::model::{Area, Slider, SliderData};
use palette::Hsl;

impl Area {
    pub fn sliders(&mut self, sliders: Vec<Slider>, width: u8) -> &mut Self {

        let step = 1.0 / (width * 2) as f32;
        let (h, s, l) = self.color.into_components();
        let marker_color = Hsl::new(h + 180.0, s, (l + 0.2).clamp(0.0,1.0));
        let saturation_marker = (s * width as f32 * 2.0).floor() as u8;
        let lightness_marker = (l * width as f32 * 2.0).floor() as u8;

        self.sliders = sliders.into_iter().map(|b| match b {
            Slider::Hue(_) => {
                todo!()
            },
            Slider::Alpha(_) => {
                todo!()
            },
            Slider::Saturation(_) => {
                let data = SliderData {
                    colors:
                        (0..width * 2).map(|i| {
                            if saturation_marker == i {
                                marker_color
                            } else {
                                Hsl::new(h, i as f32 * step, l)
                            }
                        }).collect::<Vec<Hsl>>(),
                    pos: saturation_marker,
                    width
                };
                Slider::Saturation(Some(data))
            },
            Slider::Lightness(_) => {
                let data = SliderData {
                    colors:
                        (0..width * 2).map(|i| {
                            if lightness_marker == i {
                                marker_color
                            } else {
                                Hsl::new(h, s, i as f32 * step)
                            }
                        }).collect::<Vec<Hsl>>(),
                    pos: lightness_marker,
                    width
                };
                Slider::Lightness(Some(data))
            },
            Slider::Preview(_) => { Slider::Preview(Some(width)) }
        }).collect::<Vec<Slider>>();
        self
    }
}
