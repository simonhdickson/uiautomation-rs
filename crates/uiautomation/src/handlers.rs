use windows::{
    core::implement,
    Win32::UI::Accessibility::{
        IUIAutomationElement, IUIAutomationEventHandler, IUIAutomationEventHandler_Impl,
        UIA_EVENT_ID,
    },
};

use crate::UIElement;

pub enum EventId {
    ActiveTextPositionChangedEventId,
    AsyncContentLoadedEventId,
    AutomationFocusChangedEventId,
    AutomationPropertyChangedEventId,
    ChangesEventId,
    DragDragCancelEventId,
    DragDragCompleteEventId,
    DragDragStartEventId,
    DropTargetDragEnterEventId,
    DropTargetDragLeaveEventId,
    DropTargetDroppedEventId,
    HostedFragmentRootsInvalidatedEventId,
    InputDiscardedEventId,
    InputReachedOtherElementEventId,
    InputReachedTargetEventId,
    InvokeInvokedEventId,
    LayoutInvalidatedEventId,
    LiveRegionChangedEventId,
    MenuClosedEventId,
    MenuModeEndEventId,
    MenuModeStartEventId,
    MenuOpenedEventId,
    NotificationEventId,
    SelectionItemElementAddedToSelectionEventId,
    SelectionItemElementRemovedFromSelectionEventId,
    SelectionItemElementSelectedEventId,
    SelectionInvalidatedEventId,
    StructureChangedEventId,
    SystemAlertEventId,
    TextEditConversionTargetChangedEventId,
    TextEditTextChangedEventId,
    TextTextChangedEventId,
    TextTextSelectionChangedEventId,
    ToolTipClosedEventId,
    ToolTipOpenedEventId,
    WindowWindowClosedEventId,
    WindowWindowOpenedEventId,
}

impl From<EventId> for UIA_EVENT_ID {
    fn from(value: EventId) -> Self {
        use windows::Win32::UI::Accessibility::*;

        match value {
            EventId::ActiveTextPositionChangedEventId => UIA_ActiveTextPositionChangedEventId,
            EventId::AsyncContentLoadedEventId => UIA_AsyncContentLoadedEventId,
            EventId::AutomationFocusChangedEventId => UIA_AutomationFocusChangedEventId,
            EventId::AutomationPropertyChangedEventId => UIA_AutomationPropertyChangedEventId,
            EventId::ChangesEventId => UIA_ChangesEventId,
            EventId::DragDragCancelEventId => UIA_Drag_DragCancelEventId,
            EventId::DragDragCompleteEventId => UIA_Drag_DragCompleteEventId,
            EventId::DragDragStartEventId => UIA_Drag_DragStartEventId,
            EventId::DropTargetDragEnterEventId => UIA_DropTarget_DragEnterEventId,
            EventId::DropTargetDragLeaveEventId => UIA_DropTarget_DragLeaveEventId,
            EventId::DropTargetDroppedEventId => UIA_DropTarget_DroppedEventId,
            EventId::HostedFragmentRootsInvalidatedEventId => {
                UIA_HostedFragmentRootsInvalidatedEventId
            }
            EventId::InputDiscardedEventId => UIA_InputDiscardedEventId,
            EventId::InputReachedOtherElementEventId => UIA_InputReachedOtherElementEventId,
            EventId::InputReachedTargetEventId => UIA_InputReachedTargetEventId,
            EventId::InvokeInvokedEventId => UIA_Invoke_InvokedEventId,
            EventId::LayoutInvalidatedEventId => UIA_LayoutInvalidatedEventId,
            EventId::LiveRegionChangedEventId => UIA_LiveRegionChangedEventId,
            EventId::MenuClosedEventId => UIA_MenuClosedEventId,
            EventId::MenuModeEndEventId => UIA_MenuModeEndEventId,
            EventId::MenuModeStartEventId => UIA_MenuModeStartEventId,
            EventId::MenuOpenedEventId => UIA_MenuOpenedEventId,
            EventId::NotificationEventId => UIA_NotificationEventId,
            EventId::SelectionItemElementAddedToSelectionEventId => {
                UIA_SelectionItem_ElementAddedToSelectionEventId
            }
            EventId::SelectionItemElementRemovedFromSelectionEventId => {
                UIA_SelectionItem_ElementRemovedFromSelectionEventId
            }
            EventId::SelectionItemElementSelectedEventId => {
                UIA_SelectionItem_ElementSelectedEventId
            }
            EventId::SelectionInvalidatedEventId => UIA_Selection_InvalidatedEventId,
            EventId::StructureChangedEventId => UIA_StructureChangedEventId,
            EventId::SystemAlertEventId => UIA_SystemAlertEventId,
            EventId::TextEditConversionTargetChangedEventId => {
                UIA_TextEdit_ConversionTargetChangedEventId
            }
            EventId::TextEditTextChangedEventId => UIA_TextEdit_TextChangedEventId,
            EventId::TextTextChangedEventId => UIA_Text_TextChangedEventId,
            EventId::TextTextSelectionChangedEventId => UIA_Text_TextSelectionChangedEventId,
            EventId::ToolTipClosedEventId => UIA_ToolTipClosedEventId,
            EventId::ToolTipOpenedEventId => UIA_ToolTipOpenedEventId,
            EventId::WindowWindowClosedEventId => UIA_Window_WindowClosedEventId,
            EventId::WindowWindowOpenedEventId => UIA_Window_WindowOpenedEventId,
        }
    }
}

#[implement(IUIAutomationEventHandler)]
pub struct AutomationEventHandler<CB>
where
    CB: Fn(UIElement) -> () + 'static,
{
    cb: Box<CB>,
}

impl<CB> AutomationEventHandler<CB>
where
    CB: Fn(UIElement) -> () + 'static,
{
    pub fn new(func: CB) -> Self {
        Self { cb: func.into() }
    }
}

impl<CB> IUIAutomationEventHandler_Impl for AutomationEventHandler<CB>
where
    CB: Fn(UIElement) -> () + 'static,
{
    #[allow(non_snake_case)]
    #[allow(unused_variables)]
    fn HandleAutomationEvent(
        &self,
        sender: Option<&IUIAutomationElement>,
        event_id: UIA_EVENT_ID,
    ) -> windows_core::Result<()> {
        let func = &*self.cb;
        func(sender.unwrap().clone().into());
        Ok(())
    }
}

pub struct AutomationEventHandle {
    pub(crate) handler: IUIAutomationEventHandler,
}
