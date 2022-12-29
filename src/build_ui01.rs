extern crate gtk;
extern crate exif;
extern crate chrono;
extern crate regex;

use gtk::gdk;
use gtk::glib;

// use gtk::gdk_pixbuf::{Pixbuf};

use std::fs;
use std::path::{PathBuf};
use std::io::BufReader;
// use std::io;
use std::fs::File;
use std::process::Command;

use regex::Regex;

use chrono::prelude::*;
use chrono::offset::LocalResult;
use chrono::{Duration, Utc};

use exif::{Reader, In, Tag};

use gtk::prelude::*;
use gtk::{
    ProgressBar,
    Label,
    FileChooserDialog,
    FileChooserAction,
    Button,
    ComboBoxText,
    Entry,
//    EntryExt,
//    SelectionMode,
    CheckButton,
    ListStore,
    TreeModelExt,
//    TreeSelectionExt,
    TreeView,
    TreeViewColumn,
    TreeViewExt,
    CellRendererText,
    Grid,
    ScrolledWindow,
    WidgetExt,
//    WindowPosition,
//    Window,
//    WindowType,
};

const FIRST_COL: i32 = 0;
const SECOND_COL: i32 = 1;
const THIRD_COL: i32 = 2;
const FORTH_COL: i32 = 3;
const FIFTH_COL: i32 = 4;

const STYLE: &str = "
button.text-button {
    /* If we don't put it, the yellow background won't be visible */
    border-style: outset;
    border-width: 5px;
    border-color: #888888;
    background-image: none;
}
#MessTitle {
    font-size: large;
}
/*  progress bar height */
#bar1, progress, trough {
   color: black;
   font-weight: bold;   
   min-height: 15px;
}";

use dump_file::dump_file;

