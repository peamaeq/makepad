use makepad_render::*;
use crate::scrollbar::*;

#[derive(Debug, Clone)]
pub struct ScrollView{
    pub view:View,
    pub scroll_h:Option<ScrollBar>,
    pub scroll_v:Option<ScrollBar>,
}

impl ScrollView{

    pub fn new() -> Self {
        Self {
            view: View::new(),
            scroll_h: None,
            scroll_v: None
        }
    }

    pub fn new_standard_hv(cx: &mut Cx) -> Self {
        Self {
            view: View::new(),
            scroll_h: Some(ScrollBar::new(cx)),
            scroll_v: Some(ScrollBar::new(cx)
                .with_smoothing(0.15)),
        }
    }
   
    pub fn with_scroll_h(self, s:ScrollBar)->Self{
        Self{scroll_h:Some(s), ..self}
    }
      
    pub fn with_scroll_v(self, s:ScrollBar)->Self{
        Self{scroll_v:Some(s), ..self}
    }
   
    pub fn draw_scroll_view<F>(&mut self, cx: &mut Cx, layout: Layout, f: F)
    where
        F: FnOnce(&mut Cx)
    {
        if self.begin_view(cx, layout).is_ok() {
            f(cx);
            self.end_view(cx);
        }
    }

    pub fn begin_view(&mut self, cx: &mut Cx, layout: Layout) -> ViewRedraw {
        self.view.begin_view(cx, layout)
    }
    
    pub fn view_will_redraw(&mut self, cx: &mut Cx)->bool{
        self.view.view_will_redraw(cx)
    }
    
    pub fn handle_scroll_view(&mut self, cx: &mut Cx, event: &mut Event) -> bool {
        let mut ret_h = ScrollBarEvent::None;
        let mut ret_v = ScrollBarEvent::None;
        
        if let Some(scroll_h) = &mut self.scroll_h {
            ret_h = scroll_h.handle_scroll_bar(cx, event);
        }
        if let Some(scroll_v) = &mut self.scroll_v {
            ret_v = scroll_v.handle_scroll_bar(cx, event);
        }
        if let Some(view_id) = self.view.view_id{
            match ret_h {
                ScrollBarEvent::None => (),
                ScrollBarEvent::Scroll {scroll_pos, ..} => {
                    cx.set_view_scroll_x(view_id, scroll_pos);
                },
                _ => ()
            };
            match ret_v {
                ScrollBarEvent::None => (),
                ScrollBarEvent::Scroll {scroll_pos, ..} => {
                    cx.set_view_scroll_y(view_id, scroll_pos);
                },
                _ => ()
            };
            ret_h != ScrollBarEvent::None || ret_v != ScrollBarEvent::None
        }
        else{
            false
        }
    }
    
    pub fn get_scroll_pos(&self, cx: &Cx) -> Vec2 {
        if let Some(view_id) = self.view.view_id {
            let cxview = &cx.views[view_id];
            cxview.unsnapped_scroll
        }
        else {
            Vec2::default()
        }
    }
    
    pub fn set_scroll_pos(&mut self, cx: &mut Cx, pos: Vec2) -> bool {
        let view_id = self.view.view_id.unwrap();
        //let view_area = Area::DrawList(DrawListArea{draw_list_id:draw_list_id, redraw_id:cx.redraw_id});
        let mut changed = false;
        if let Some(scroll_h) = &mut self.scroll_h {
            if scroll_h.set_scroll_pos(cx, pos.x) {
                let scroll_pos = scroll_h.get_scroll_pos();
                cx.set_view_scroll_x(view_id, scroll_pos);
                changed = true;
            }
        }
        if let Some(scroll_v) = &mut self.scroll_v {
            if scroll_v.set_scroll_pos(cx, pos.y) {
                let scroll_pos = scroll_v.get_scroll_pos();
                cx.set_view_scroll_y(view_id, scroll_pos);
                changed = true;
            }
        }
        changed
    }
    
