use std::{
    rc::Rc,
    time::{Duration, Instant},
};

use gpui::{
    AnyView, DismissEvent, Entity, EntityId, EventEmitter, FocusHandle, Focusable, ManagedView,
    MouseButton, Subscription, Task,
};
use ui::{Tooltip, animation::DefaultAnimations, prelude::*};
use sotid_actions::toast;

use crate::Workspace;

const DEFAULT_TOAST_DURATION: Duration = Duration::from_secs(10);
const MINIMUM_RESUME_DURATION: Duration = Duration::from_millis(800);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _window, _cx| {
        workspace.register_action(|_workspace, _: &toast::RunAction, window, cx| {
            let workspace = cx.entity();
            let window = window.window_handle();
            cx.defer(move |cx| {
                let action = workspace
                    .read(cx)
                    .toast_layer
                    .read(cx)
                    .active_toast
                    .as_ref()
                    .and_then(|active_toast| active_toast.action.clone());

                if let Some(on_click) = action.and_then(|action| action.on_click) {
                    window
                        .update(cx, |_, window, cx| {
                            on_click(window, cx);
                        })
                        .ok();
                }
            });
        });
    })
    .detach();
}

pub trait ToastView: ManagedView {
    fn action(&self) -> Option<ToastAction>;

    fn auto_dismiss(&self) -> bool {
        true
    }
}

#[derive(Clone)]
pub struct ToastAction {
    pub id: ElementId,
    pub label: SharedString,
    pub on_click: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl ToastAction {
    pub fn new(
        label: SharedString,
        on_click: Option<Rc<dyn Fn(&mut Window, &mut App) + 'static>>,
    ) -> Self {
        let id = ElementId::Name(label.clone());

        Self {
            id,
            label,
            on_click,
        }
    }
}

trait ToastViewHandle {
    fn view(&self) -> AnyView;
}

impl<V: ToastView> ToastViewHandle for Entity<V> {
    fn view(&self) -> AnyView {
        self.clone().into()
    }
}

pub struct ActiveToast {
    id: EntityId,
    toast: Box<dyn ToastViewHandle>,
    action: Option<ToastAction>,
    _subscriptions: [Subscription; 1],
    focus_handle: FocusHandle,
}

struct DismissTimer {
    instant_started: Instant,
    _task: Task<()>,
}

pub struct ToastLayer {
    active_toast: Option<ActiveToast>,
    duration_remaining: Option<Duration>,
    dismiss_timer: Option<DismissTimer>,
}

impl Default for ToastLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl ToastLayer {
    pub fn new() -> Self {
        Self {
            active_toast: None,
            duration_remaining: None,
            dismiss_timer: None,
        }
    }

    pub fn toggle_toast<V>(&mut self, cx: &mut Context<Self>, new_toast: Entity<V>)
    where
        V: ToastView,
    {
        if let Some(active_toast) = &self.active_toast {
            let show_new = active_toast.id != new_toast.entity_id();
            self.hide_toast(cx);
            if !show_new {
                return;
            }
        }
        self.show_toast(new_toast, cx);
    }

    pub fn show_toast<V>(&mut self, new_toast: Entity<V>, cx: &mut Context<Self>)
    where
        V: ToastView,
    {
        let action = new_toast.read(cx).action();
        let auto_dismiss = new_toast.read(cx).auto_dismiss();
        let focus_handle = cx.focus_handle();

        self.active_toast = Some(ActiveToast {
            _subscriptions: [cx.subscribe(&new_toast, |this, _, _: &DismissEvent, cx| {
                this.hide_toast(cx);
            })],
            id: new_toast.entity_id(),
            toast: Box::new(new_toast),
            action,
            focus_handle,
        });

        if auto_dismiss {
            self.start_dismiss_timer(DEFAULT_TOAST_DURATION, cx);
        }

        cx.notify();
    }

    pub fn hide_toast(&mut self, cx: &mut Context<Self>) {
        self.active_toast.take();
        cx.notify();
    }

    pub fn active_toast<V>(&self) -> Option<Entity<V>>
    where
        V: 'static,
    {
        let active_toast = self.active_toast.as_ref()?;
        active_toast.toast.view().downcast::<V>().ok()
    }

    pub fn has_active_toast(&self) -> bool {
        self.active_toast.is_some()
    }

