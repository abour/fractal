pub fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
	let (r, g, b);

	if s == 0. {
		r = l;
		g = l;
		b = l;
	} else {
		let (q, p);
		if l < 0.5 {
			q = l * (1. + s);
		} else {
			q = l + s - l * s;
		}

		p = 2. * l - q;
		r = hue_to_rgb(p, q, h + 1.0 / 3.0);
		g = hue_to_rgb(p, q, h);
		b = hue_to_rgb(p, q, h - 1.0 / 3.0);
	}

	return ((r * 255.) as u8, (g * 255.) as u8, (b * 255.) as u8);
}

fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
	if t < 0. {
		t += 1.0;
	}
	if t > 1.0 {
		t -= 1.0;
	}

	if t < (1.0 / 6.0) {
		return p + (q - p) * 6.0 * t;
	} else if t < (1.0 / 2.0) {
		return q;
	} else if t < (2.0 / 3.0) {
		return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
	}

	return p;
}