pub fn build_ui(application: &gtk::Application) {

      let provider = gtk::CssProvider::new();
      provider.load_from_data(STYLE.as_bytes());
      if let Some(display) = gdk::Display::get_default() {
          gtk::StyleContext::add_provider_for_display(
              &display,
              &provider,
              gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
      } else {
          eprintln!("Error initializing gtk css provider.");
      };
      

    let window = gtk::ApplicationWindow::new(application);
    let wtitle = format!("Photo Date Changer Rust GTK4 version: {}.{}.{}",gtk::get_major_version(), gtk::get_minor_version(), gtk::get_micro_version());

    window.set_title(Some(&wtitle));
//    window.set_position(WindowPosition::Center);
    window.set_size_request(800, 500);

    let messagetitle_label = Label::new(Some("Message: "));
    gtk::WidgetExt::set_widget_name(&messagetitle_label, "MessTitle");
    let messageval_label = Label::new(Some("Message area"));

    let cdir1box_check = CheckButton::with_label(Some(" "));
    cdir1box_check.set_active(true);
    let cdirectory1_button = Button::with_label("Source Directory");
    let cdirectory1_combobox = ComboBoxText::new();
    cdirectory1_combobox.set_hexpand(true);

    let cdirectory_o_button = Button::with_label("Target Directory");
    let cdirectory_o_combobox = ComboBoxText::new();
    cdirectory_o_combobox.set_hexpand(true);

    let csourcedirbox_check = CheckButton::with_label(Some("Source Dir Date:"));
    let csourcedirval_label = Label::new(Some("None"));
    let ctargetdirbox_check = CheckButton::with_label(Some("Target Dir Date:"));
    let ctargetdirval_label = Label::new(Some("None"));

    let coffset_label = Label::new(Some("Offset (-YY:MM:DD:hh:mm:ss):"));
    let coffset_entry = Entry::new();
    coffset_entry.set_text("-00:00:00:00:00:00");
    let coffsetbox_check = CheckButton::with_label(Some("Keep Offset"));

    let cdatenamebox_check = CheckButton::with_label(Some("Date in Name"));
    let crenamedatebox_check = CheckButton::with_label(Some("Rename with Date")); 

    let ctree_view1 = TreeView::new();
    let seltree = ctree_view1.get_selection();
    seltree.set_mode(gtk::SelectionMode::Multiple); // note 5
    let ccolumn10 = TreeViewColumn::new();
    let ccolumn11 = TreeViewColumn::new();
    let ccolumn12 = TreeViewColumn::new();
    let ccolumn13 = TreeViewColumn::new();
    let ccolumn14 = TreeViewColumn::new();
    let ccell10 = CellRendererText::new();
    let ccell11 = CellRendererText::new();
    let ccell12 = CellRendererText::new();
    let ccell13 = CellRendererText::new();
    let ccell14 = CellRendererText::new();
    ccolumn10.pack_start(&ccell10, true);
    ccolumn11.pack_start(&ccell11, true);
    ccolumn12.pack_start(&ccell12, true);
    ccolumn13.pack_start(&ccell13, true);
    ccolumn14.pack_start(&ccell14, true);
    // Assiciate view's column with model's id column
    ccolumn10.add_attribute(&ccell10, "text", 0);
    ccolumn11.add_attribute(&ccell11, "text", 1);
    ccolumn12.add_attribute(&ccell12, "text", 2);
    ccolumn13.add_attribute(&ccell13, "text", 3);
    ccolumn14.add_attribute(&ccell14, "text", 4);
    ccolumn10.set_title("Name");
    ccolumn11.set_title("Date From");
    ccolumn12.set_title("Current Date");
    ccolumn13.set_title("Assign Date");
    ccolumn14.set_title("New Name");
    ctree_view1.append_column(&ccolumn10);
    ctree_view1.append_column(&ccolumn11);
    ctree_view1.append_column(&ccolumn12);
    ctree_view1.append_column(&ccolumn13);
    ctree_view1.append_column(&ccolumn14);

    let cscroll_window1 = ScrolledWindow::new();
//    let cscroll_window1 = ScrolledWindow::new(None , None);
    cscroll_window1.set_child(Some(&ctree_view1));
    cscroll_window1.set_hexpand(true);
    cscroll_window1.set_vexpand(true);

    let cfilesize_label = Label::new(Some("Length of File Description:"));
    let cfilesize_entry = Entry::new();
    cfilesize_entry.set_text("10");

    let cupsel_button = Button::with_label("Update Selection");

    let cexchg_button = Button::with_label("Execute Change");
 
    let cupallbox_check = CheckButton::with_label(Some(" "));
    let cupall_button = Button::with_label("Update All");

//    let cresetbox_check = CheckButton::new_with_label(" ");
//    let creset_button = Button::new_with_label("Reset");


    let progress_progressbar = ProgressBar::new();
    progress_progressbar.set_show_text(true);
    gtk::WidgetExt::set_widget_name(&progress_progressbar, "bar1");


    let vbox = Grid::new();
    vbox.set_column_spacing(5);
    vbox.set_row_spacing(5);
//    item, column, row, column length, row length
    vbox.attach(&messagetitle_label, 1, 0 , 1, 1);
    vbox.attach(&messageval_label, 2, 0 , 8, 1);
    vbox.attach(&cdir1box_check, 0, 1 , 1, 1);
    vbox.attach(&cdirectory1_button, 1, 1 , 2, 1);
    vbox.attach(&cdirectory1_combobox, 3, 1 , 3, 1);
    vbox.attach(&cdirectory_o_button, 6, 1 , 2, 1);
    vbox.attach(&cdirectory_o_combobox, 8, 1 , 2, 1);
    vbox.attach(&cdatenamebox_check, 1, 2 , 1, 1);
    vbox.attach(&csourcedirbox_check, 3, 2 , 1, 1);
    vbox.attach(&csourcedirval_label, 4, 2 , 1, 1);
    vbox.attach(&ctargetdirbox_check, 8, 2 , 1, 1);
    vbox.attach(&ctargetdirval_label, 9, 2 , 1, 1);
    vbox.attach(&coffset_label, 2, 4 , 1, 1);
    vbox.attach(&crenamedatebox_check, 1, 3, 1, 1);
    vbox.attach(&coffset_entry, 2, 5 , 1, 1);
    vbox.attach(&coffsetbox_check, 3, 5 , 1, 1);
    vbox.attach(&cfilesize_label, 2, 3 , 1, 1);
    vbox.attach(&cfilesize_entry, 3, 3 , 1, 1);
    vbox.attach(&cupsel_button, 9, 5 , 1, 1);    
    vbox.attach(&cscroll_window1, 0, 6 , 10, 4); 
    vbox.attach(&cexchg_button, 1, 10 , 1, 1); 
    vbox.attach(&cupallbox_check, 4, 10, 1, 1);
    vbox.attach(&cupall_button, 5, 10 , 1, 1); 
//    vbox.attach(&cresetbox_check, 8, 10, 1, 1);
//    vbox.attach(&creset_button, 9, 10 , 1, 1);    
    vbox.attach(&progress_progressbar, 0, 13 , 10, 1);

    window.set_child(Some(&vbox));
    window.set_destroy_with_parent(true);
    window.show(); 

//----------------- source directory  button start -----------------------------------
    cdirectory1_button.connect_clicked(glib::clone!(@weak window, @weak cdatenamebox_check, @weak cdir1box_check, @weak cdirectory1_combobox, @weak csourcedirval_label, @weak messageval_label, @weak ctree_view1 => move|_| {
        
        if !cdir1box_check.get_active() {
            messageval_label.set_markup("<span color=\"#FF000000\">Check Box was not set to get Source Directory</span>");
        } else {
            cdir1box_check.set_active(false);
            messageval_label.set_text("getting directory");

            let dialog = FileChooserDialog::new(
                Some("Choose a Directory"),
                Some(&window),
                FileChooserAction::SelectFolder,
                &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
            );

            dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
               if response == gtk::ResponseType::Ok {
                 let mut baddate1 = 0;
                 if let Some(foldername) = d.get_file() {
                    let mut sourcedate = format!("NONE");
                    let re = Regex::new(r"(\d{8})").unwrap();
                    if re.is_match(&foldername.to_string()) {
                         for cap in re.captures_iter(&foldername.to_string()) {
                              sourcedate = format!("YYYYMMDD: {}", &cap[1]);
                         }
                    } else {
                         let rea = Regex::new(r"(\d{6})").unwrap();
                         if rea.is_match(&foldername.to_string()) {
                              for capa in rea.captures_iter(&foldername.to_string()) {
                                   sourcedate = format!("YYYYMM: {}", &capa[1]);
                              }
                         } else {
                              let reb = Regex::new(r"(\d{4})").unwrap();
                              if reb.is_match(&foldername.to_string()) {
                                   for capb in reb.captures_iter(&foldername.to_string()) {
                                        sourcedate = format!("YYYY: {}", &capb[1]);
                                   }
                              }
                        }
                    }
                    csourcedirval_label.set_text(&sourcedate);
                    cdirectory1_combobox.prepend_text(&foldername.to_string());
                    cdirectory1_combobox.set_active(Some(0));
                    let current_dir = PathBuf::from(&foldername.to_string());
                    let new_model = ListStore::new(&[String::static_type(), String::static_type(), String::static_type(), String::static_type(), String::static_type(), String::static_type()]);
//                    let mut orient = format!("-");
                    let mut date_from;
                    let mut file_date;
                    let mut listitems: Vec<String> = Vec::new();
                    let mut numentry = 0;
                    for entry1 in fs::read_dir(&current_dir).unwrap() {
                         let entry = entry1.unwrap();
                         if let Ok(metadata) = entry.metadata() {
                             if let Ok(file_name) = entry.file_name().into_string() {
                                 if metadata.is_file() {
                                     if file_name.ends_with(".jpg") | file_name.ends_with(".JPG") |
                                        file_name.ends_with(".jpeg") |file_name.ends_with(".JPEG") {
                                          if cdatenamebox_check.get_active() {
                                              let dateto;
                                              let mut dateyr = 0;
                                              let mut datemo = 0;
                                              let mut dateday = 0;
                                              let mut datehr = 0;
                                              let mut datemin = 0;
                                              let mut datesec = 0;
                                              let mut datenum = 0;
                                              if file_name.len() < 25 {
                                                  baddate1 = 1;
                                              } else {
// date in name start
                                                  let date1ar2: Vec<&str> = file_name[0..23].split("_").collect();
                                                  let lendat2 = date1ar2.len();
                                                  for indl in 0..lendat2 {
                                                       let date_int: i32 = date1ar2[indl].clone().parse().unwrap_or(-9999);
                                                       if date_int == -9999 {
                                                           baddate1 = 1;
                                                           break;
                                                       } else {
                                                           match indl {
                                                             0 => dateyr = date_int,
                                                             1 => datemo = date_int as u32,
                                                             2 => dateday = date_int as u32,
                                                             3 => datehr = date_int as i32,
                                                             4 => datemin = date_int as i32,
                                                             5 => datesec = date_int as i32,
                                                             6 => datenum = date_int as i32,
                                                             _ => baddate1 = 1,
                                                           }
                                                       }
                                                  }
                                              }
                                              if baddate1 == 0 {
                                                  let datexx = Local.ymd_opt(dateyr, datemo, dateday);
                                                  if datexx == LocalResult::None {
                                                      baddate1 = 1;
                                                  } else {
                                                      if (datenum < 0) | (datenum > 999) {
                                                          baddate1 = 1;
                                                      } else if (datehr < 0) | (datehr > 23) {
                                                          baddate1 = 1;
                                                      } else if (datemin < 0) | (datemin > 59) {
                                                          baddate1 = 1;
                                                      } else if (datesec < 0) | (datesec > 59) {
                                                          baddate1 = 1;
                                                      }
                                                  }
                                              }
// date in name end
//                          add the mod date values 
                                              if baddate1 == 0 {
                                                  dateto = Utc.ymd(dateyr, datemo, dateday).and_hms_milli(datehr as u32, datemin as u32, datesec as u32, 0);
                                                  file_date = format!("{}", dateto.format("%Y-%m-%d %T"));
                                                  date_from = format!("date in name");
                                              } else {
                                                  messageval_label.set_markup("<span color=\"#FF000000\">********* BAD date in Name **********</span>");
                                                  cdatenamebox_check.set_active(false);
                                                  break;
                                              }
                                          } else {
                                              let datetime: DateTime<Local> = metadata.modified().unwrap().into();
                                              file_date = format!("{}", datetime.format("%Y-%m-%d %T"));
                                              date_from = format!("file date");
                                              let file_path = entry.path();
                                              if let Err(_e) = dump_file(&file_path) {
//                                                  orient = format!("Meta error : {}", e);
                                              } else {
                                                  let file = File::open(file_path).unwrap();
                                                  let reader = Reader::new().read_from_container(&mut BufReader::new(&file)).unwrap();
                                                  if let Some(field1) = reader.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
                                                      file_date = format!("{}",field1.value.display_as(field1.tag));
                                                      date_from = format!("date taken");
                                                  } else {
                                                      if let Some(field2) = reader.get_field(Tag::DateTime, In::PRIMARY) {
                                                          file_date = format!("{}",field2.value.display_as(field2.tag));
                                                          date_from = format!("image date");
                                                      }
                                                  }
                                              }
                                          }
                                          let listival = file_name + "|" + &date_from + "|" + &file_date + "|-|-";
                                          listitems.push(listival);
                                          numentry = numentry + 1;
                                       
                                     }
                                 }
                             }
                         }
                    }
                    if baddate1 == 0 {
                        if numentry > 0 {
                            listitems.sort();
                            let listitemlen = listitems.len();
                            let newtoi = listitemlen as i32 ;
                            for indexi in 0..newtoi {
                                 let namelist = &listitems[indexi as usize];
                                 let namesplit: Vec<&str> = namelist.split("|").collect();
                                 new_model.insert_with_values(None,
                                         &[FIRST_COL as u32, SECOND_COL as u32, THIRD_COL as u32, FORTH_COL as u32, FIFTH_COL as u32,],
                                          &[&namesplit[0], &namesplit[1], &namesplit[2], &namesplit[3], &namesplit[4]]);
                            }
                            let errstring = format!("{} files in directory ", numentry);
                            messageval_label.set_text(&errstring);
                            ctree_view1.set_model(Some(&new_model));
                        } else {
                            messageval_label.set_markup("<span color=\"#FF000000\">********* Directory 1: directory has no images **********</span>");
                        }
                    }                
                 } else { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* Directory : ERROR GETTING folder **********</span>");
                 }
               }
               if messageval_label.get_text() == "getting directory" {
                   messageval_label.set_markup("<span color=\"#FF000000\">********* Directory: ERROR  OPEN  button not selected **********</span>");
               }
               d.close();
            });
            dialog.show();

        }
    }));