    fn pause_dismiss_timer(&mut self) {
        let Some(dismiss_timer) = self.dismiss_timer.take() else {
            return;
        };
        let Some(duration_remaining) = self.duration_remaining.as_mut() else {
            return;
        };
        *duration_remaining =
            duration_remaining.saturating_sub(dismiss_timer.instant_started.elapsed());
        if *duration_remaining < MINIMUM_RESUME_DURATION {
            *duration_remaining = MINIMUM_RESUME_DURATION;
        }
    }

    /// Starts a timer to automatically dismiss the toast after the specified duration
    pub fn start_dismiss_timer(&mut self, duration: Duration, cx: &mut Context<Self>) {
        self.clear_dismiss_timer(cx);

        let instant_started = std::time::Instant::now();
        let task = cx.spawn(async move |this, cx| {
            cx.background_executor().timer(duration).await;

            if let Some(this) = this.upgrade() {
                this.update(cx, |this, cx| this.hide_toast(cx));
            }
        });

        self.duration_remaining = Some(duration);
        self.dismiss_timer = Some(DismissTimer {
            instant_started,
            _task: task,
        });
        cx.notify();
    }

    /// Restarts the dismiss timer with a new duration
    pub fn restart_dismiss_timer(&mut self, cx: &mut Context<Self>) {
        let Some(duration) = self.duration_remaining else {
            return;
        };
        self.start_dismiss_timer(duration, cx);
        cx.notify();
    }

    /// Clears the dismiss timer if one exists
    pub fn clear_dismiss_timer(&mut self, cx: &mut Context<Self>) {
        self.dismiss_timer.take();
        cx.notify();
    }
}

impl Render for ToastLayer {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(active_toast) = &self.active_toast else {
            return div();
        };

        div().absolute().size_full().bottom_0().left_0().child(
            v_flex()
                .id(("toast-layer-container", active_toast.id))
                .absolute()
                .w_full()
                .bottom(px(0.))
                .flex()
                .flex_col()
                .items_center()
                .track_focus(&active_toast.focus_handle)
                .child(
                    h_flex()
                        .id("active-toast-container")
                        .occlude()
                        .on_hover(cx.listener(|this, hover_start, _window, cx| {
                            if *hover_start {
                                this.pause_dismiss_timer();
                            } else {
                                this.restart_dismiss_timer(cx);
                            }
                            cx.stop_propagation();
                        }))
                        .on_click(|_, _, cx| {
                            cx.stop_propagation();
                        })
                        .on_mouse_down(
                            MouseButton::Middle,
                            cx.listener(|this, _, _, cx| {
                                this.hide_toast(cx);
                            }),
                        )
                        .child(active_toast.toast.view()),
                )
                .animate_in(AnimationDirection::FromBottom, true),
        )
    }
}




#[derive(RegisterComponent)]
pub struct StatusToast {
    icon: Option<Icon>,
    text: SharedString,
    action: Option<ToastAction>,
    show_dismiss: bool,
    auto_dismiss: bool,
    this_handle: Entity<Self>,
    focus_handle: FocusHandle,
}

impl StatusToast {
    pub fn new(
        text: impl Into<SharedString>,
        cx: &mut App,
        f: impl FnOnce(Self, &mut Context<Self>) -> Self,
    ) -> Entity<Self> {
        cx.new(|cx| {
            let focus_handle = cx.focus_handle();

            f(
                Self {
                    text: text.into(),
                    icon: None,
                    action: None,
                    show_dismiss: false,
                    auto_dismiss: true,
                    this_handle: cx.entity(),
                    focus_handle,
                },
                cx,
            )
        })
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn auto_dismiss(mut self, auto_dismiss: bool) -> Self {
        self.auto_dismiss = auto_dismiss;
        self
    }

    pub fn action(
        mut self,
        label: impl Into<SharedString>,
        f: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        let this_handle = self.this_handle.clone();
        self.action = Some(ToastAction::new(
            label.into(),
            Some(Rc::new(move |window, cx| {
                this_handle.update(cx, |_, cx| {
                    cx.emit(DismissEvent);
                });
                f(window, cx);
            })),
        ));
        self
    }

    pub fn dismiss_button(mut self, show: bool) -> Self {
        self.show_dismiss = show;
        self
    }
}

impl Render for StatusToast {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_action_or_dismiss = self.action.is_some() || self.show_dismiss;

