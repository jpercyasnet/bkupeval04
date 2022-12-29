extern crate gtk;
extern crate exif;
extern crate chrono;
extern crate regex;

use gtk::gdk;
use gtk::glib;

// use gtk::gdk_pixbuf::{Pixbuf};

use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Write, BufRead, BufReader};
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
    FileFilter,
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
    let wtitle = format!("Backup Evaluation Rust GTK4 version: {}.{}.{}",gtk::get_major_version(), gtk::get_minor_version(), gtk::get_micro_version());

    window.set_title(Some(&wtitle));
//    window.set_position(WindowPosition::Center);
    window.set_size_request(800, 500);

    let messagetitle_label = Label::new(Some("Message: "));
    gtk::WidgetExt::set_widget_name(&messagetitle_label, "MessTitle");
    let messageval_label = Label::new(Some("Message area"));

    let cdirectory1_button = Button::with_label("XML input file");
    let cdirectory1_combobox = ComboBoxText::new();
    cdirectory1_combobox.set_hexpand(true);

    let cdirectory_o_button = Button::with_label("Target Directory");
    let cdirectory_o_combobox = ComboBoxText::new();
    cdirectory_o_combobox.set_hexpand(true);
    let cnumrows_label = Label::new(Some("number of rows:"));
    let cnumrows_entry = Entry::new();
    let cgetrows_button = Button::with_label("get total rows");

//    let ctree_view1 = TreeView::new();
//    let seltree = ctree_view1.get_selection();
//    seltree.set_mode(gtk::SelectionMode::Multiple); // note 5
//    let ccolumn10 = TreeViewColumn::new();
//    let ccolumn11 = TreeViewColumn::new();
//    let ccolumn12 = TreeViewColumn::new();
//    let ccolumn13 = TreeViewColumn::new();
//    let ccolumn14 = TreeViewColumn::new();
//    let ccell10 = CellRendererText::new();
//    let ccell11 = CellRendererText::new();
//    let ccell12 = CellRendererText::new();
//    let ccell13 = CellRendererText::new();
//    let ccell14 = CellRendererText::new();
//    ccolumn10.pack_start(&ccell10, true);
//    ccolumn11.pack_start(&ccell11, true);
//    ccolumn12.pack_start(&ccell12, true);
//    ccolumn13.pack_start(&ccell13, true);
//    ccolumn14.pack_start(&ccell14, true);
    // Assiciate view's column with model's id column
//    ccolumn10.add_attribute(&ccell10, "text", 0);
//    ccolumn11.add_attribute(&ccell11, "text", 1);
//    ccolumn12.add_attribute(&ccell12, "text", 2);
//    ccolumn13.add_attribute(&ccell13, "text", 3);
//    ccolumn14.add_attribute(&ccell14, "text", 4);
//    ccolumn10.set_title("Name");
//    ccolumn11.set_title("Date From");
//    ccolumn12.set_title("Current Date");
//    ccolumn13.set_title("Assign Date");
//    ccolumn14.set_title("New Name");
//    ctree_view1.append_column(&ccolumn10);
//    ctree_view1.append_column(&ccolumn11);
//   ctree_view1.append_column(&ccolumn12);
//    ctree_view1.append_column(&ccolumn13);
//    ctree_view1.append_column(&ccolumn14);

//    let cscroll_window1 = ScrolledWindow::new();
//    let cscroll_window1 = ScrolledWindow::new(None , None);
//    cscroll_window1.set_child(Some(&ctree_view1));
//    cscroll_window1.set_hexpand(true);
//    cscroll_window1.set_vexpand(true);
    let ctarget_label = Label::new(Some("Target file name:"));
    let ctarget_entry = Entry::new();
    ctarget_entry.set_text("target.cdlist");
    let cexconv_button = Button::with_label("Execute Conversion");


    let progress_progressbar = ProgressBar::new();
    progress_progressbar.set_show_text(true);
    gtk::WidgetExt::set_widget_name(&progress_progressbar, "bar1");


    let vbox = Grid::new();
    vbox.set_column_spacing(5);
    vbox.set_row_spacing(5);