//----------------- source directory  button end -----------------------------------

//----------------- target directory button start -----------------------------------
    cdirectory_o_button.connect_clicked(glib::clone!(@weak window, @weak cdirectory_o_combobox, @weak ctargetdirval_label, @weak messageval_label => move|_| {
    
        messageval_label.set_text("getting directory");

        let dialog = FileChooserDialog::new(
            Some("Choose a Directory"),
            Some(&window),
            FileChooserAction::SelectFolder,
            &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
        );

        dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
          if response == gtk::ResponseType::Ok {
            if let Some(foldername) = d.get_file() {
                let topath_dir = PathBuf::from(&foldername.to_string());
                let mut numfiles = 0;
                for entry1 in fs::read_dir(&topath_dir).unwrap() {
                     let entry = entry1.unwrap();
                     if let Ok(metadata) = entry.metadata() {
                         if metadata.is_file() {
                             numfiles = numfiles + 1;
                         }
                     }
                }
                if numfiles > 0 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* ERROR Target directory is not empty ******</span>");
                } else {
                    cdirectory_o_combobox.prepend_text(&foldername.to_string());
                    cdirectory_o_combobox.set_active(Some(0));
                    messageval_label.set_text("target directory selected");
                    let mut targetdate = format!("NONE");
                    let re = Regex::new(r"(\d{8})").unwrap();
                    if re.is_match(&foldername.to_string()) {
                         for cap in re.captures_iter(&foldername.to_string()) {
                              targetdate = format!("YYYYMMDD: {}", &cap[1]);
                         }
                    } else {
                         let rea = Regex::new(r"(\d{6})").unwrap();
                         if rea.is_match(&foldername.to_string()) {
                              for capa in rea.captures_iter(&foldername.to_string()) {
                                   targetdate = format!("YYYYMM: {}", &capa[1]);
                              }
                         } else {
                              let reb = Regex::new(r"(\d{4})").unwrap();
                              if reb.is_match(&foldername.to_string()) {
                                   for capb in reb.captures_iter(&foldername.to_string()) {
                                        targetdate = format!("YYYY: {}", &capb[1]);
                                   }
                              }
                        }
                    }
                    ctargetdirval_label.set_text(&targetdate);

                }
            } else { 
                messageval_label.set_markup("<span color=\"#FF000000\">********* Directory: ERROR GETTING folder **********</span>");
            }
          }
          if messageval_label.get_text() == "getting directory" {
              messageval_label.set_markup("<span color=\"#FF000000\">********* Directory: ERROR  OPEN  button not selected **********</span>");
          }
          d.close();
        });
        dialog.show();
    }));
//----------------- target directory button end -----------------------------------

//----------------- update all button start -----------------------------------
    cupall_button.connect_clicked(glib::clone!(@weak cupallbox_check, @weak csourcedirbox_check, @weak ctargetdirbox_check, @weak csourcedirval_label, @weak ctargetdirval_label, @weak ctree_view1, @weak crenamedatebox_check, @weak cfilesize_entry, @weak coffset_entry, @weak progress_progressbar, @weak messageval_label  => move|_| {

        let mut bolok = true;
// see if update all check box is checked
        if !cupallbox_check.get_active() {
            messageval_label.set_markup("<span color=\"#FF000000\">Check Box was not set to get Source Directory</span>");
            bolok = false;
        } else {
// see if directory list has files
            cupallbox_check.set_active(false);
            let treemodel1 = ctree_view1.get_model();
            if treemodel1 == None {
                messageval_label.set_markup("<span color=\"#FF000000\">********* ERROR NOTHING IN DIRECTORY LIST **********</span>");
                bolok = false;
            }
        }
// evaluate source and target dir date
        let mut offsettest = false;
        let mut year_int: i32 = 0;
        let mut month_int: i32 = 0;
        let mut day_int: i32 = 0;
        let mut year_set = false;
        let mut month_set = false;
        let mut day_set = false;
        if bolok {
            if csourcedirbox_check.get_active() {
                if ctargetdirbox_check.get_active() {
                    messageval_label.set_markup("<span color=\"#FF000000\">Check Box for both Source and Target Dir Date are set</span>");
                    bolok = false;
                } else {
                    let sourcedir_text = csourcedirval_label.get_text();
                    if sourcedir_text.len() < 18 {
                        if sourcedir_text.len() < 14 {
                            if sourcedir_text.len() < 10 {
                                 messageval_label.set_markup("<span color=\"#FF000000\">Invalid value in Source Dir Date</span>");
                                 bolok = false;
                            } else {
                                 year_int = sourcedir_text[6..10].parse().unwrap_or(-99);
                                 offsettest = true;
                                 year_set = true;
                            }
                        } else {
                            year_int = sourcedir_text[8..12].parse().unwrap_or(-99);
                            month_int = sourcedir_text[12..14].parse().unwrap_or(-99);
                            offsettest = true;
                            year_set = true;
                            month_set = true;
                        }                                  
                    } else {
                        year_int = sourcedir_text[10..14].parse().unwrap_or(-99);
                        month_int = sourcedir_text[14..16].parse().unwrap_or(-99);
                        day_int = sourcedir_text[16..18].parse().unwrap_or(-99);
                        offsettest = true;
                        year_set = true;
                        month_set = true;
                        day_set = true;
                    }
                }
            } else {
                if ctargetdirbox_check.get_active() {
                    let targetdir_text = ctargetdirval_label.get_text(); 
                    if targetdir_text.len() < 18 {
                        if targetdir_text.len() < 14 {
                            if targetdir_text.len() < 10 {
                                 messageval_label.set_markup("<span color=\"#FF000000\">Invalid value in Target Dir Date</span>");
                                 bolok = false;
                            } else {
                                 year_int = targetdir_text[6..10].parse().unwrap_or(-99);
                                 offsettest = true;
                                 year_set = true;
                            }
                        } else {
                            year_int = targetdir_text[8..12].parse().unwrap_or(-99);
                            month_int = targetdir_text[12..14].parse().unwrap_or(-99);
                            offsettest = true;
                            year_set = true;
                            month_set = true;
                        }                                  
                    } else {
                        year_int = targetdir_text[10..14].parse().unwrap_or(-99);
                        month_int = targetdir_text[14..16].parse().unwrap_or(-99);
                        day_int = targetdir_text[16..18].parse().unwrap_or(-99);
                        offsettest = true;
                        year_set = true;
                        month_set = true;
                        day_set = true;
                    }
                }
            }
        }
        if bolok {
            if year_set {
                if year_int > 0 {
                    if (year_int < 1900) | (year_int > 2100) {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* invalid year for dir date. Must be between 1900 and 2100 **********</span>");
                        bolok = false;
                    }
                } else if year_int == -99 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* invalid year for dir date. Must be an integer **********</span>");
                    bolok = false;
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* invalid year for dir date. Not a positive integer **********</span>");
                    bolok = false;
                }
            }
        }
        if bolok {
            if month_set {
                if month_int > 0 {
                    if (month_int < 1) | (month_int > 12) {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* invalid month for dir date. Must be between 1 and 12 **********</span>");
                        bolok = false;
                    }
                } else if month_int == -99 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* invalid month for dir date. Must be an integer **********</span>");
                    bolok = false;
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* invalid month for dir date. Not a positive integer **********</span>");
                    bolok = false;
                }
            }
        }                                  
        if bolok {
            if day_set {
                if day_int > 0 {
                    if (day_int < 1) | (day_int > 31) {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* invalid day for dir date. Must be between 1 and 31 **********</span>");
                        bolok = false;
                    } else {
                        let datexx = Local.ymd_opt(year_int, month_int as u32, day_int as u32);
                        if datexx == LocalResult::None {
                            messageval_label.set_markup("<span color=\"#FF000000\">********* invalid day for dir date month. **********</span>");
                            bolok = false;
                        }
                    }
                } else if day_int == -99 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* invalid day for dir date. Must be an integer **********</span>");
                    bolok = false;
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* invalid day for dir date. Not a positive integer **********</span>");
                    bolok = false;
                }
            }
        }                                  
