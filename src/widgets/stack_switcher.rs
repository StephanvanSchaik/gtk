// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! A controller for GtkStack

use ffi;
use cast::{GTK_STACK_SWITCHER, GTK_STACK};
use FFIWidget;

/// GtkStackSwitcher — A controller for GtkStack
struct_Widget!(StackSwitcher);

pub trait StackSwitcherBuilder {
    fn stack_switcher(&self) -> Option<StackSwitcher>;
}

impl StackSwitcherBuilder for ::Gtk {
    fn stack_switcher(&self) -> Option<StackSwitcher> {
        let tmp_pointer = unsafe { ffi::gtk_stack_switcher_new() };
        check_pointer!(tmp_pointer, StackSwitcher)
    }
}

impl StackSwitcher {
    pub fn set_stack(&mut self, stack: ::Stack) {
        unsafe {
            ffi::gtk_stack_switcher_set_stack(GTK_STACK_SWITCHER(self.pointer),
                                              GTK_STACK(stack.unwrap_widget()))
        }
    }

    pub fn get_stack(&self) -> Option<::Stack> {
        let tmp_pointer = unsafe { ffi::gtk_stack_switcher_get_stack(GTK_STACK_SWITCHER(self.pointer)) };
        if tmp_pointer.is_null() {
            None
        } else {
            Some(::FFIWidget::wrap_widget(tmp_pointer))
        }
    }
}

impl_drop!(StackSwitcher);
impl_TraitWidget!(StackSwitcher);

impl ::ContainerTrait for StackSwitcher {}
impl ::BoxTrait for StackSwitcher {}

impl_widget_events!(StackSwitcher);
