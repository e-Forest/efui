use efui::*;
#[macroquad::main("EfuiTester")]
async fn main() {
    let mut iam = InterActorManager::default();
    let mut f = Frame::new(Rect::new(10., 10., 200., 100.), &mut iam);
    loop {
        clear_background(WHITE);
        f.update(&mut iam);
        iam.update();
        f.draw();
        iam.draw();

        next_frame().await;
    }
}
