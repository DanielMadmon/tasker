use std::rc::Rc;

use slint::{Model, VecModel, SharedString};


slint::slint!{
    import { App } from "bin/tasker_gui/src/main_window.slint";
}
pub fn main(){ 
    let app = App::new().unwrap();
    let mut rows:Vec<TableRow> = app.get_table().iter().collect();
    rows.extend(rows.clone());
    let new_row = TableRow{
        name:SharedString::from("test1")
    };
    rows.push(new_row);
    let new_row = TableRow{
        name:SharedString::from("test2")
    };
    rows.push(new_row);

    let table_model: Rc<VecModel<TableRow>> = Rc::new(VecModel::from(rows));
    app.set_table(table_model.into());
    app.run().unwrap();
}
