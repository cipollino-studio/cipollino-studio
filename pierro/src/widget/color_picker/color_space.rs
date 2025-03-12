
pub trait ColorSpace {
    fn to_rgb(color: [f32; 3]) -> [f32; 3];
    fn from_rgb(rgb: [f32; 3]) -> [f32; 3];
}

pub enum HSVColorSpace {}
impl ColorSpace for HSVColorSpace {

    fn to_rgb(color: [f32; 3]) -> [f32; 3] {
        let [h, s, v] = color;

        let h = h * 6.0;
        let h_int = h.floor() as i32;
        let h_frac = h.fract();

        let alpha = v * (1.0 - s);
        let beta = v * (1.0 - h_frac * s);
        let gamma = v * (1.0 - (1.0 - h_frac) * s);

        match h_int {
            0 => [v, gamma, alpha],
            1 => [beta, v, alpha],
            2 => [alpha, v, gamma],
            3 => [alpha, beta, v],
            4 => [gamma, alpha, v],
            5 => [v, alpha, beta],
            _ => [0.0, 0.0, 0.0]
        }
    }

    fn from_rgb(rgb: [f32; 3]) -> [f32; 3] {
        let [r, g, b] = rgb;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = (max - min).max(0.000001);
        
        let h = match max {
            _ if max == r => (g - b) / delta,
            _ if max == g => (b - r) / delta + 2.0,
            _ if max == b => (r - g) / delta + 4.0,
            _ => 0.0
        };
        let mut h = h / 6.0;
        if h < 0.0 {
            h += 1.0;
        } 
        let v = max;
        let s = if v == 0.0 {
            0.0
        } else {
            delta / v
        };

        [h, s, v]
    }
    
}
