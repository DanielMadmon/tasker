
fn main() {
    let res = slint_build::compile("bin/tasker_gui/src/main_window.slint");
     if let Err(error) = res {
        println!("erorr:++++{:?}", error);
    }
}