        h_flex()
            .id("status-toast")
            .elevation_3(cx)
            .gap_2()
            .py_1p5()
            .pl_2p5()
            .map(|this| {
                if has_action_or_dismiss {
                    this.pr_1p5()
                } else {
                    this.pr_2p5()
                }
            })
            .flex_none()
            .bg(cx.theme().colors().surface_background)
            .shadow_lg()
            .when_some(self.icon.clone(), |this, icon| this.child(icon))
            .child(Label::new(self.text.clone()).color(Color::Default))
            .when_some(self.action.as_ref(), |this, action| {
                this.child(
                    Button::new(action.id.clone(), action.label.clone())
                        .tooltip(Tooltip::for_action_title(
                            action.label.clone(),
                            &toast::RunAction,
                        ))
                        .color(Color::Muted)
                        .when_some(action.on_click.clone(), |el, handler| {
                            el.on_click(move |_click_event, window, cx| handler(window, cx))
                        }),
                )
            })
            .when(self.show_dismiss, |this| {
                let handle = self.this_handle.clone();
                this.child(
                    IconButton::new("dismiss", IconName::Close)
                        .shape(ui::IconButtonShape::Square)
                        .icon_size(IconSize::Small)
                        .icon_color(Color::Muted)
                        .tooltip(Tooltip::text("Dismiss"))
                        .on_click(move |_click_event, _window, cx| {
                            handle.update(cx, |_, cx| {
                                cx.emit(DismissEvent);
                            });
                        }),
                )
            })
    }
}

impl ToastView for StatusToast {
    fn action(&self) -> Option<ToastAction> {
        self.action.clone()
    }

    fn auto_dismiss(&self) -> bool {
        self.auto_dismiss
    }
}

impl Focusable for StatusToast {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<DismissEvent> for StatusToast {}

impl Component for StatusToast {
    fn scope() -> ComponentScope {
        ComponentScope::Notification
    }

    fn description() -> &'static str {
        "A compact, transient toast used to surface status updates \
        such as completed operations or pending updates, with optional icon, \
        action, and dismiss affordances."
    }

    fn preview(_window: &mut Window, cx: &mut App) -> AnyElement {
        let text_example = StatusToast::new("Operation completed", cx, |this, _| this);

        let action_example = StatusToast::new("Update ready to install", cx, |this, _cx| {
            this.action("Restart", |_, _| {})
        });

        let dismiss_button_example =
            StatusToast::new("Dismiss Button", cx, |this, _| this.dismiss_button(true));

        let icon_example = StatusToast::new(
            "Nathan Sobo accepted your contact request",
            cx,
            |this, _| {
                this.icon(
                    Icon::new(IconName::Check)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
            },
        );

        let success_example = StatusToast::new("Pushed 4 changes to `zed/main`", cx, |this, _| {
            this.icon(
                Icon::new(IconName::Check)
                    .size(IconSize::Small)
                    .color(Color::Success),
            )
        });

        let error_example = StatusToast::new(
            "git push: Couldn't find remote origin `iamnbutler/zed`",
            cx,
            |this, _cx| {
                this.icon(
                    Icon::new(IconName::XCircle)
                        .size(IconSize::Small)
                        .color(Color::Error),
                )
                .action("More Info", |_, _| {})
            },
        );

        let warning_example = StatusToast::new("You have outdated settings", cx, |this, _cx| {
            this.icon(
                Icon::new(IconName::Warning)
                    .size(IconSize::Small)
                    .color(Color::Warning),
            )
            .action("More Info", |_, _| {})
        });

        let pr_example =
            StatusToast::new("`zed/new-notification-system` created!", cx, |this, _cx| {
                this.icon(
                    Icon::new(IconName::GitBranch)
                        .size(IconSize::Small)
                        .color(Color::Muted),
                )
                .action("Open Pull Request", |_, cx| {
                    cx.open_url("https://github.com/")
                })
            });

        v_flex()
            .gap_6()
            .p_4()
            .children(vec![
                example_group_with_title(
                    "Basic Toast",
                    vec![
                        single_example("Text", div().child(text_example).into_any_element()),
                        single_example("Action", div().child(action_example).into_any_element()),
                        single_example("Icon", div().child(icon_example).into_any_element()),
                        single_example(
                            "Dismiss Button",
                            div().child(dismiss_button_example).into_any_element(),
                        ),
                    ],
                ),
                example_group_with_title(
                    "Examples",
                    vec![
                        single_example("Success", div().child(success_example).into_any_element()),
                        single_example("Error", div().child(error_example).into_any_element()),
                        single_example("Warning", div().child(warning_example).into_any_element()),
                        single_example("Create PR", div().child(pr_example).into_any_element()),
                    ],
                )
                .vertical(),
            ])
            .into_any_element()
    }
}