//    item, column, row, column length, row length
    vbox.attach(&messagetitle_label, 1, 0 , 1, 1);
    vbox.attach(&messageval_label, 2, 0 , 8, 1);
//    vbox.attach(&cdir1box_check, 0, 1 , 1, 1);
    vbox.attach(&cdirectory1_button, 1, 1 , 2, 1);
    vbox.attach(&cdirectory1_combobox, 3, 1 , 3, 1);
    vbox.attach(&cdirectory_o_button, 6, 1 , 2, 1);
    vbox.attach(&cdirectory_o_combobox, 8, 1 , 2, 1);
//    vbox.attach(&cdatenamebox_check, 1, 2 , 1, 1);
//    vbox.attach(&csourcedirbox_check, 3, 2 , 1, 1);
//    vbox.attach(&csourcedirval_label, 4, 2 , 1, 1);
//    vbox.attach(&ctargetdirbox_check, 8, 2 , 1, 1);
//    vbox.attach(&ctargetdirval_label, 9, 2 , 1, 1);
    vbox.attach(&cnumrows_label, 2, 3 , 1, 1);
//    vbox.attach(&crenamedatebox_check, 1, 3, 1, 1);
   vbox.attach(&cgetrows_button, 1, 4 , 1, 1);    
    vbox.attach(&cnumrows_entry, 2, 4 , 1, 1);
//    vbox.attach(&coffsetbox_check, 3, 5 , 1, 1);
    vbox.attach(&ctarget_label, 6, 3 , 1, 1);
    vbox.attach(&ctarget_entry, 8, 3 , 2, 1);
//    vbox.attach(&cscroll_window1, 0, 6 , 10, 4); 
    vbox.attach(&cexconv_button, 9, 10 , 1, 1); 
//    vbox.attach(&cupallbox_check, 4, 10, 1, 1);
//    vbox.attach(&cupall_button, 5, 10 , 1, 1); 
//    vbox.attach(&cresetbox_check, 8, 10, 1, 1);
//    vbox.attach(&creset_button, 9, 10 , 1, 1);    
    vbox.attach(&progress_progressbar, 0, 13 , 10, 1);

    window.set_child(Some(&vbox));
    window.set_destroy_with_parent(true);
    window.show(); 

//----------------- source directory  button start -----------------------------------
    cdirectory1_button.connect_clicked(glib::clone!(@weak window, @weak cdirectory1_combobox, @weak messageval_label => move|_| {
        
            messageval_label.set_text("getting directory");

            let dialog = FileChooserDialog::new(
                Some("Choose a XML file"),
                Some(&window),
                FileChooserAction::Open,
                &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
            );
            let filter = FileFilter::new();
            filter.add_pattern("*.xml");
            dialog.set_filter(&filter);

            dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
               if response == gtk::ResponseType::Ok {
                 let mut baddate1 = 0;
                 if let Some(filename) = d.get_file() {
                   if let Some(filepath) = filename.get_path() {
                     cdirectory1_combobox.prepend_text(&filepath.display().to_string());
                     cdirectory1_combobox.set_active(Some(0));
                     messageval_label.set_text("XML file selected");
                   } else {
                     messageval_label.set_markup("<span color=\"#FF000000\">********* Directory : ERROR GETTING file path **********</span>");
                   }
                 } else { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* Directory : ERROR GETTING file **********</span>");
                 }
               }
               if messageval_label.get_text() == "getting directory" {
                   messageval_label.set_markup("<span color=\"#FF000000\">********* Directory: ERROR  OPEN  button not selected **********</span>");
               }
               d.close();
            });
            dialog.show();

      
    }));
//----------------- source directory  button end -----------------------------------

