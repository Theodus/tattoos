use cairo::{self, Context, Error};
use std::f64::consts::PI;

type Result = core::result::Result<(), Error>;

fn main() -> Result {
    let full = 1000.0;
    let half = full / 2.0;

    let surface = cairo::SvgSurface::new(full, full, Some("out.svg"))?;
    let ctx = Context::new(&surface)?;
    ctx.translate(half, half);
    ctx.scale(1.0, -1.0);
    ctx.set_source_rgb(0.0, 0.0, 0.0);

    // debug point
    ctx.arc(0.0, 0.0, full / 250.0, 0.0, PI * 2.0);
    ctx.fill()?;

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
