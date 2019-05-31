use crate::commands::{JoshutoCommand, JoshutoRunnable};
use crate::context::JoshutoContext;
use crate::error::JoshutoError;
use crate::structs::JoshutoDirList;
use crate::window::JoshutoView;

use std::collections::hash_map::Entry;

#[derive(Clone, Debug)]
pub struct ReloadDirList;

impl ReloadDirList {
    pub fn new() -> Self {
        ReloadDirList
    }
    pub const fn command() -> &'static str {
        "reload_dir_list"
    }

    pub fn reload(
        index: usize,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), std::io::Error> {
        let curr_tab = &mut context.tabs[index];
        let sort_option = &context.config_t.sort_option;
        curr_tab.curr_list.update_contents(sort_option)?;
        if let Some(s) = curr_tab.curr_list.index {
            let dir_len = curr_tab.curr_list.contents.len();
            curr_tab.curr_list.pagestate.update_page_state(
                s,
                view.mid_win.rows,
                dir_len,
                context.config_t.scroll_offset,
            );
            curr_tab.curr_list.outdated = false;
        }

        if let Some(parent) = curr_tab.curr_list.path.parent() {
            match curr_tab.history.entry(parent.to_path_buf().clone()) {
                Entry::Occupied(mut entry) => {
                    let dirlist = entry.get_mut();
                    dirlist.update_contents(sort_option)?;
                    if let Some(s) = dirlist.index {
                        let dir_len = dirlist.contents.len();
                        dirlist.pagestate.update_page_state(
                            s,
                            view.mid_win.rows,
                            dir_len,
                            context.config_t.scroll_offset,
                        );
                        dirlist.outdated = false;
                    }
                }
                Entry::Vacant(entry) => {
                    let s = JoshutoDirList::new(parent.to_path_buf().clone(), sort_option)?;
                    entry.insert(s);
                }
            }
        }
        Ok(())
    }
}

impl JoshutoCommand for ReloadDirList {}

impl std::fmt::Display for ReloadDirList {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(Self::command())
    }
}

impl JoshutoRunnable for ReloadDirList {
    fn execute(
        &self,
        context: &mut JoshutoContext,
        view: &JoshutoView,
    ) -> Result<(), JoshutoError> {
        match Self::reload(context.curr_tab_index, context, view) {
            Ok(_) => {
                let curr_tab = &mut context.tabs[context.curr_tab_index];
                curr_tab.refresh(view, &context.config_t);
                ncurses::doupdate();
                Ok(())
            }
            Err(e) => Err(JoshutoError::IO(e)),
        }
    }
}