    pub fn set_scroll_view_total(&mut self, cx: &mut Cx, view_total: Vec2) {
        if let Some(scroll_h) = &mut self.scroll_h {
            scroll_h.set_scroll_view_total(cx, view_total.x)
        }
        if let Some(scroll_v) = &mut self.scroll_v {
            scroll_v.set_scroll_view_total(cx, view_total.y)
        }
    }
    
    pub fn get_scroll_view_total(&mut self) -> Vec2 {
        Vec2 {
            x: if let Some(scroll_h) = &mut self.scroll_h {
                scroll_h.get_scroll_view_total()
            }else {0.},
            y: if let Some(scroll_v) = &mut self.scroll_v {
                scroll_v.get_scroll_view_total()
            }else {0.}
        }
    }
    
    pub fn scroll_into_view(&mut self, cx: &mut Cx, rect: Rect) {
        if let Some(scroll_h) = &mut self.scroll_h {
            scroll_h.scroll_into_view(cx, rect.pos.x, rect.size.x, true);
        }
        if let Some(scroll_v) = &mut self.scroll_v {
            scroll_v.scroll_into_view(cx, rect.pos.y, rect.size.y, true);
        }
    }
    
    pub fn scroll_into_view_no_smooth(&mut self, cx: &mut Cx, rect: Rect) {
        if let Some(scroll_h) = &mut self.scroll_h {
            scroll_h.scroll_into_view(cx, rect.pos.x, rect.size.x, false);
        }
        if let Some(scroll_v) = &mut self.scroll_v {
            scroll_v.scroll_into_view(cx, rect.pos.y, rect.size.y, false);
        }
    }
    
    pub fn scroll_into_view_abs(&mut self, cx: &mut Cx, rect: Rect) {
        let self_rect = self.get_rect(cx); 
        if let Some(scroll_h) = &mut self.scroll_h {
            scroll_h.scroll_into_view(cx, rect.pos.x - self_rect.pos.x, rect.size.x, true);
        }
        if let Some(scroll_v) = &mut self.scroll_v {
            scroll_v.scroll_into_view(cx, rect.pos.y  - self_rect.pos.y, rect.size.y, true);
        }
    }
    
    pub fn set_scroll_target(&mut self, cx: &mut Cx, pos: Vec2) {
        if let Some(scroll_h) = &mut self.scroll_h {
            scroll_h.set_scroll_target(cx, pos.x);
        }
        if let Some(scroll_v) = &mut self.scroll_v {
            scroll_v.set_scroll_target(cx, pos.y);
        }
    }
    
    pub fn end_view(&mut self, cx: &mut Cx) -> Area {

        let view_id = self.view.view_id.unwrap();
        let view_area = Area::View(ViewArea {view_id: view_id, redraw_id: cx.redraw_id});
        
        // lets ask the turtle our actual bounds
        let view_total = cx.get_turtle_bounds();
        let mut rect_now = cx.get_turtle_rect();
        if rect_now.size.y.is_nan() {
            rect_now.size.y = view_total.y;
        }
        if rect_now.size.x.is_nan() {
            rect_now.size.x = view_total.x;
        }
        
        if let Some(scroll_h) = &mut self.scroll_h {
            let scroll_pos = scroll_h.draw_scroll_bar(cx, Axis::Horizontal, view_area, rect_now, view_total);
            cx.set_view_scroll_x(view_id, scroll_pos);
        }
        if let Some(scroll_v) = &mut self.scroll_v {
            //println!("SET SCROLLBAR {} {}", rect_now.h, view_total.y);
            let scroll_pos = scroll_v.draw_scroll_bar(cx, Axis::Vertical, view_area, rect_now, view_total);
            cx.set_view_scroll_y(view_id, scroll_pos);
        }
        
        let rect = cx.end_turtle(view_area);
        let cxview = &mut cx.views[view_id];
        cxview.rect = rect;
        cx.view_stack.pop();
        
        return view_area
    }
    
    pub fn get_rect(&mut self, cx: &Cx) -> Rect {
        self.view.get_rect(cx)
    }
    
    
    pub fn redraw_view(&self, cx: &mut Cx) {
        self.view.redraw_view(cx)
    }
    
    pub fn area(&self) -> Area {
        self.view.area()
    }
}
