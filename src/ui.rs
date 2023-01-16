extern crate native_windows_gui as nwg;
use crate::qbittorrent::{self, GlobalInfo};
use nwg::TrayNotificationFlags as TNF;

#[derive(Default)]
pub struct SystemTray {
    pub hosts: Vec<qbittorrent::QbtHost>,
    window: nwg::MessageWindow,
    icon: nwg::Icon,
    tray: nwg::TrayNotification,
    tray_menu: nwg::Menu,
    info_item: nwg::MenuItem,
    pause_all_item: nwg::MenuItem,
    resume_all_item: nwg::MenuItem,
    host_items: Vec<nwg::MenuItem>,
    exit_item: nwg::MenuItem,
}

impl SystemTray {
    fn show_menu(&self) {
        let (x, y) = nwg::GlobalCursor::position();
        self.tray_menu.popup(x, y);
    }

    fn info_notification(&self, host: &qbittorrent::QbtHost, info: &GlobalInfo) {
        let flags = TNF::USER_ICON | TNF::LARGE_ICON | TNF::SILENT | TNF::QUIET;
        self.tray.show(
            &format!("{}", info),
            Some(&host.name),
            Some(flags),
            Some(&self.icon),
        );
    }

    fn error_notification(&self, host: &qbittorrent::QbtHost, error: reqwest::Error) {
        let flags = TNF::ERROR_ICON | TNF::LARGE_ICON | TNF::SILENT | TNF::QUIET;
        self.tray.show(
            &format!("{:#?}", error),
            Some(&host.name),
            Some(flags),
            Some(&self.icon),
        );
    }

    fn info(&self, host: &qbittorrent::QbtHost) {
        let info = host.get_info();
        match info {
            Ok(i) => self.info_notification(host, &i),
            Err(e) => self.error_notification(host, e),
        }
    }

    fn pause(&self, host: &qbittorrent::QbtHost) {
        match host.pause() {
            Ok(_) => {
                std::thread::sleep(std::time::Duration::from_secs(5));
                let info = host.get_info();
                match info {
                    Ok(i) => self.info_notification(host, &i),
                    Err(e) => self.error_notification(host, e),
                }
            }
            Err(e) => self.error_notification(host, e),
        }
    }

    fn resume(&self, host: &qbittorrent::QbtHost) {
        match host.resume() {
            Ok(_) => {
                std::thread::sleep(std::time::Duration::from_secs(5));
                let info = host.get_info();
                match info {
                    Ok(i) => self.info_notification(host, &i),
                    Err(e) => self.error_notification(host, e),
                }
            }
            Err(e) => self.error_notification(host, e),
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

//
// ALL of this stuff is handled by native-windows-derive
//
mod system_tray_ui {
    use super::*;
    use native_windows_gui as nwg;
    use nwg::MenuSeparator;
    use std::cell::RefCell;
    use std::ops::Deref;
    use std::rc::Rc;

    pub struct SystemTrayUi {
        inner: Rc<SystemTray>,
        default_handler: RefCell<Vec<nwg::EventHandler>>,
    }

    impl nwg::NativeUi<SystemTrayUi> for SystemTray {
        fn build_ui(mut data: SystemTray) -> Result<SystemTrayUi, nwg::NwgError> {
            use nwg::Event as E;

            let ico = include_bytes!("../assets/thonkers.ico");
            // Resources
            nwg::Icon::builder()
                .source_bin(Some(ico))
                .build(&mut data.icon)?;

            // Controls
            nwg::MessageWindow::builder().build(&mut data.window)?;
            nwg::TrayNotification::builder()
                .parent(&data.window)
                .icon(Some(&data.icon))
                .tip(Some("QBT Controller"))
                .realtime(true)
                .build(&mut data.tray)?;

            nwg::Menu::builder()
                .popup(true)
                .parent(&data.window)
                .build(&mut data.tray_menu)?;

            nwg::MenuItem::builder()
                .text("Info")
                .parent(&data.tray_menu)
                .build(&mut data.info_item)?;

            nwg::MenuItem::builder()
                .text("Pause All")
                .parent(&data.tray_menu)
                .build(&mut data.pause_all_item)?;

            nwg::MenuItem::builder()
                .text("Resume All")
                .parent(&data.tray_menu)
                .build(&mut data.resume_all_item)?;

            let mut sep: MenuSeparator = Default::default();
            nwg::MenuSeparator::builder()
                .parent(&data.tray_menu)
                .build(&mut sep)?;

            for i in &data.hosts {
                let mut item: nwg::Menu = Default::default();
                let mut item_pause: nwg::MenuItem = Default::default();
                let mut item_resume: nwg::MenuItem = Default::default();

                nwg::Menu::builder()
                    .text(&i.name)
                    .parent(&mut data.tray_menu)
                    .build(&mut item)?;

                nwg::MenuItem::builder()
                    .text("Pause")
                    .parent(&mut item)
                    .build(&mut item_pause)?;

                nwg::MenuItem::builder()
                    .text("Resume")
                    .parent(&mut item)
                    .build(&mut item_resume)?;

                data.host_items.push(item_pause);
                data.host_items.push(item_resume);
            }

            nwg::MenuItem::builder()
                .parent(&data.window)
                .text("Exit")
                .parent(&data.tray_menu)
                .build(&mut data.exit_item)?;

            // Wrap-up
            let ui = SystemTrayUi {
                inner: Rc::new(data),
                default_handler: Default::default(),
            };

            // Events
            let evt_ui = Rc::downgrade(&ui.inner);
            let handle_events = move |evt, _evt_data, handle| {
                if let Some(evt_ui) = evt_ui.upgrade() {
                    match evt {
                        E::OnContextMenu => {
                            if &handle == &evt_ui.tray {
                                SystemTray::show_menu(&evt_ui);
                            }
                        }
                        E::OnMenuItemSelected => {
                            if &handle == &evt_ui.info_item {
                                for i in &evt_ui.hosts {
                                    evt_ui.info(i);
                                }
                            } else if &handle == &evt_ui.pause_all_item {
                                for i in &evt_ui.hosts {
                                    evt_ui.pause(i);
                                }
                            } else if &handle == &evt_ui.resume_all_item {
                                for i in &evt_ui.hosts {
                                    evt_ui.resume(i);
                                }
                            } else if &handle == &evt_ui.exit_item {
                                SystemTray::exit(&evt_ui);
                            } else {
                                for i in 0..evt_ui.host_items.len() {
                                    if &handle == &evt_ui.host_items[i] {
                                        println!("{:#?}", evt_ui.hosts[i]);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            };

            ui.default_handler
                .borrow_mut()
                .push(nwg::full_bind_event_handler(
                    &ui.window.handle,
                    handle_events,
                ));

            return Ok(ui);
        }
    }

    impl Drop for SystemTrayUi {
        /// To make sure that everything is freed without issues, the default handler must be unbound.
        fn drop(&mut self) {
            let mut handlers = self.default_handler.borrow_mut();
            for handler in handlers.drain(0..) {
                nwg::unbind_event_handler(&handler);
            }
        }
    }

    impl Deref for SystemTrayUi {
        type Target = SystemTray;

        fn deref(&self) -> &SystemTray {
            &self.inner
        }
    }
}
