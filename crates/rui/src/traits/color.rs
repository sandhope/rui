use gpui::Hsla;

pub trait SoftColor {
    fn soft(&self) -> Hsla;
    fn hover(&self) -> Hsla;
}

impl SoftColor for Hsla {
    fn soft(&self) -> Hsla {
        let h = self.h;
        let s = (self.s * 1.0).clamp(0., 1.);
        let l = (self.l * 1.3).min(0.96);
        let a = (self.a * 0.3).max(0.1);

        Hsla { h, s, l, a }
    }

    fn hover(&self) -> Hsla {
        Hsla {
            h: self.h,
            s: self.s,
            l: (self.l + 0.05).min(1.0),
            a: self.a,
        }
    }
}