//----------------- target directory button start -----------------------------------
    cdirectory_o_button.connect_clicked(glib::clone!(@weak window, @weak cdirectory_o_combobox, @weak ctarget_entry, @weak cdirectory1_combobox, @weak messageval_label => move|_| {
    
        messageval_label.set_text("getting directory");

        let dialog = FileChooserDialog::new(
            Some("Choose ouput  Directory"),
            Some(&window),
            FileChooserAction::SelectFolder,
            &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
        );
        dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
          if response == gtk::ResponseType::Ok {
            if let Some(foldername) = d.get_file() {
              if let Some(folderpath) = foldername.get_path() {
                     cdirectory_o_combobox.prepend_text(&folderpath.display().to_string());
                     cdirectory_o_combobox.set_active(Some(0));
                     messageval_label.set_text("Target folder selected");
              } else {
                     messageval_label.set_markup("<span color=\"#FF000000\">********* Directory : ERROR GETTING folder path **********</span>");
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
//----------------- get rows button start -----------------------------------
    cgetrows_button.connect_clicked(glib::clone!(@weak cdirectory1_combobox, @weak cnumrows_entry, @weak progress_progressbar, @weak messageval_label  => move|_| {
        if let Some(filename) = cdirectory1_combobox.get_active_text() {
            let str_filename = filename.to_string();
            if Path::new(&str_filename).exists() {
                progress_progressbar.set_fraction(0.5);
                while glib::MainContext::pending(&glib::MainContext::default()) {
                    glib::MainContext::iteration(&glib::MainContext::default(),true);
                }
                // Open the file in read-only mode (ignoring errors).
                let mut count;
                {  
                   let file = File::open(str_filename).unwrap();
                    // uses a reader buffer
                    let mut reader = BufReader::new(file);
                    count = reader.lines().fold(0, |sum, _| sum + 1);
//                    println!("line count: {}", count);
                }
                let numrowtext = format!("{}",count);
                cnumrows_entry.set_text(&numrowtext);
                messageval_label.set_text("number of rows has been set");
                progress_progressbar.set_fraction(1.0);
                while glib::MainContext::pending(&glib::MainContext::default()) {
                    glib::MainContext::iteration(&glib::MainContext::default(),true);
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* source file does not exist **********</span>");
            }
                
        } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* ERROR GETTING FROM DIRECTORY IN COMBOBOX **********</span>");
        }
    }));
//----------------- get rows button end -----------------------------------
    
//----------------- convert button start -----------------------------------
    cexconv_button.connect_clicked(glib::clone!(@weak cdirectory1_combobox, @weak cdirectory_o_combobox, @weak ctarget_entry, @weak cnumrows_entry, @weak progress_progressbar, @weak messageval_label  => move|_| {
// files must be in utf-8 format
// linux command file -i filename will show format
// linux command iconv -f format -t UTF-8 filename -o outputfile    will convert file to UTF-8   
        let mut bolok = true;
        let mut numrows: i64 = 1;
        let mut targetfullname = format!("");
        progress_progressbar.set_fraction(0.5);
        while glib::MainContext::pending(&glib::MainContext::default()) {
               glib::MainContext::iteration(&glib::MainContext::default(),true);
        }
        if let Some(dirname) = cdirectory_o_combobox.get_active_text() {
            let str_dirname = dirname.to_string();
            if Path::new(&str_dirname).exists() {
                let strtarget = ctarget_entry.get_text();
                if strtarget.len() < 4 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* target name less than 4 characters **********</span>");
                    bolok = false;
                } else {
                    if !strtarget.contains(".") { 
                        messageval_label.set_markup("<span color=\"#FF000000\">********* target name does not have a file type (ie xx.xxx) **********</span>");
                        bolok = false;
                    } else {
                        let mut lrperpos = strtarget.rfind(".").unwrap();
                        if (strtarget.len() - lrperpos) < 4 {
                            messageval_label.set_markup("<span color=\"#FF000000\">********* target name does not have a valid type (must be at least 3 characters **********</span>");
                            bolok = false;
                        } else {
                            let mut lfperpos = strtarget.find(".").unwrap();
                            if lfperpos < 3 {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* target name is least than 3 characters **********</span>");
                                bolok = false;
                            } else {
                                targetfullname = format!("{}/{}", str_dirname, strtarget);
                                if Path::new(&targetfullname).exists() {
                                    messageval_label.set_markup("<span color=\"#FF000000\">********* target name already exists **********</span>");
                                    bolok = false;
                                }
                            }
                        }
                    }
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* target directory does not exist **********</span>");
                bolok = false;
            }
                
        } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* ERROR GETTING TARGET DIRECTORY IN COMBOBOX **********</span>");
            bolok = false;
        }
        if bolok {
            let strnumrows = cnumrows_entry.get_text();
            numrows = strnumrows.parse().unwrap_or(-99);
            if numrows < 10 {
                messageval_label.set_markup("<span color=\"#FF000000\">********* INVALID NUMBER IN NUMBER OF ROWS ENTRY **********</span>");
                bolok = false;
            }
        }
        if bolok {         
          if let Some(filename) = cdirectory1_combobox.get_active_text() {
            let str_filename = filename.to_string();
            let str_filenamex = str_filename.clone();
            if Path::new(&str_filename).exists() {
                // Open the file in read-only mode (ignoring errors).
                let file = File::open(str_filenamex).unwrap(); 
                let mut reader = BufReader::new(file);
                let mut targetfile = File::create(targetfullname).unwrap();
                let mut line = String::new();
                let mut linenum = 0;
                let mut slevel = " ";
                let mut sCd = String::new();
                let mut sDir = String::new();
                let mut sFile = String::new();
                let mut sDate = String::new();
                let mut sAttr = String::new();
                let mut sSize = format!("");
                let mut lfilesize = 0;
            	let mut shexval = format!("");
                loop {
                   match reader.read_line(&mut line) {
                      Ok(bytes_read) => {
                          // EOF: save last file address to restart from this address for next run
                          if bytes_read == 0 {
                              break;
                          }
                          linenum = linenum + 1;
                          if line.contains("<Cd>") {
                              slevel = "Cd";
//                              println!("{}. {}", linenum, line);
                          } else if line.contains("<Directory>") {
                              slevel = "Dir";
//                              println!("{}. {}", linenum, line);                         
                          } else if line.contains("<File>") {
                              slevel = "File";
//                              println!("{}. {}", linenum, line);                         
                          } else if line.contains("</File>") {
                              slevel = "Dir";
                              let stroutput = format!("{}{} {:02}{}{:03}{}{:03}{}{:02}{}{:02}{}",
                                                      shexval,
                                                      sSize,
                                                      sCd.len(),
                                                      sCd,
                                                      sDir.len(),
                                                      sDir,
                                                      sFile.len(),
                                                      sFile,
                                                      sDate.len(),
                                                      sDate,
                                                      sAttr.len(),
                                                      sAttr);
                              writeln!(&mut targetfile, "{}", stroutput).unwrap();
//                              println!("{}. {} write out file", linenum, line);                         
                          } else if line.contains("<Name>") {
                              let mut lCurrPos = line.find("<Name>").unwrap();
                              let lCurrPos1 = line.find("</Name>").unwrap();
         					  let lLen = lCurrPos1 - lCurrPos - 6;
         					  lCurrPos = lCurrPos + 6;
         					  let nameval;
         					  if (lCurrPos1 != 0) & (lLen > 0) {
         					      nameval = line.get(lCurrPos..(lCurrPos+lLen)).unwrap();
         					  } else {
         					      nameval = "***no /Name or null value***";
         					  }                       
//         					  println!("{}. {}", linenum, line);
            				  if slevel == "Cd" {
            			          sCd = nameval.to_string();
//                                  println!("{}. {}", slevel, sCd);                         
            				  } else if slevel == "Dir" {
         					      if (lCurrPos1 != 0) & (lLen > 0) {
         					          sDir = nameval.to_string();
         					      } else {
         						      sDir = "/".to_string();
         					      }                       
//                                  println!("{}. {}", slevel, sDir);                         
            				  } else if slevel == "File" {
            				      shexval = format!("");
            				      for b in nameval.bytes() {
            				         shexval = format!("{}{:02X}", shexval, b);
                                  }
                                  if nameval.len() > 255 {
                                    shexval = shexval.get(0..511).unwrap().to_string();
                                  } else {
                                    for c in 0..(256 - nameval.len()) {
            				           shexval = format!("{}00", shexval);
            				        }
            				      }
//                                  println!("hex value length: {} {}", shexval.len(), shexval.get(0..20).unwrap().to_string());
            				      sFile = nameval.to_string();
//                                  println!("{}. {}", slevel, sFile);
                              } else {
//                                  println!("****error: no level set and Name is shown ****");
            		          }
                          } else if line.contains("<FullName>") {
                              let mut lCurrPos = line.find("<FullName>").unwrap();
                              let lCurrPos1 = line.find("</FullName>").unwrap();
        					  let lLen = lCurrPos1 - lCurrPos - 10;
         					  lCurrPos = lCurrPos + 10;
         					  let nameval;
         					  if slevel == "Dir" {
         					  	  if (lCurrPos1 != 0) & (lLen > 0) {
         					          nameval = line.get(lCurrPos..(lCurrPos+lLen)).unwrap();
         					          sDir = nameval.to_string();
//           					          println!("sDir: {}", sDir);
         					      }
         					  }                       
//         					  println!("{}. {}", linenum, line);
                          } else {
                              if slevel == "File" {
                                  if line.contains("<Date>") {
                                      let mut lCurrPos = line.find("<Date>").unwrap();
                                      let lCurrPos1 = line.find("</Date>").unwrap();
        					          let lLen = lCurrPos1 - lCurrPos - 6;
         					          lCurrPos = lCurrPos + 6;
         					          let nameval;
         					          if (lCurrPos1 != 0) & (lLen > 0) {
         					              nameval = line.get(lCurrPos..(lCurrPos+lLen)).unwrap();
                                          sDate = nameval.to_string();
                                      } else {
                                          sDate = "***no /Date or null value***".to_string();
                                      }
//                                      println!("date: {}", sDate);
//         					          println!("{}. {}", linenum, line);                                 
                                  } else if line.contains("<Attr>") {
                                      let mut lCurrPos = line.find("<Attr>").unwrap();
                                      let lCurrPos1 = line.find("</Attr>").unwrap();
        					          let lLen = lCurrPos1 - lCurrPos - 6;
         					          lCurrPos = lCurrPos + 6;
         					          let nameval;
         					          if (lCurrPos1 != 0) & (lLen > 0) {
         					              nameval = line.get(lCurrPos..(lCurrPos+lLen)).unwrap();
                                          sAttr = nameval.to_string();
                                      } else {
                                          sAttr = "***no /Attr or null value***".to_string();
                                      }
//                                      println!("attr: {}", sAttr);
//         					          println!("{}. {}", linenum, line);
                                  } else if line.contains("<Size>") {
                                      let mut lCurrPos = line.find("<Size>").unwrap();
                                      let lCurrPos1 = line.find("</Size>").unwrap();
        					          let lLen = lCurrPos1 - lCurrPos - 6;
         					          lCurrPos = lCurrPos + 6;
         					          let nameval;
         					          if (lCurrPos1 != 0) & (lLen > 0) {
         					              nameval = line.get(lCurrPos..(lCurrPos+lLen)).unwrap();
         					              let test_int: i64 = nameval.parse().unwrap_or(-99);
         					              if test_int >= 0 {
         					                  sSize = format!("{:016}",test_int);
         					              } else {
         					                  sSize = format!("invalid size value: {}", nameval);
         					              }
                                      } else {
                                          sSize = format!("***no /Size or null value***");
                                      }
//                                      println!("size: {}", sSize);
//         					          println!("{}. {}", linenum, line);
         					      }                            
                              } 
                          let progressfr: f64 = linenum as f64 / numrows as f64;
                          progress_progressbar.set_fraction(progressfr);
                          while glib::MainContext::pending(&glib::MainContext::default()) {
                                 glib::MainContext::iteration(&glib::MainContext::default(),true);
                          }
                          }
                          if linenum > numrows {
                              break;
                          }
                          // do not accumulate data
                          line.clear();
                      }
                      Err(err) => {
                          messageval_label.set_markup("<span color=\"#FF000000\">********* error reading xml file **********</span>");
                          bolok = false;   
                          break;
                      }
                   };
                }
                if bolok {
                    messageval_label.set_text("source file exists and read");
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* source file does not exist **********</span>");
                bolok = false;
            }
                
          } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* ERROR GETTING XML FILE IN COMBOBOX **********</span>");
            bolok = false;
          }
        }
    }));

//----------------- convert button end -----------------------------------

//-------------------- connects end
}