// validate offset values
        let mut dateyr1 = 0;
        let mut datemo1 = 0;
        let mut dateday1 = 0;
        let mut datehr1 = 0;
        let mut datemin1 = 0;
        let mut datesec1 = 0;
        if bolok {
            let datemod1_text = coffset_entry.get_text();
            let datemod1_textx: &String = &format!("{}", datemod1_text);
            let date1ar1: Vec<&str> = datemod1_textx[0..].split(":").collect();
            let lendat1 = date1ar1.len();
            if (lendat1 > 6) | (lendat1 < 6) {
                bolok = false;
            } else {
                for indl in 0..lendat1 {
                     let date_int: i32 = date1ar1[indl].clone().parse().unwrap_or(-9999);
                     if date_int == -9999 {
                         bolok = false;
                     } else {
                         match indl {
                            0 => dateyr1 = date_int as i64,
                            1 => datemo1 = date_int as i64,
                            2 => dateday1 = date_int as i64,
                            3 => datehr1 = date_int as i64,
                            4 => datemin1 = date_int as i64,
                            5 => datesec1 = date_int as i64,
                            _ => bolok = false,
                         }
                     }
                }
            }
            if !bolok {
                messageval_label.set_markup("<span color=\"#FF000000\">********* Date offset is not formatted correctly **********</span>");
            } else {
                if dateyr1 != 0 {
                    offsettest = true;
                } else if datemo1 != 0 {
                    offsettest = true;
                } else if dateday1 != 0 {
                    offsettest = true;
                } else if datehr1 != 0 {
                    offsettest = true;
                } else if datemin1 != 0 {
                    offsettest = true;
                } else if datesec1 != 0 {
                    offsettest = true;
                }
            }
        }

        let mut filesize_int: i32 = 0;
        if bolok {
            if crenamedatebox_check.get_active() {
// see if filesize exists and is between 4 and 16
                let filesize_text = cfilesize_entry.get_text();
                filesize_int = filesize_text.parse().unwrap_or(-99);
                if filesize_int > 0 {
                    if (filesize_int < 4) | (filesize_int > 16) {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* Invalid file length. Must be between 4 and 16 **********</span>");
                        bolok = false;
                    }
                } else if filesize_int == -99 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* File length is not an integer **********</span>");
                    bolok = false;
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* File length is not positive integer **********</span>");
                    bolok = false;
                }
            }
        }
