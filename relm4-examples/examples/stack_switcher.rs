use gtk::prelude::{BoxExt, GtkWindowExt, WidgetExt};
use relm4::{gtk, AppUpdate, ComponentUpdate, Model, RelmApp, RelmComponent, Sender, Widgets};

enum HeaderMsg {}

struct HeaderModel {}

impl Model for HeaderModel {
    type Msg = HeaderMsg;
    type Widgets = HeaderWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for HeaderModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        HeaderModel {}
    }

    fn update(
        &mut self,
        _msg: HeaderMsg,
        _components: &(),
        _sender: Sender<HeaderMsg>,
        _parent_sender: Sender<AppMsg>,
    ) {
    }
}

#[relm4::widget]
impl Widgets<HeaderModel, AppModel> for HeaderWidgets {
    view! {
        gtk::HeaderBar {
            set_title_widget = Some(&gtk::Box) {
                add_css_class: "linked",
                append: switcher = &gtk::StackSwitcher {}
            }
        }
    }
}

struct StackModel {}

enum StackMsg {}

impl Model for StackModel {
    type Msg = StackMsg;
    type Widgets = StackWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for StackModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        StackModel {}
    }

    fn update(
        &mut self,
        _msg: StackMsg,
        _components: &(),
        _sender: Sender<StackMsg>,
        _parent_sender: Sender<AppMsg>,
    ) {
    }
}

#[relm4::widget]
impl Widgets<StackModel, AppModel> for StackWidgets {
    view! {
        gtk::Stack {
            add_titled(Some("First"), "First") = &gtk::Label {
                set_label: "First page"
            },
            add_titled(Some("Second"), "Second") = &gtk::Label {
                set_label: "Second page"
            }
        }
    }
}

#[derive(relm4::Components)]
struct AppComponents {
    header: RelmComponent<HeaderModel, AppModel>,
    stack: RelmComponent<StackModel, AppModel>,
}

#[derive(Debug)]
enum AppMode {}

enum AppMsg {}

struct AppModel {}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = gtk::ApplicationWindow {
            set_default_width: 500,
            set_default_height: 250,
            set_titlebar: Some(components.header.root_widget()),
            set_child: Some(components.stack.root_widget()),
        }
    }

    fn post_init() {
        let header_widgets = components.header.widgets().unwrap();
        header_widgets
            .switcher
            .set_stack(Some(components.stack.root_widget()));
    }
}

impl AppUpdate for AppModel {
    fn update(
        &mut self,
        _msg: AppMsg,
        _components: &AppComponents,
        _sender: Sender<AppMsg>,
    ) -> bool {
        true
    }
}

fn main() {
    let model = AppModel {};
    let relm = RelmApp::new(model);
    relm.run();
}
