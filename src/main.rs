use nannou::prelude::*;

fn main() {
    // sketchは、単純なドローイングに適した命令
    // 素早く描画を開始できる
    nannou::sketch(view).run();
    // 時間やキーボードなどの入力には反応することができる

    // なお、上記は以下の書き方と同義
    // nannou::app(model).simple_window(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.to_frame(app, &frame).unwrap();
}
