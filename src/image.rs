use crate::graphics::Canvas;

pub fn canvas_to_ppm(canvas: &Canvas) -> String {
    let mut output = vec![];

    output.push("P3".to_string());
    output.push(format!("{} {}", canvas.w(), canvas.h()));
    output.push("255".to_string());

    (0..canvas.h()).for_each(|y| {
        let mut current = String::new();

        (0..canvas.w())
            .flat_map(|x| {
                let c = canvas.px(x, y);

                fn to_int(v: f64) -> i32 {
                    if v < 0.0 {
                        0
                    } else if v > 1.0 {
                        255
                    } else {
                        (v * 255.0) as i32
                    }
                }

                [to_int(c.r()), to_int(c.g()), to_int(c.b())]
            })
            .for_each(|v| {
                if current.is_empty() {
                    current += &v.to_string();
                } else {
                    let v_str = format!(" {}", v);
                    if current.len() + v_str.len() <= 70 {
                        current += &v_str;
                    } else {
                        output.push(current.to_string());
                        current = v.to_string();
                    }
                }
            });

        if !current.is_empty() {
            output.push(current);
        }
    });

    // to ensure that file ends with a newline character
    output.push("".to_string());

    output.join("\n")
}

#[cfg(test)]
mod tests {
    use crate::graphics::Color;

    use super::*;

    #[test]
    fn test_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = canvas_to_ppm(&c);
        assert!(ppm.starts_with("P3\n5 3\n255\n"));
    }

    #[test]
    fn test_ppm_px_data() {
        let mut c = Canvas::new(5, 3);
        c.write_px(0, 0, Color::new(1.5, 0.0, 0.0));
        c.write_px(2, 1, Color::new(0.0, 0.5, 0.0));
        c.write_px(4, 2, Color::new(-0.5, 0.0, 1.0));
        let ppm = canvas_to_ppm(&c);
        let ppm = ppm.lines().collect::<Vec<_>>();

        assert_eq!(ppm[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(ppm[4], "0 0 0 0 0 0 0 127 0 0 0 0 0 0 0");
        assert_eq!(ppm[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn test_ppm_px_data_split() {
        let mut c = Canvas::new(10, 2);
        (0..2).for_each(|y| {
            (0..10).for_each(|x| {
                c.write_px(x, y, Color::new(1.0, 0.8, 0.6));
            })
        });
        let ppm = canvas_to_ppm(&c);
        let ppm = ppm.lines().collect::<Vec<_>>();

        assert_eq!(
            ppm[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            ppm[4],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            ppm[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            ppm[6],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    #[test]
    fn test_ppm_end_newline() {
        let ppm = canvas_to_ppm(&Canvas::new(5, 3));
        assert!(ppm.ends_with("\n"));
    }
}