// check if files to process and loop through creating another model with changes
        if bolok {
            let view1model = ctree_view1.get_model();
            if view1model == None {
                messageval_label.set_markup("<span color=\"#FF000000\">********* ERROR NOTHING IN FILE LIST **********</span>");
            } else {
                progress_progressbar.set_fraction(0.0);
                while glib::MainContext::pending(&glib::MainContext::default()) {
                   glib::MainContext::iteration(&glib::MainContext::default(),true);
                }
                let new_model = ListStore::new(&[String::static_type(), String::static_type(), String::static_type(), String::static_type(), String::static_type()]);
                let view1modeluw = view1model.unwrap();
                let mut valid = true;
                let validval = view1modeluw.get_iter_first().unwrap();
                let mut numrow = 0;
                let numchildren = view1modeluw.iter_n_children(None);
                while valid {
                      let filenameval = view1modeluw.get(&validval,0).get::<String>();
                      let filenamestr = format!("{:?}", filenameval);
                      let filenameln = filenamestr.len();
                      let filenameend = filenameln - 3;
                      let filenamestart = 9;
                      let filenamex = filenamestr.get(filenamestart..filenameend).unwrap();

                      let  mut filelocx = "No Chg";

                      let filecurval = view1modeluw.get(&validval,2).get::<String>();
                      let filecurstr = format!("{:?}", filecurval);
                      let filecurln = filecurstr.len();
                      let filecurend = filecurln - 3;
                      let filecurstart = 9;
                      let filecurx = filecurstr.get(filecurstart..filecurend).unwrap();

                      let fileassval = view1modeluw.get(&validval,3).get::<String>();
                      let fileassstr = format!("{:?}", fileassval);
                      let fileassln = fileassstr.len();
                      let fileassend = fileassln - 3;
                      let fileassstart = 9;
                      let fileassxstr;
                      let mut fileassx = fileassstr.get(fileassstart..fileassend).unwrap();
                      if fileassx == "-" {
                          fileassx = filecurx;
                      }
                      let filenewnamestr;
                      let mut filenewnamex = filenamex;

                      valid = view1modeluw.iter_next(&validval);

                      if offsettest {
                          let mut dateto;
                          let mut dateyr = 0;
                          let mut datemo = 0;
                          let mut dateday = 0;
                          let mut datehr = 0;
                          let mut datemin = 0;
                          let mut datesec = 0;
                          let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
                          let after = re.replace_all(&fileassx, "_");
                          let listdatex: Vec<&str> = after.split("_").collect();
                          let lendat2 = listdatex.len();
                          let mut baddate1 = 0;
                          for indl in 0..lendat2 {
                              let date_int: i32 = listdatex[indl].clone().parse().unwrap_or(-9999);
                              if date_int == -9999 {
                                  baddate1 = 1;
                                  break;
                              } else {
                                  match indl {
                                     0 => dateyr = date_int,
                                     1 => datemo = date_int as u32,
                                     2 => dateday = date_int as u32,
                                     3 => datehr = date_int as i32,
                                     4 => datemin = date_int as i32,
                                     5 => datesec = date_int as i32,
                                     _ => baddate1 = 1,
                                  }
                              }
                         }
                         if baddate1 == 0 {
                             if year_set {
                                 dateyr = year_int;
                             }
                             if month_set {
                                 datemo = month_int as u32;
                             }
                             if day_set {
                                 dateday = day_int as u32;
                             }
                             let datexx = Local.ymd_opt(dateyr, datemo, dateday);
                             if datexx == LocalResult::None {
                                 baddate1 = 1;
                             } else {
                                 if (datehr < 0) | (datehr > 23) {
                                      baddate1 = 1;
                                 } else if (datemin < 0) | (datemin > 59) {
                                      baddate1 = 1;
                                 } else if (datesec < 0) | (datesec > 59) {
                                      baddate1 = 1;
                                 }
                             }
                         }
// date in name end
//                          add the mod date values 
                         if baddate1 == 0 {
                             dateto = Utc.ymd(dateyr, datemo, dateday).and_hms_milli(datehr as u32, datemin as u32, datesec as u32, 0);
                             dateto = dateto + Duration::days(dateyr1*365) +
                                                   Duration::days(datemo1*30) +
                                                   Duration::days(dateday1) +
                                                   Duration::hours(datehr1) +
                                                   Duration::minutes(datemin1) +
                                                   Duration::seconds(datesec1);
                             fileassxstr = format!("{}", dateto.format("%Y:%m:%d %H:%M:%S"));
//                             let fileassxln = fileassxstr.len();
//                             let fileassxend = filenameln - 1;
//                             let fileassxstart = 0;
//                             fileassx = fileassxstr.get(fileassxstart..fileassxend).unwrap();
                             fileassx = &fileassxstr;
                             filelocx = "Date Chg";
                         } else {
                             messageval_label.set_markup("<span color=\"#FF000000\">********* get_strvector: BAD DATE  is not correct **********</span>");
                             bolok = false;
                             break;
                         }
                      }
                      if bolok {
                          if crenamedatebox_check.get_active() {
                              let filenamexx = format!("{}", filenamex.clone());
                              let strfilesplit: Vec<&str> = filenamexx.split(".").collect();
                              let lenfilesplit = strfilesplit.len();
                              if lenfilesplit != 2 {
                                  messageval_label.set_markup("<span color=\"#FF000000\">********* one of filename does not have just 1 period **********</span>");
                                  bolok = false;
                                  break;
                              }
                              let prefix1: String = strfilesplit[0].parse().unwrap();
                              let suffix1: String = strfilesplit[1].parse().unwrap();
                              let mut strlen = prefix1.len() as i32;
                              let mut prefixx: String = "x".to_owned();
                              if strlen < filesize_int {
                                  strlen = strlen + 1;
                                  while strlen < filesize_int {
                                         prefixx.push_str("x");
                                         strlen = strlen + 1;
                                  }
                                  prefixx.push_str(&prefix1);
                              } else {
                                  prefixx = strfilesplit[0][(strlen - filesize_int)as usize..].parse().unwrap();
                              }
                              prefixx.push_str(".");
                              prefixx.push_str(&suffix1);
                              let re = Regex::new(r"[^A-Za-z0-9.]").unwrap();
                              let after = re.replace_all(&prefixx, "_");
                              let datesubstrfx = after.to_string();
                              let dateto;
                              let mut dateyr = 0;
                              let mut datemo = 0;
                              let mut dateday = 0;
                              let mut datehr = 0;
                              let mut datemin = 0;
                              let mut datesec = 0;
                              let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
                              let after = re.replace_all(&fileassx, "_");
                              let listdatex: Vec<&str> = after.split("_").collect();
                              let lendat2 = listdatex.len();
                              let mut baddate1 = 0;
                              for indl in 0..lendat2 {
                                   let date_int: i32 = listdatex[indl].clone().parse().unwrap_or(-9999);
                                   if date_int == -9999 {
                                       baddate1 = 1;
                                       break;
                                   } else {
                                       match indl {
                                         0 => dateyr = date_int,
                                         1 => datemo = date_int as u32,
                                         2 => dateday = date_int as u32,
                                         3 => datehr = date_int as i32,
                                         4 => datemin = date_int as i32,
                                         5 => datesec = date_int as i32,
                                         _ => baddate1 = 1,
                                      }
                                   }
                              }
                              if baddate1 == 0 {
                                  let datexx = Local.ymd_opt(dateyr, datemo, dateday);
                                  if datexx == LocalResult::None {
                                      baddate1 = 1;
                                  } else {
                                      if (datehr < 0) | (datehr > 23) {
                                           baddate1 = 1;
                                      } else if (datemin < 0) | (datemin > 59) {
                                           baddate1 = 1;
                                      } else if (datesec < 0) | (datesec > 59) {
                                           baddate1 = 1;
                                      }
                                  }
                              }
                              if baddate1 == 0 {
                                  dateto = Utc.ymd(dateyr, datemo, dateday).and_hms_milli(datehr as u32, datemin as u32, datesec as u32, 0);
                                  filenewnamestr = format!("{}_500_{}", dateto.format("%Y_%m_%d_%H_%M_%S"), datesubstrfx);
//                                  let filenewnameln = filenewnamestr.len();
//                                  let filenewnameend = filenewnameln - 1;
//                                  let filenewnamestart = 0;
//                                  filenewnamex = filenewnamestr.get(filenewnamestart..filenewnameend).unwrap();
                                  filenewnamex = &filenewnamestr;
                                  filelocx = "Date Chg";
                              } else {
                                  messageval_label.set_markup("<span color=\"#FF000000\">********* BAD DATE in assign date **********</span>");
                                  bolok = false;
                                  break;
                              }
                          }
                      }
                      if bolok {
                          new_model.insert_with_values(None,
                                 &[FIRST_COL as u32, SECOND_COL as u32, THIRD_COL as u32, FORTH_COL as u32, FIFTH_COL as u32,],
                                 &[&filenamex, &filelocx, &filecurx, &fileassx, &filenewnamex]);
                      }
                      numrow = numrow + 1;
                      let progressfr: f64 = numrow as f64 / numchildren as f64;
                      progress_progressbar.set_fraction(progressfr);
                      while glib::MainContext::pending(&glib::MainContext::default()) {
                          glib::MainContext::iteration(&glib::MainContext::default(),true);
                      }
                }
                if bolok {
                    ctree_view1.set_model(Some(&new_model));
                    let messvalx = format!("convert merge merged {} files", numrow);
                    messageval_label.set_text(&messvalx);
                }
            }
        } 
    }));
//----------------- update all button end -----------------------------------

