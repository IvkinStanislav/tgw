mod utils;

use std::{
    panic,
    // collections::HashMap,
    // io::{
    //     Cursor,
    //     Read,
    //     BufReader
    // }
};

use log::{info, error, Level};
use wasm_bindgen::prelude::*;
use docx_rs::{
    read_docx,
    Docx,
    DocumentChild,
    ParagraphChild,
    RunChild,
    Text,
    Run,
    Document
};
use anyhow::Result;

// TODO единый батник для запуска
// TODO init затащить в generate
#[wasm_bindgen]
pub fn init() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(Level::Debug).unwrap();
}

#[wasm_bindgen]
pub fn generate(data: js_sys::Uint8Array) {
    let data = data.to_vec();
    processing(&data).unwrap();
}

// 1) А можно ли собрать новый документ по старому? Проверить и сделать скачивание
// 2) При первом проходе запомнить параграфы, где надо поменять текст
// 3) При втором проходе по запомненой карте собирать документ по старым данным + по новым, поторые брать из карты
fn processing(data: &[u8]) -> Result<()> {
    let docx = read_docx(&data)?;
    for child in docx.document.children {
        match child {
            DocumentChild::Paragraph(paragraph) => {
                for child in paragraph.children {
                    match child {
                        ParagraphChild::Run(run) => {
                            for child in run.children {
                                match child {
                                    RunChild::Text(text) => {
                                        info!("text.text {}", text.text);
                                    },
                                    _ => ()
                                }
                            }
                        },
                        _ => ()
                    }
                }
            },
            _ => ()
        };
    }

    Ok(())
}