include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub unsafe fn draw_rect(x: f32, y: f32, w: f32, h: f32, r: f32, g: f32, b: f32) {
    glDisable(GL_BLEND);
    glDisable(GL_TEXTURE_2D);

    glColor3f(r, g, b);
    glLineWidth(1.0);

    glBegin(GL_LINE_LOOP);
    glVertex2f(x, y);
    glVertex2f(x + w, y);
    glVertex2f(x + w, y + h);
    glVertex2f(x, y + h);
    glEnd();

    glEnable(GL_TEXTURE_2D);
    glEnable(GL_BLEND);
}