//----------------- update selection button start -----------------------------------
    cupsel_button.connect_clicked(glib::clone!(@weak csourcedirbox_check, @weak ctargetdirbox_check, @weak csourcedirval_label, @weak ctargetdirval_label, @weak ctree_view1, @weak crenamedatebox_check, @weak cfilesize_entry, @weak coffset_entry, @weak progress_progressbar, @weak messageval_label  => move|_| {

        let mut bolok = true;
// see if directory list has files
        let treemodel1 = ctree_view1.get_model();
        if treemodel1 == None {
            messageval_label.set_markup("<span color=\"#FF000000\">********* ERROR NOTHING IN DIRECTORY LIST **********</span>");
            bolok = false;
        }
// evaluate source and target dir date
        let mut offsettest = false;
        let mut year_int: i32 = 0;
        let mut month_int: i32 = 0;
        let mut day_int: i32 = 0;
        let mut year_set = false;
        let mut month_set = false;
        let mut day_set = false;
        if bolok {
            if csourcedirbox_check.get_active() {
                if ctargetdirbox_check.get_active() {
                    messageval_label.set_markup("<span color=\"#FF000000\">Check Box for both Source and Target Dir Date are set</span>");
                    bolok = false;
                } else {
                    let sourcedir_text = csourcedirval_label.get_text();
                    if sourcedir_text.len() < 18 {
                        if sourcedir_text.len() < 14 {
                            if sourcedir_text.len() < 10 {
                                 messageval_label.set_markup("<span color=\"#FF000000\">Invalid value in Source Dir Date</span>");
                                 bolok = false;
                            } else {
                                 year_int = sourcedir_text[6..10].parse().unwrap_or(-99);
                                 offsettest = true;
                                 year_set = true;
                            }
                        } else {
                            year_int = sourcedir_text[8..12].parse().unwrap_or(-99);
                            month_int = sourcedir_text[12..14].parse().unwrap_or(-99);
                            offsettest = true;
                            year_set = true;
                            month_set = true;
                        }                                  
                    } else {
                        year_int = sourcedir_text[10..14].parse().unwrap_or(-99);
                        month_int = sourcedir_text[14..16].parse().unwrap_or(-99);
                        day_int = sourcedir_text[16..18].parse().unwrap_or(-99);
                        offsettest = true;
                        year_set = true;
                        month_set = true;
                        day_set = true;
                    }
                }
            } else {
                if ctargetdirbox_check.get_active() {
                    let targetdir_text = ctargetdirval_label.get_text(); 
                    if targetdir_text.len() < 18 {
                        if targetdir_text.len() < 14 {
                            if targetdir_text.len() < 10 {
                                 messageval_label.set_markup("<span color=\"#FF000000\">Invalid value in Target Dir Date</span>");
                                 bolok = false;
                            } else {
                                 year_int = targetdir_text[6..10].parse().unwrap_or(-99);
                                 offsettest = true;
                                 year_set = true;
                            }
                        } else {
                            year_int = targetdir_text[8..12].parse().unwrap_or(-99);
                            month_int = targetdir_text[12..14].parse().unwrap_or(-99);
                            offsettest = true;
                            year_set = true;
                            month_set = true;
                        }                                  
                    } else {
                        year_int = targetdir_text[10..14].parse().unwrap_or(-99);
                        month_int = targetdir_text[14..16].parse().unwrap_or(-99);
                        day_int = targetdir_text[16..18].parse().unwrap_or(-99);
                        offsettest = true;
                        year_set = true;
                        month_set = true;
                        day_set = true;
                    }
                }
            }
        }
        if bolok {
            if year_set {
                if year_int > 0 {
                    if (year_int < 1900) | (year_int > 2100) {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* invalid year for dir date. Must be between 1900 and 2100 **********</span>");
                        bolok = false;
                    }
                } else if year_int == -99 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* invalid year for dir date. Must be an integer **********</span>");
                    bolok = false;
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* invalid year for dir date. Not a positive integer **********</span>");
                    bolok = false;
                }
            }
        }
        if bolok {
            if month_set {
                if month_int > 0 {
                    if (month_int < 1) | (month_int > 12) {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* invalid month for dir date. Must be between 1 and 12 **********</span>");
                        bolok = false;
                    }
                } else if month_int == -99 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* invalid month for dir date. Must be an integer **********</span>");
                    bolok = false;
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* invalid month for dir date. Not a positive integer **********</span>");
                    bolok = false;
                }
            }
        }                                  
        if bolok {
            if day_set {
                if day_int > 0 {
                    if (day_int < 1) | (day_int > 31) {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* invalid day for dir date. Must be between 1 and 31 **********</span>");
                        bolok = false;
                    } else {
                        let datexx = Local.ymd_opt(year_int, month_int as u32, day_int as u32);
                        if datexx == LocalResult::None {
                            messageval_label.set_markup("<span color=\"#FF000000\">********* invalid day for dir date month. **********</span>");
                            bolok = false;
                        }
                    }
                } else if day_int == -99 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* invalid day for dir date. Must be an integer **********</span>");
                    bolok = false;
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* invalid day for dir date. Not a positive integer **********</span>");
                    bolok = false;
                }
            }
        }                                  
// validate offset values
        let mut dateyr1 = 0;
        let mut datemo1 = 0;
        let mut dateday1 = 0;
        let mut datehr1 = 0;
        let mut datemin1 = 0;
        let mut datesec1 = 0;
        if bolok {
            let datemod1_text = coffset_entry.get_text();
            let datemod1_textx: &String = &format!("{}", datemod1_text);
            let date1ar1: Vec<&str> = datemod1_textx[0..].split(":").collect();
            let lendat1 = date1ar1.len();
            if (lendat1 > 6) | (lendat1 < 6) {
                bolok = false;
            } else {
                for indl in 0..lendat1 {
                     let date_int: i32 = date1ar1[indl].clone().parse().unwrap_or(-9999);
                     if date_int == -9999 {
                         bolok = false;
                     } else {
                         match indl {
                            0 => dateyr1 = date_int as i64,
                            1 => datemo1 = date_int as i64,
                            2 => dateday1 = date_int as i64,
                            3 => datehr1 = date_int as i64,
                            4 => datemin1 = date_int as i64,
                            5 => datesec1 = date_int as i64,
                            _ => bolok = false,
                         }
                     }
                }
            }
            if !bolok {
                messageval_label.set_markup("<span color=\"#FF000000\">********* Date offset is not formatted correctly **********</span>");
            } else {
                if dateyr1 != 0 {
                    offsettest = true;
                } else if datemo1 != 0 {
                    offsettest = true;
                } else if dateday1 != 0 {
                    offsettest = true;
                } else if datehr1 != 0 {
                    offsettest = true;
                } else if datemin1 != 0 {
                    offsettest = true;
                } else if datesec1 != 0 {
                    offsettest = true;
                }
            }
        }

        let mut filesize_int: i32 = 0;
        if bolok {
            if crenamedatebox_check.get_active() {
// see if filesize exists and is between 4 and 16
                let filesize_text = cfilesize_entry.get_text();
                filesize_int = filesize_text.parse().unwrap_or(-99);
                if filesize_int > 0 {
                    if (filesize_int < 4) | (filesize_int > 16) {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* Invalid file length. Must be between 4 and 16 **********</span>");
                        bolok = false;
                    }
                } else if filesize_int == -99 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* File length is not an integer **********</span>");
                    bolok = false;
                } else {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* File length is not positive integer **********</span>");
                    bolok = false;
                }
            }
        }
