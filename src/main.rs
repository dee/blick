#![allow(unused_imports)]
#![allow(dead_code)]

use iepub::prelude::*;
use qmetaobject::*;
use cstr::cstr;
use std::env::args;
use ::log::debug;
use qmetaobject::*;

#[derive(QObject, Default)]
struct EpubModel {
    base: qt_base_class!(trait QObject),
    chapters: qt_property!(QVariantList; NOTIFY chapters_changed),
    current_content: qt_property!(QString; NOTIFY content_changed),
    chapters_changed: qt_signal!(),
    content_changed: qt_signal!(),
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
    let args: Vec<String> = args().collect();
    if args.len() == 1 {
        println!("File name is not specified.");
        return;
    }

    let mut model = EpubModel::default();
    model.load_epub(args[1].clone());
    let obj_box = QObjectBox::new(model);

    qml_register_type::<EpubModel>(cstr!("EpubViewer"), 1, 0, cstr!("EpubModel"));
    let mut engine = QmlEngine::new();
    engine.set_property("epubModel".into(), obj_box.pinned().into());
    engine.load_data(include_str!("main.qml").into());
    engine.exec();
}
