use gnuplot::*;

pub fn chart_ex_hy(ex: &[f64], hy: &[f64], tick: &f64) {
    let x_axis_label = format!("FDTD cells, tick = {0}", tick);
    let mut figure = Figure::new();
    figure.axes2d()
        .set_x_label(&x_axis_label, &[])
        .set_y_label("Ex", &[])
        .set_y_range(AutoOption::Fix(-1.125),AutoOption::Fix(1.125))
        .set_size(1.0, 0.50)
        .set_pos(0.0,0.5)
        .set_y_ticks(Some((Fix(1.0), 0)), &[Mirror(false)], &[])
        .set_x_ticks(Some((Fix(20.0), 0)), &[Mirror(true)], &[])
        .lines(0..200, ex.iter(), &[Color("black")]);

    // Produce the gnuplot chart of the Hy pulse.
    figure.axes2d()
        .set_x_label(&x_axis_label, &[])
        .set_y_label("Hy", &[])
        .set_y_range(AutoOption::Fix(-1.125),AutoOption::Fix(1.125))
        .set_size(1.0, 0.50)
        .set_y_ticks(Some((Fix(1.0), 0)), &[Mirror(false)], &[])
        .set_x_ticks(Some((Fix(20.0), 0)), &[Mirror(true)], &[])
        .lines(0..200, hy.iter(), &[Color("black")]);
    figure.show();
}
