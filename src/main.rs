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

    let r = x * 0.1;
    let a = r * 0.5;
    let b = a * f64::sqrt(3.0);
    let s = b / f64::cos(PI / 6.0);
    let n = 5;
    let l = r + (s * n as f64);
    translated(&ctx, -(l - r) / 2.0, 0.0, |ctx| {
        ctx.set_line_width(r * 0.2);
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

    {
        // debug point
        ctx.save()?;
        ctx.set_source_rgb(1.0, 0.0, 0.0);
        ctx.arc(0.0, 0.0, x / 250.0, 0.0, PI * 2.0);
        ctx.fill()?;
        ctx.restore()?;
    }

    // old_design(&ctx, x)?;

    Ok(())
}

fn translated(ctx: &Context, dx: f64, dy: f64, f: impl Fn(&Context) -> Result) -> Result {
    ctx.save()?;
    ctx.translate(dx, dy);
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

fn old_design(ctx: &Context, x: f64) -> Result {
    let half = x * 0.5;
    ctx.set_line_width(half * 0.02);
    ctx.arc(0.0, 0.0, half * 0.97, 0.0, PI * 2.0);
    ctx.stroke()?;
    radial_repeat(&ctx, 12, PI / 12.0, |ctx| {
        ctx.translate(0.0, half * 0.66);
        ctx.set_line_width(half * 0.012);
        ctx.rectangle(-half * 0.02, half * 0.001, half * 0.04, half * 0.04);
        ctx.stroke()?;
        ctx.set_line_width(half * 0.012);
        isosceles_triangle_stroke(&ctx, half * 0.28, PI / 3.4)?;
        ctx.translate(0.0, half * 0.05);
        ctx.set_line_width(half * 0.02);
        isosceles_triangle_stroke(&ctx, half * 0.08, PI / 3.4)?;
        ctx.translate(0.0, half * 0.08);
        isosceles_triangle_stroke(&ctx, half * 0.08, PI / 3.4)?;
        ctx.translate(0.0, half * 0.08);
        isosceles_triangle_stroke(&ctx, half * 0.08, PI / 3.4)?;
        Ok(())
    })?;
    radial_repeat(&ctx, 12, 0.0, |ctx| {
        ctx.translate(0.0, half * 0.76);
        ctx.set_line_width(half * 0.03);
        ctx.arc(0.0, 0.0, half * 0.05, 0.0, PI * 2.0);
        ctx.stroke()?;
        ctx.set_line_width(half * 0.02);
        ctx.arc(0.0, 0.0, half * 0.085, 0.0, PI * 2.0);
        ctx.stroke()?;
        Ok(())
    })?;
    ctx.set_line_width(half * 0.022);
    ctx.arc(0.0, 0.0, half * 0.664, 0.0, PI * 2.0);
    ctx.stroke()?;
    radial_repeat(&ctx, 6, 0.0, |ctx| {
        ctx.translate(0.0, half * 0.41);
        ctx.set_line_width(half * 0.022);
        ctx.arc(0.0, 0.0, half * 0.24, 0.0, PI * 2.0);
        ctx.stroke()?;
        ctx.arc(0.0, 0.0, half * 0.1, 0.0, PI * 2.0);
        ctx.stroke()?;
        ctx.set_line_width(half * 0.016);
        ctx.arc(0.0, 0.0, half * 0.2, 0.0, PI);
        ctx.stroke()?;
        ctx.translate(0.0, half * 0.035);
        ctx.set_line_width(half * 0.016);
        isosceles_triangle_stroke(&ctx, -half * 0.11, PI / 3.0)?;
        let line_x = half * 0.16;
        ctx.move_to(0.0, half * 0.06);
        ctx.line_to(0.0, line_x);
        ctx.move_to(0.0, 0.0);
        ctx.line_to(line_x, line_x * (PI / 6.0).tan());
        ctx.move_to(0.0, 0.0);
        ctx.line_to(-line_x, line_x * (PI / 6.0).tan());
        let tangent_x = half * 0.031;
        ctx.translate(tangent_x, -tangent_x / (PI / 6.0).tan());
        ctx.move_to(0.0, 0.0);
        ctx.line_to(line_x, line_x * (PI / 6.0).tan());
        ctx.move_to(0.0, 0.0);
        ctx.rel_line_to(0.0, -half * 0.22);
        ctx.translate(-tangent_x * 2.0, 0.0);
        ctx.move_to(0.0, 0.0);
        ctx.line_to(-line_x, line_x * (PI / 6.0).tan());
        ctx.move_to(0.0, 0.0);
        ctx.rel_line_to(0.0, -half * 0.22);
        ctx.stroke()?;
        Ok(())
    })?;
    radial_repeat(&ctx, 6, PI / 6.0, |ctx| {
        ctx.translate(0.0, half * 0.355);
        ctx.set_line_width(half * 0.015);
        ctx.arc(0.0, 0.0, half * 0.035, 0.0, PI * 2.0);
        ctx.stroke()?;
        ctx.arc(0.0, 0.0, half * 0.11, 0.0, PI * 2.0);
        ctx.stroke()?;
        Ok(())
    })?;

    Ok(())
}