// check if files to process and loop through creating another model with changes
        let selectiont = ctree_view1.get_selection();
        let view1model = ctree_view1.get_model();
        if bolok {
            if view1model == None {
                messageval_label.set_markup("<span color=\"#FF000000\">********* ERROR NOTHING IN FILE LIST **********</span>");
                    bolok = false;
            } else {
                if selectiont.count_selected_rows() < 1 {
                    messageval_label.set_markup("<span color=\"#FF000000\">*********  NO SELECTION IN TO DIRECTORY **********</span>");
                    bolok = false;
                }            
            }
        }
        if bolok {
            progress_progressbar.set_fraction(0.0);
            while glib::MainContext::pending(&glib::MainContext::default()) {
                glib::MainContext::iteration(&glib::MainContext::default(),true);
            }
            let mut listitems: Vec<String> = Vec::new();
            let (pathlist, modeltt) = selectiont.get_selected_rows();
            let pathsize = pathlist.len();
            for indl in 0..pathsize {
// load list of files into a vector list
                 let itert = modeltt.get_iter(&pathlist[indl]).unwrap();
                 let tofilenameval = modeltt.get(&itert, 0).get::<String>();
                 let filenamestr = format!("{:?}", tofilenameval);
                 let filenameln = filenamestr.len();
                 let filenameend = filenameln - 3;
                 let filenamestart = 9;
                 let filenamex = filenamestr.get(filenamestart..filenameend).unwrap();
                 listitems.push(filenamex.to_string());
            }
            let listitemslen = listitems.len();
            let newtoi = listitemslen as i32 ;
            let new_model = ListStore::new(&[String::static_type(), String::static_type(), String::static_type(), String::static_type(), String::static_type()]);
            let view1modeluw = view1model.unwrap();
            let mut valid = true;
            let validval = view1modeluw.get_iter_first().unwrap();
            while valid {
                   let filenameval = view1modeluw.get(&validval,0).get::<String>();
                   let filenamestr = format!("{:?}", filenameval);
                   let filenameln = filenamestr.len();
                   let filenameend = filenameln - 3;
                   let filenamestart = 9;
                   let filenamex = filenamestr.get(filenamestart..filenameend).unwrap();

                   let  mut filelocx = "No Chg";

                   let filecurval = view1modeluw.get(&validval,2).get::<String>();
                   let filecurstr = format!("{:?}", filecurval);
                   let filecurln = filecurstr.len();
                   let filecurend = filecurln - 3;
                   let filecurstart = 9;
                   let filecurx = filecurstr.get(filecurstart..filecurend).unwrap();

                   let fileassval = view1modeluw.get(&validval,3).get::<String>();
                   let fileassstr = format!("{:?}", fileassval);
                   let fileassln = fileassstr.len();
                   let fileassend = fileassln - 3;
                   let fileassstart = 9;
                   let fileassxstr;
                   let mut fileassx = fileassstr.get(fileassstart..fileassend).unwrap();
                   if fileassx == "-" {
                       fileassx = filecurx;
                   }

                   let filenamenwval = view1modeluw.get(&validval,4).get::<String>();
                   let filenamenwstr = format!("{:?}", filenamenwval);
                   let filenamenwln = filenamenwstr.len();
                   let filenamenwend = filenamenwln - 3;
                   let filenamenwstart = 9;
                   let filenamenwx = filenamenwstr.get(filenamenwstart..filenamenwend).unwrap();

                   let filenewnamestr;
                   let mut filenewnamex = filenamenwx;

                   valid = view1modeluw.iter_next(&validval);

                   let mut foundx = false;
                   for indexi in 0..newtoi {
//                        let mut listval: String  = listitems[indexi as usize];
//                        let mut listval  = &listitems[indexi as usize];
                        if listitems[indexi as usize] == filenamex {
                            foundx = true;
                        }
                   }
                   if foundx {
// need to load all and scan if item is in the list and then process 
                       if offsettest {
                           let mut dateto;
                           let mut dateyr = 0;
                           let mut datemo = 0;
                           let mut dateday = 0;
                           let mut datehr = 0;
                           let mut datemin = 0;
                           let mut datesec = 0;
                           let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
                           let after = re.replace_all(&fileassx, "_");
                           let listdatex: Vec<&str> = after.split("_").collect();
                           let lendat2 = listdatex.len();
                           let mut baddate1 = 0;
                           for indl in 0..lendat2 {
                               let date_int: i32 = listdatex[indl].clone().parse().unwrap_or(-9999);
                               if date_int == -9999 {
                                   baddate1 = 1;
                                   break;
                               } else {
                                   match indl {
                                      0 => dateyr = date_int,
                                      1 => datemo = date_int as u32,
                                      2 => dateday = date_int as u32,
                                      3 => datehr = date_int as i32,
                                      4 => datemin = date_int as i32,
                                      5 => datesec = date_int as i32,
                                      _ => baddate1 = 1,
                                   }
                               }
                          }
                          if baddate1 == 0 {
                              if year_set {
                                  dateyr = year_int;
                              }
                              if month_set {
                                  datemo = month_int as u32;
                              }
                              if day_set {
                                  dateday = day_int as u32;
                              }
                              let datexx = Local.ymd_opt(dateyr, datemo, dateday);
                              if datexx == LocalResult::None {
                                  baddate1 = 1;
                              } else {
                                  if (datehr < 0) | (datehr > 23) {
                                       baddate1 = 1;
                                  } else if (datemin < 0) | (datemin > 59) {
                                       baddate1 = 1;
                                  } else if (datesec < 0) | (datesec > 59) {
                                       baddate1 = 1;
                                  }
                              }
                          }
// date in name end
//                          add the mod date values 
                          if baddate1 == 0 {
                              dateto = Utc.ymd(dateyr, datemo, dateday).and_hms_milli(datehr as u32, datemin as u32, datesec as u32, 0);
                              dateto = dateto + Duration::days(dateyr1*365) +
                                                    Duration::days(datemo1*30) +
                                                    Duration::days(dateday1) +
                                                    Duration::hours(datehr1) +
                                                    Duration::minutes(datemin1) +
                                                    Duration::seconds(datesec1);
                              fileassxstr = format!("{}", dateto.format("%Y:%m:%d %H:%M:%S"));
//                             let fileassxln = fileassxstr.len();
//                             let fileassxend = filenameln - 1;
//                             let fileassxstart = 0;
//                             fileassx = fileassxstr.get(fileassxstart..fileassxend).unwrap();
                              fileassx = &fileassxstr;
                              filelocx = "Date Chg";
                          } else {
                              messageval_label.set_markup("<span color=\"#FF000000\">********* get_strvector: BAD DATE  is not correct **********</span>");
                              break;
                          }
                       }
                       if bolok {
                           if !crenamedatebox_check.get_active() {
                               filenewnamex = &filenamex.clone();
                           } else {    
                               let filenamexx = format!("{}", filenamex.clone());
                               let strfilesplit: Vec<&str> = filenamexx.split(".").collect();
                               let lenfilesplit = strfilesplit.len();
                               if lenfilesplit != 2 {
                                   messageval_label.set_markup("<span color=\"#FF000000\">********* one of filename does not have just 1 period **********</span>");
                                   break;
                               }
                               let prefix1: String = strfilesplit[0].parse().unwrap();
                               let suffix1: String = strfilesplit[1].parse().unwrap();
                               let mut strlen = prefix1.len() as i32;
                               let mut prefixx: String = "x".to_owned();
                               if strlen < filesize_int {
                                   strlen = strlen + 1;
                                   while strlen < filesize_int {
                                          prefixx.push_str("x");
                                          strlen = strlen + 1;
                                   }
                                   prefixx.push_str(&prefix1);
                               } else {
                                   prefixx = strfilesplit[0][(strlen - filesize_int)as usize..].parse().unwrap();
                               }
                               prefixx.push_str(".");
                               prefixx.push_str(&suffix1);
                               let re = Regex::new(r"[^A-Za-z0-9.]").unwrap();
                               let after = re.replace_all(&prefixx, "_");
                               let datesubstrfx = after.to_string();
                               let dateto;
                               let mut dateyr = 0;
                               let mut datemo = 0;
                               let mut dateday = 0;
                               let mut datehr = 0;
                               let mut datemin = 0;
                               let mut datesec = 0;
                               let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
                               let after = re.replace_all(&fileassx, "_");
                               let listdatex: Vec<&str> = after.split("_").collect();
                               let lendat2 = listdatex.len();
                               let mut baddate1 = 0;
                               for indl in 0..lendat2 {
                                    let date_int: i32 = listdatex[indl].clone().parse().unwrap_or(-9999);
                                    if date_int == -9999 {
                                        baddate1 = 1;
                                        break;
                                    } else {
                                        match indl {
                                          0 => dateyr = date_int,
                                          1 => datemo = date_int as u32,
                                          2 => dateday = date_int as u32,
                                          3 => datehr = date_int as i32,
                                          4 => datemin = date_int as i32,
                                          5 => datesec = date_int as i32,
                                          _ => baddate1 = 1,
                                       }
                                    }
                               }
                               if baddate1 == 0 {
                                   let datexx = Local.ymd_opt(dateyr, datemo, dateday);
                                   if datexx == LocalResult::None {
                                       baddate1 = 1;
                                   } else {
                                       if (datehr < 0) | (datehr > 23) {
                                            baddate1 = 1;
                                       } else if (datemin < 0) | (datemin > 59) {
                                            baddate1 = 1;
                                       } else if (datesec < 0) | (datesec > 59) {
                                            baddate1 = 1;
                                       }
                                   }
                               }
                               if baddate1 == 0 {
                                   dateto = Utc.ymd(dateyr, datemo, dateday).and_hms_milli(datehr as u32, datemin as u32, datesec as u32, 0);
                                   filenewnamestr = format!("{}_500_{}", dateto.format("%Y_%m_%d_%H_%M_%S"), datesubstrfx);
//                                  let filenewnameln = filenewnamestr.len();
//                                  let filenewnameend = filenewnameln - 1;
//                                  let filenewnamestart = 0;
//                                  filenewnamex = filenewnamestr.get(filenewnamestart..filenewnameend).unwrap();
                                   filenewnamex = &filenewnamestr;
                                   filelocx = "Date Chg";
                               } else {
                                   messageval_label.set_markup("<span color=\"#FF000000\">********* BAD DATE in assign date **********</span>");
                                   break;
                               }
                           }
                       }
                   }
                   if bolok {
                       new_model.insert_with_values(None,
                              &[FIRST_COL as u32, SECOND_COL as u32, THIRD_COL as u32, FORTH_COL as u32, FIFTH_COL as u32,],
                              &[&filenamex, &filelocx, &filecurx, &fileassx, &filenewnamex]);
                   }
            }
            ctree_view1.set_model(Some(&new_model));
            let messvalx = format!("number of rows {:?}", selectiont.count_selected_rows());
            messageval_label.set_text(&messvalx);
        }
    }));
