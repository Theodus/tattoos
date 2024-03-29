use cairo::{self, Context, Error};
use std::f64::consts::PI;

type Result = core::result::Result<(), Error>;

fn main() -> Result {
    let x = 1000.0;

    let surface = cairo::SvgSurface::new(x, x, Some("out.svg"))?;
    let ctx = Context::new(&surface)?;
    ctx.translate(x * 0.5, x * 0.5);
    ctx.scale(1.0, -1.0);
    ctx.set_source_rgb(0.0, 0.0, 0.0);

    let w1 = x * 0.015;
    let w2 = w1 * 0.5;

    // outer ring
    scoped(&ctx, |ctx| {
        ctx.set_line_width(w2);
        ctx.arc(0.0, 0.0, x * 0.495, 0.0, PI * 2.0);
        ctx.stroke()?;

        let r = x * 0.34;
        ctx.set_line_width(w1);
        ctx.arc(0.0, 0.0, r, PI * 0.06, PI * 1.94);
        ctx.stroke()?;

        radial_repeat(&ctx, 12, PI * -0.5, |ctx| {
            let r1 = x * 0.025;
            ctx.translate(0.0, r + r1 * 2.0);
            ctx.set_line_width(w1);
            ctx.arc(0.0, 0.0, r1, 0.0, PI * 2.0);
            ctx.stroke()?;
            ctx.set_line_width(w2);
            ctx.arc(0.0, 0.0, x * 0.5 * 0.085, 0.0, PI * 2.0);
            ctx.stroke()
        })?;
        radial_repeat(&ctx, 12, PI / 12.0, |ctx| {
            ctx.translate(0.0, r);
            ctx.set_line_width(w2);
            isosceles_triangle_stroke(&ctx, x * 0.15, PI / 3.6)?;
            for ty in [0.0, 0.035, 0.035, 0.035] {
                ctx.translate(0.0, x * ty);
                isosceles_triangle_stroke(&ctx, x * 0.04, PI / 3.4)?;
            }
            Ok(())
        })?;

        // inner ring
        radial_repeat(&ctx, 6, PI / 6.0, |ctx| {
            ctx.translate(0.0, x * 0.21);
            ctx.set_line_width(w2);
            ctx.arc(0.0, 0.0, x * 0.12, 0.0, PI * 2.0);
            ctx.stroke()?;
            ctx.arc(0.0, 0.0, x * 0.05, 0.0, PI * 2.0);
            ctx.stroke()?;
            ctx.arc(0.0, 0.0, x * 0.1, -0.2, PI + 0.2);
            ctx.stroke()?;
            ctx.arc(0.0, 0.0, x * 0.1, 4.0, 5.4);
            ctx.stroke()?;
            ctx.translate(0.0, x * 0.017);
            isosceles_triangle_stroke(&ctx, -x * 0.05, PI / 3.0)?;
            let line_x = x * 0.08;
            ctx.move_to(0.0, x * 0.03);
            ctx.line_to(0.0, line_x);
            ctx.move_to(0.0, 0.0);
            ctx.line_to(line_x, line_x * (PI / 6.0).tan());
            ctx.move_to(0.0, 0.0);
            ctx.line_to(-line_x, line_x * (PI / 6.0).tan());
            let tangent_x = x * 0.0155;
            ctx.translate(tangent_x, -tangent_x / (PI / 6.0).tan());
            ctx.move_to(0.0, 0.0);
            ctx.line_to(line_x, line_x * (PI / 6.0).tan());
            ctx.move_to(0.0, 0.0);
            ctx.rel_line_to(0.0, -x * 0.09);
            ctx.translate(-tangent_x * 2.0, 0.0);
            ctx.move_to(0.0, 0.0);
            ctx.line_to(-line_x, line_x * (PI / 6.0).tan());
            ctx.move_to(0.0, 0.0);
            ctx.rel_line_to(0.0, -x * 0.09);
            ctx.stroke()
        })?;
        radial_repeat(&ctx, 6, 0.0, |ctx| {
            ctx.translate(0.0, x * 0.18);
            ctx.set_line_width(w2);
            let dx = x * 0.082;
            ctx.move_to(-dx, 0.0);
            ctx.line_to(dx, 0.0);
            ctx.stroke()
        })?;

        // central element
        scoped(&ctx, |ctx| {
            let r = x * 0.08;
            let a = r * 0.5;
            let b = a * f64::sqrt(3.0);
            let s = b / f64::cos(PI / 6.0);
            let n = 5;
            let l = r + (s * n as f64);

            // mask
            scoped(&ctx, |ctx| {
                ctx.set_operator(cairo::Operator::Clear);
                // ctx.set_source_rgb(0.0, 1.0, 1.0);
                ctx.move_to(-a, b);
                ctx.line_to(-r, 0.0);
                ctx.line_to(-a, -b);
                ctx.line_to(l - (s + a), -b);
                ctx.line_to(l, 0.0);
                ctx.line_to(l - (s + a), b);
                ctx.fill()
            })?;

            ctx.set_line_width(w1);
            ctx.arc(0.0, 0.0, r * 0.4, 0.0, PI * 2.0);

            ctx.move_to(r, 0.0);
            ctx.line_to(a, -b);
            ctx.line_to(-a, -b);
            ctx.line_to(-r, 0.0);
            ctx.line_to(-a, b);
            ctx.line_to(a, b);

            ctx.line_to(r, 0.0);
            ctx.line_to(l, 0.0);
            ctx.move_to(0.0, b);
            ctx.line_to(l - (s + a), b);
            ctx.line_to(l, 0.0);
            ctx.line_to(l - (s + a), -b);
            ctx.line_to(0.0, -b);

            for i in 0..(n - 1) {
                ctx.move_to(a + (s * i as f64), b);
                ctx.line_to(r + (s * (i + 1) as f64), 0.0);
                ctx.line_to(a + (s * i as f64), -b);
            }

            ctx.stroke()
        })?;

        Ok(())
    })?;

    Ok(())
}

fn scoped(ctx: &Context, f: impl Fn(&Context) -> Result) -> Result {
    ctx.save()?;
    f(ctx)?;
    ctx.restore()
}

fn radial_repeat(
    ctx: &Context,
    count: usize,
    offset: f64,
    f: impl Fn(&Context) -> Result,
) -> Result {
    for i in 0..count {
        let increment = -(2.0 * PI) / count as f64;
        ctx.save()?;
        ctx.rotate(increment * i as f64 + offset);
        f(ctx)?;
        ctx.restore()?;
    }
    Ok(())
}

fn isosceles_triangle_stroke(ctx: &Context, height: f64, top_angle: f64) -> Result {
    ctx.move_to(0.0, height);
    let b = height * (top_angle / 2.0).tan();
    ctx.line_to(b, 0.0);
    ctx.line_to(-b, 0.0);
    ctx.close_path();
    ctx.stroke()
}
