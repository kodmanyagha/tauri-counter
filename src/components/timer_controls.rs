use yew::prelude::*;

use crate::{app::TimerState, helpers::format_time};

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    pub timer_state: UseStateHandle<TimerState>,
    pub timer_duration: UseStateHandle<u32>,
    pub session_length: UseStateHandle<u32>,
}

#[function_component]
pub fn TimerControls(props: &Props) -> Html {
    let take_break = {
        let Props {
            session_length,
            timer_duration,
            timer_state,
        } = props.clone();

        Callback::from(move |_: ()| {
            timer_state.set(TimerState::Break);
            timer_duration.set(0);
            session_length.set(5 * 60);
        })
    };

    let pause_timer = {
        let timer_state = props.timer_state.clone();

        Callback::from(move |_: ()| {
            timer_state.set(TimerState::Break);
        })
    };

    let reset_timer = {
        let Props {
            session_length,
            timer_duration,
            timer_state,
        } = props.clone();

        Callback::from(move |_: ()| {
            timer_state.set(TimerState::Paused);
            timer_duration.set(0);
            session_length.set(25 * 60);
        })
    };

    let resume_timer = {
        let timer_state = props.timer_state.clone();
        Callback::from(move |_: ()| {
            timer_state.set(TimerState::Running);
        })
    };

    let start_session = {
        let Props {
            session_length,
            timer_duration,
            timer_state,
        } = props.clone();

        Callback::from(move |_: ()| {
            timer_state.set(TimerState::Running);
            //timer_duration.set(0);
            //session_length.set(25 * 60);
        })
    };

    match *props.timer_state {
        TimerState::Running => {
            html!(
                <div class={classes!("flex", "flex-row", "space-x-2")}>
                    <button class={classes!("p-3")}
                    onclick={move |_| {
                        take_break.emit(());
                    }}
                    >
                       {"Coffee"}
                    </button>
                    <button class={classes!("p-3")}
                    onclick={move |_| {
                        pause_timer.emit(());
                    }}
                    >
                        {"Pause"}
                    </button>
                    <button class={classes!("p-3")}
                    onclick={move |_| {
                        reset_timer.emit(());
                    }}
                    >
                        {"Refresh"}
                    </button>
                </div>
            )
        }
        TimerState::Paused => {
            html!(
                <div class={classes!("flex", "flex-row", "space-x-2")}>
                    <button class={classes!("p-3")}
                    onclick={move |_| {
                        take_break.emit(());
                    }}
                    >
                       {"Coffee"}
                    </button>
                    <button class={classes!("p-3")}
                    onclick={move |_| {
                        resume_timer.emit(());
                    }}
                    >
                        {"Play"}
                    </button>
                    <button class={classes!("p-3")}
                    onclick={move |_| {
                        reset_timer.emit(());
                    }}
                    >
                        {"Refresh"}
                    </button>
                </div>
            )
        }
        TimerState::Break => {
            html!(
                <div class={classes!("flex", "flex-row", "space-x-2")}>
                    <button class={classes!("p-3")}
                    onclick={move |_| {
                        start_session.emit(());
                    }}
                    >
                    {"Continue"}
                    </button>
                </div>
            )
        }
    }
}
