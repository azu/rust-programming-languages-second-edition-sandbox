pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<Draw>>,
}

// Screenはcomponentをdrawする
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// component
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        // 実際にボタンを描画するコード
    }
}