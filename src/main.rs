#![allow(unused_imports)]
#![allow(dead_code)]

use iepub::prelude::*;
use qmetaobject::*;
use cstr::cstr;
use std::env::args;
use std::ffi::c_int;
use std::path::PathBuf;
use ::log::debug;
use qmetaobject::*;
use clap::{Arg, Command, ArgAction};

#[derive(QObject, Default)]
struct EpubModel {
    base: qt_base_class!(trait QObject),
    chapters: qt_property!(QVariantList; NOTIFY chapters_changed),
    current_content: qt_property!(QString; NOTIFY content_changed),
    font_size: qt_property!(i16; NOTIFY font_size_changed),
    chapters_changed: qt_signal!(),
    content_changed: qt_signal!(),
    font_size_changed: qt_signal!(),
    epub: Option<EpubBook>,

    load_chapter: qt_method!(fn load_chapter(&mut self, index: usize) {
        match self.load_chapter_internal(index) {
            Ok(buf) => {
                self.current_content = QString::from(buf);
                self.content_changed();
            },
            Err(()) => {}
        }
    })
}

impl EpubModel {
    fn load_epub(&mut self, path: String) {
        //self.epub = read_from_file(path);
        match read_from_file(path) {
            Ok(epub) => {
                // println!("Loaded epub");
                epub.chapters().into_iter().for_each(|ch| {
                    if ch.title().is_empty() {
                        // println!("Found chapter {}", ch.file_name());
                        self.chapters.push(QVariant::from(QString::from(ch.file_name())));
                    }
                    else {
                        // println!("Found chapter {}", ch.title());
                        self.chapters.push(QVariant::from(QString::from(ch.title())));
                    }
                });
                self.epub = Some(epub);
                self.chapters_changed();
                self.current_content = QString::from("");
            },
            Err(e) => {
                println!("Failed to load epub file: {}", e)
            }
        }
    }

    pub fn load_chapter_internal(&mut self, index: usize) -> Result<String, ()> {
        match &self.epub {
            Some(e) => {
                println!("Loading chapter number {}", index);
                match e.chapters().nth(index) {
                    Some(chapter) => {
                        //println!("{:#?}", chapter);
                        let mut clone_chapter = chapter.clone();
                        let chapter_data = clone_chapter.string_data();
                        println!("Read string: {} characters", chapter_data.len());
                        //self.current_content = QString::from(chapter_data);
                        Ok(chapter_data)
                    },
                    None => { 
                        println!("Failed to get a chapter!");
                        Err(())
                    }
                }
            },
            None => Err(()),
        }
    }
}

fn main() {
    let matches = clap::Command::new("blick")
        .version("0.1.0")
        .arg(
            Arg::new("filename").required(true)
                .value_parser(clap::value_parser!(PathBuf))
        )
        .arg(
            Arg::new("fontsize")
                .long("fontsize")
                .short('f')
                .value_parser(clap::value_parser!(u16))
                .help("Main text font pixel size")
        )
        .get_matches();

    let filename: &PathBuf = matches
        .get_one::<PathBuf>("filename")
        .expect("Required filename missing");

    let mut model = EpubModel::default();
    model.load_epub(String::from(filename.to_str().unwrap()));
    if matches.contains_id("fontsize") {
        match matches.get_one::<u16>("fontsize") {
            Some(font_size) => {
                model.font_size = *font_size as i16; 
            },
            None => {
                println!("Failed to set font pixel size");
            }
        }
    }

    let obj_box = QObjectBox::new(model);
    qml_register_type::<EpubModel>(cstr!("EpubViewer"), 1, 0, cstr!("EpubModel"));
    let mut engine = QmlEngine::new();
    engine.set_property("epubModel".into(), obj_box.pinned().into());
    engine.load_data(include_str!("main.qml").into());
    engine.exec();
}
