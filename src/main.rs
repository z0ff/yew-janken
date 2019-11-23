use yew::{html, Component, ComponentLink, Html, ShouldRender};
use rand::Rng;

struct Model {
    state: State
}

struct State {
    rand: i32,
    hand: String,
    result: String,
}

impl State {
    fn get_rand(&mut self) {
        self.rand = rand::thread_rng().gen_range(1, 101)
    }
    fn paper(&mut self) {
        self.hand = "✋".to_string()
    }
    fn scissor(&mut self) {
        self.hand = "✌".to_string()
    }
    fn rock(&mut self) {
        self.hand = "✊".to_string()
    }
    fn lose(&mut self) {
        self.result = "YOU LOSE".to_string()
    }
    fn win(&mut self) {
        self.result = "YOU WIN!".to_string()
    }
}

enum Msg {
    Rock,
    Paper,
    Scissor,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            state: State {
                hand: "✊".to_string(),
                result: "あなたは何を出す？".to_string(),
                rand: 0,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Rock => {
                self.state.get_rand();
                if self.state.rand == 1 {
                    self.state.scissor();
                    self.state.win();
                } else {
                    self.state.paper();
                    self.state.lose();
                }
            }
            Msg::Paper => {
                self.state.get_rand();
                if self.state.rand == 1 {
                    self.state.rock();
                    self.state.win();
                } else {
                    self.state.scissor();
                    self.state.lose();
                }
            }
            Msg::Scissor => {
                self.state.get_rand();
                if self.state.rand == 1 {
                    self.state.paper();
                    self.state.win();
                } else {
                    self.state.rock();
                    self.state.lose();
                }
            }
        }
        true
    }

    fn view(&self) -> Html<Self> {
        html! {
            <div>
                <h1>{&self.state.hand}{"👱‍♂️🥤"}</h1>
                <h1>{&self.state.result}</h1>
                <button onclick=|_| Msg::Rock, >{ "✊グーで勝つ" }</button>
                <button onclick=|_| Msg::Paper, >{ "✋パーで勝つ" }</button>
                <button onclick=|_| Msg::Scissor, >{ "✌チョキで勝つ" }</button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}