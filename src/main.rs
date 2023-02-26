// TODO: What are these actually doing?
#![deny(clippy::all)]
#![forbid(unsafe_code)]

use eframe::egui;
use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use strum::{EnumIter, IntoEnumIterator};
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().expect("Unable to create Runtime");

    // Enter the runtime so that `tokio::spawn` is available immediately.
    let _enter = rt.enter();

    // Execute the runtime in its own thread.
    // The future doesn't have to do anything. In this example, it just sleeps forever.
    std::thread::spawn(move || {
        rt.block_on(async {
            loop {
                tokio::time::sleep(Duration::from_secs(3600)).await;
            }
        })
    });

    // Run the GUI in the main thread.
    eframe::run_native(
        "DeepL GUI",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
#[derive(Debug, PartialEq, Clone, Copy, Serialize, EnumIter)]
enum SourceLang {
    BG,
    CS,
    DA,
    DE,
    EL,
    EN,
    ES,
    ET,
    FI,
    FR,
    HU,
    ID,
    IT,
    JA,
    KO,
    LT,
    LV,
    NB,
    NL,
    PL,
    PT,
    RO,
    RU,
    SK,
    SL,
    SV,
    TR,
    UK,
    ZH,
}

impl fmt::Display for SourceLang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, EnumIter)]
enum TargetLang {
    BG,
    CS,
    DA,
    DE,
    EL,
    EN,
    ENGB,
    ENUS, // TODO: Render as EN-US? Otherwise not usable
    ES,
    ET,
    FI,
    FR,
    HU,
    ID,
    IT,
    JA,
    KO,
    LT,
    LV,
    NB,
    NL,
    PL,
    PT,
    PTBR,
    PTPT,
    RO,
    RU,
    SK,
    SL,
    SV,
    TR,
    UK,
    ZH,
}

impl fmt::Display for TargetLang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Clone, Serialize)]
struct Req {
    source_lang: SourceLang,
    target_lang: TargetLang,
    text: [String; 1],
}

struct MyApp {
    // Sender/Receiver for async notifications.
    tx: Sender<String>,
    rx: Receiver<String>,

    req: Req,
    translation: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();

        Self {
            tx,
            rx,
            req: Req {
                source_lang: SourceLang::DE,
                target_lang: TargetLang::EN,
                text: ["".to_owned()],
            },
            translation: "".to_owned(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(translation) = self.rx.try_recv() {
            self.translation = translation
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ComboBox::from_label("Source Language")
                .selected_text(self.req.source_lang.to_string())
                .show_ui(ui, |ui| {
                    for source_lang in SourceLang::iter() {
                        ui.selectable_value(
                            &mut self.req.source_lang,
                            source_lang,
                            source_lang.to_string(),
                        );
                    }
                });

            egui::ComboBox::from_label("Target Language")
                .selected_text(self.req.target_lang.to_string())
                .show_ui(ui, |ui| {
                    for target_lang in TargetLang::iter() {
                        ui.selectable_value(
                            &mut self.req.target_lang,
                            target_lang,
                            target_lang.to_string(),
                        );
                    }
                });

            ui.horizontal(|ui| {
                let name_label = ui.label("Query: ");
                ui.text_edit_singleline(&mut self.req.text[0])
                    .labelled_by(name_label.id); // TODO: Call API debounced
                                                 // TODO: Focus text-field on start
            });
            if ui.button("Send Request").clicked() {
                send_req(self.req.clone(), self.tx.clone(), ctx.clone());
            }
            ui.label(self.translation.clone())
        });
    }
}

#[derive(Deserialize)]
struct T {
    text: String,
}

#[derive(Deserialize)]
struct Res {
    translations: [T; 1],
}

fn send_req(req: Req, tx: Sender<String>, ctx: egui::Context) {
    let api_key: String = env::var("DEEPL_API_KEY").expect("DEEPL_API_KEY not set");
    tokio::spawn(async move {
        let body = Client::default()
            .post("https://api-free.deepl.com/v2/translate")
            .json(&req)
            .header(AUTHORIZATION, format!("DeepL-Auth-Key {}", api_key))
            .send()
            .await
            .expect("Unable to send request")
            .json::<Res>()
            .await
            .expect("Unable to parse response");

        let translation = body.translations.into_iter().nth(0).unwrap();
        let _ = tx.send(translation.text);
        ctx.request_repaint();
    });
}
