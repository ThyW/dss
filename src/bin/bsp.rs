extern crate dss;

use dss::data_structures::bsptree::{BSPTree, MoveDirection, Rectangle};

use std::cell::RefCell;
use std::rc::Rc;

use std::os::raw::c_double;
use tcl::*;
use tk::{cmd::*, *};

fn convert(rect: Rectangle) -> ((f64, f64), (f64, f64)) {
    let (x, y) = (rect.x as f32, rect.y as f32);
    let (nx, ny) = (x + rect.w as f32, y + rect.h as f32);

    ((x as f64, y as f64), (nx as f64, ny as f64))
}

fn draw_tree(tree: Rc<RefCell<BSPTree>>, c: TkCanvas<impl Fn() + Copy + 'static>) -> TkResult<()> {
    let nodes = tree.borrow_mut().walk();
    c.create_rectangle(0., 0., 640., 480., -fill("white"))?;
    for node in nodes {
        let n = node.borrow();
        let rect = n.get_rect();
        if n.get_data().is_some() {
            let (fst, snd) = convert(rect);
            if n.is_focused() {
                c.create_rectangle(fst.0, fst.1, snd.0, snd.1, -fill("green"))?;
            } else {
                c.create_rectangle(fst.0, fst.1, snd.0, snd.1, -fill("red"))?;
            }
        } else {
            let (fst, snd) = convert(rect);
            c.create_rectangle(fst.0, fst.1, snd.0, snd.1, -fill("blue"))?;
        }
    }
    println!("===");
    tree.borrow().print(1);
    println!("===");
    Ok(())
}

fn main() -> TkResult<()> {
    let tree = Rc::new(RefCell::new(BSPTree::new(Rectangle::new(0, 0, 640, 480))));

    let tk = make_tk!()?;
    let root = tk.root();

    let c = root.add_canvas("canvas" - width(640) - height(480) - background("white"))?;
    c.pack(())?;

    let t = tree.clone();

    let count = Rc::new(RefCell::new(0));

    root.bind(
        event::key_press(TkKey::n),
        tclosure!(tk, args: "%x %y", move |_x: c_double, _y: c_double| -> TkResult<()> {
            let co = *count.borrow();
            t.borrow_mut().insert(co + 1);
            draw_tree(t.clone(), c)?;
            count.replace(co + 1);
            Ok(())
        }),
    )?;
    let t = tree.clone();
    root.bind(
        event::key_press(TkKey::d),
        tclosure!(tk, args: "%x %y", move |_x: c_double, _y: c_double| -> TkResult<()> {
            t.borrow_mut().delete_focused();
            draw_tree(t.clone(), c)?;
            Ok(())
        }),
    )?;

    let t = tree.clone();
    root.bind(
        event::key_press(TkKey::s),
        tclosure!(tk, args: "%x %y", move |_x: c_double, _y: c_double| -> TkResult<()> {
            t.borrow_mut().toggle_split();
            draw_tree(t.clone(), c)?;
            Ok(())
        }),
    )?;

    let t = tree.clone();
    root.bind(
        event::button_press_1(),
        tclosure!(tk, args: "%x %y", move |x: c_double, y: c_double| -> TkResult<()> {
            t.borrow_mut().focus_coords(x as i32, y as i32);
            draw_tree(t.clone(), c)?;
            Ok(())
        }),
    )?;
    let t = tree.clone();
    root.bind(
        event::key_press(TkKey::h),
        tclosure!(tk, args: "%x %y", move |_x: c_double, _y: c_double| -> TkResult<()> {
            t.borrow_mut().move_focus(MoveDirection::Left);
            draw_tree(t.clone(), c)?;
            Ok(())
        }),
    )?;
    let t = tree.clone();
    root.bind(
        event::key_press(TkKey::l),
        tclosure!(tk, args: "%x %y", move |_x: c_double, _y: c_double| -> TkResult<()> {
            t.borrow_mut().move_focus(MoveDirection::Right);
            draw_tree(t.clone(), c)?;
            Ok(())
        }),
    )?;
    let t = tree.clone();
    root.bind(
        event::key_press(TkKey::k),
        tclosure!(tk, args: "%x %y", move |_x: c_double, _y: c_double| -> TkResult<()> {
            t.borrow_mut().move_focus(MoveDirection::Up);
            draw_tree(t.clone(), c)?;
            Ok(())
        }),
    )?;
    let t = tree.clone();
    root.bind(
        event::key_press(TkKey::j),
        tclosure!(tk, args: "%x %y", move |_x: c_double, _y: c_double| -> TkResult<()> {
            t.borrow_mut().move_focus(MoveDirection::Down);
            draw_tree(t.clone(), c)?;
            Ok(())
        }),
    )?;

    tree.borrow_mut().delete_focused();

    draw_tree(tree.clone(), c)?;

    main_loop();

    tree.borrow().print(1);

    Ok(())
}
