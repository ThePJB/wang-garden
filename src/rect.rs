use crate::kmath::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect{x,y,w,h}
    }
    pub fn child(&self, x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect::new(
            self.x + x*self.w,
            self.y + y*self.h,
            self.w * w,
            self.h * h,
        )
    }
    pub fn contains(&self, p: Vec2) -> bool {
        p.x >= self.left() && p.x <= self.right() && p.y >= self.top() && p.y <= self.bot()
    }
    pub fn relative_pos(&self, p: Vec2) -> Option<Vec2> {
        if self.contains(p) {
            println!("p x: {} self x: {} self w: {}", p.x, self.x, self.w);
            Some(Vec2::new((p.x - self.x) / self.w, (p.y - self.y) / self.h))
        } else {
            None
        }
    }
    pub fn centroid(&self) -> Vec2 {
        Vec2::new(self.x + self.w/2.0, self.y + self.h/2.0)
    }
    pub fn new_centered(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect::new(x-w/2.0, y-h/2.0, w, h)
    }
    pub fn child_with_aspect_ratio(&self, a: f32) -> Rect {
        let self_a = self.w/self.h;
        if a > self_a {
            // want one thats wider than us
            // so width will be our full width
            // if its a is twice ours height will be half I suppose
            let height = self_a / a;
            self.child(0.0, (1.0 - height) / 2.0, 1.0, height)
        } else {
            let width = a / self_a;
            self.child((1.0 - width) / 2.0, 0.0, width, 1.0)
        }
    }
    pub fn translate(&self, v: Vec2) -> Rect {
        return Rect::new(self.x + v.x, self.y + v.y, self.w, self.h);
    }
    pub fn dilate(&self, d: f32) -> Rect {
        return Rect::new(self.x - d, self.y - d, self.w + 2.0*d, self.h + 2.0*d);
    }
    pub fn left(self) -> f32 {
        self.x
    }
    pub fn right(self) -> f32 {
        self.x + self.w
    }
    pub fn top(self) -> f32 {
        self.y
    }
    pub fn bot(self) -> f32 {
        self.y + self.h
    }
}

#[test]
fn test_child_aspect() {
    let r = Rect::new(0.0, 0.0, 1.0, 1.0);
    
    assert_eq!(Rect::new(0.0, 0.0, 1.0, 1.0).child_with_aspect_ratio(1.0), Rect::new(0.0, 0.0, 1.0, 1.0));
    assert_eq!(Rect::new(0.0, 0.0, 1.0, 1.0).child_with_aspect_ratio(2.0), Rect::new(0.0, 0.25, 1.0, 0.5));
    assert_eq!(Rect::new(0.0, 0.0, 1.0, 1.0).child_with_aspect_ratio(0.5), Rect::new(0.25, 0.0, 0.5, 1.0));
    
    assert_eq!(Rect::new(1.0, 1.0, 1.0, 1.0).child_with_aspect_ratio(1.0), Rect::new(1.0, 1.0, 1.0, 1.0));
    assert_eq!(Rect::new(1.0, 1.0, 1.0, 1.0).child_with_aspect_ratio(2.0), Rect::new(1.0, 1.25, 1.0, 0.5));
    assert_eq!(Rect::new(1.0, 1.0, 1.0, 1.0).child_with_aspect_ratio(0.5), Rect::new(1.25,1.0, 0.5, 1.0));
}