//----------------- update selection button end -----------------------------------

//----------------- convert copy button start -----------------------------------
    cexchg_button.connect_clicked(glib::clone!(@weak cdirectory1_combobox, @weak ctree_view1, @weak progress_progressbar, @weak messageval_label  => move|_| {

        let mut bolok = true;
        let mut str_cur_dir1 = format!(" ");
        let mut str_cur_dir_o = format!(" ");

// check if both directories exist and they are not equal
        if bolok {
            if let Some(cur_dir1) = cdirectory1_combobox.get_active_text() {
                str_cur_dir1 = cur_dir1.to_string();
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* convert COPY: ERROR GETTING FROM DIRECTORY IN COMBOBOX **********</span>");
                bolok = false;
            }
        }

// check if outdirectory has files (must not have files)
        if bolok {
            if let Some(cur_dir_o) = cdirectory_o_combobox.get_active_text() {
                str_cur_dir_o = cur_dir_o.to_string();
                for entry1 in fs::read_dir(&str_cur_dir_o).unwrap() {
                     let entry = entry1.unwrap();
                     if let Ok(metadata) = entry.metadata() {
                         if let Ok(_file_name) = entry.file_name().into_string() {
                             if metadata.is_file() {
                                 messageval_label.set_markup("<span color=\"#FF000000\">********* convert COPY: OUTPUT DIRECTORY HAS FILES IN IT **********</span>");
                                 bolok = false;
                             }
                         }
                     }
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* convert COPY: ERROR GETTING OUT DIRECTORY IN COMBOBOX  **********</span>");
                bolok = false;
           }
        }

// check if merge files and if so process
        if bolok {
            let view3model = ctree_view1.get_model();
            if view3model == None {
                messageval_label.set_markup("<span color=\"#FF000000\">********* convert Copy: ERROR NOTHING IN MERGE LIST **********</span>");
            } else {
                progress_progressbar.set_fraction(0.0);
                while glib::MainContext::pending(&glib::MainContext::default()) {
                    glib::MainContext::iteration(&glib::MainContext::default(),true);
                }
                let view3modeluw = view3model.unwrap();
                let mut valid = true;
                let validval = view3modeluw.get_iter_first().unwrap();
                let mut numrow = 0;
                let numchildren = view3modeluw.iter_n_children(None);
                while valid {
                      let fileassval = view3modeluw.get(&validval,3).get::<String>();
                      let fileassstr = format!("{:?}", fileassval);
                      let fileassln = fileassstr.len();
                      let fileassend = fileassln - 3;
                      let fileassstart = 9;
                      let fileassx = fileassstr.get(fileassstart..fileassend).unwrap();
                      if fileassx != "-" {
                          let filetoval = view3modeluw.get(&validval,4).get::<String>();
                          let filetostr = format!("{:?}", filetoval);
                          let filetoln = filetostr.len();
                          let filetoend = filetoln - 3;
                          let filetostart = 9;
                          let filetox = filetostr.get(filetostart..filetoend).unwrap();
                          if filetox != "-" {
                              let dateto;
                              let mut dateyr = 0;
                              let mut datemo = 0;
                              let mut dateday = 0;
                              let mut datehr = 0;
                              let mut datemin = 0;
                              let mut datesec = 0;
                              let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
                              let after = re.replace_all(&fileassx, "_");
                        
                              let listdatex: Vec<&str> = after.split("_").collect();
                              let lendat2 = listdatex.len();
                              let mut baddate1 = 0;
                              for indl in 0..lendat2 {
                                   let date_int: i32 = listdatex[indl].clone().parse().unwrap_or(-9999);
                                   if date_int == -9999 {
                                       baddate1 = 1;
                                       break;
                                   } else {
                                       match indl {
                                          0 => dateyr = date_int,
                                          1 => datemo = date_int as u32,
                                          2 => dateday = date_int as u32,
                                          3 => datehr = date_int as i32,
                                          4 => datemin = date_int as i32,
                                          5 => datesec = date_int as i32,
                                          _ => baddate1 = 1,
                                       }
                                   }
                              }
                              if baddate1 == 0 {
                                  let datexx = Local.ymd_opt(dateyr, datemo, dateday);
                                  if datexx == LocalResult::None {
                                      baddate1 = 1;
                                  } else {
                                      if (datehr < 0) | (datehr > 23) {
                                          baddate1 = 1;
                                      } else if (datemin < 0) | (datemin > 59) {
                                          baddate1 = 1;
                                      } else if (datesec < 0) | (datesec > 59) {
                                          baddate1 = 1;
                                      }
                                  }
                              }
// date in name end
//                          add the mod date values 
                              if baddate1 == 0 {
                                  dateto = Utc.ymd(dateyr, datemo, dateday).and_hms_milli(datehr as u32, datemin as u32, datesec as u32, 0);
                              } else {
                                  messageval_label.set_markup("<span color=\"#FF000000\">********* BAD ASSIGN DATE **********</span>");
                                  bolok = false;
                                  break;
                              }
                              let filefromval = view3modeluw.get(&validval,0).get::<String>();
                              let filefromstr = format!("{:?}", filefromval);
                              let filefromln = filefromstr.len();
                              let filefromend = filefromln - 3;
                              let filefromstart = 9;
                              let filefromx = filefromstr.get(filefromstart..filefromend).unwrap();
                              let fullfrom = str_cur_dir1.clone() + "/" + filefromx;

                              let fullto = str_cur_dir_o.clone() + "/" + filetox;


                              let arg2 = format!("-AllDates={}", dateto.format("%Y:%m:%d %H:%M:%S")); 

                              let _output = Command::new("exiftool")
                                                    .arg("-v")
                                                    .arg("-o")
                                                    .arg(&fullto)
                                                    .arg(&arg2)
                                                    .arg(&fullfrom)
                                                    .output()
                                                    .expect("failed to execute process");
                              numrow = numrow + 1;
                              let progressfr: f64 = numrow as f64 / numchildren as f64;
                              progress_progressbar.set_fraction(progressfr);
                              while glib::MainContext::pending(&glib::MainContext::default()) {
                                  glib::MainContext::iteration(&glib::MainContext::default(),true);
                              }
                          }
                      }
                      valid = view3modeluw.iter_next(&validval);
                }
                if bolok {
                    let messvalx = format!("copied {} files of {}", numrow, numchildren);
                    messageval_label.set_text(&messvalx);
                }
            }
        }
    }));

//----------------- convert copy button end -----------------------------------

//-------------------- connects end
}
