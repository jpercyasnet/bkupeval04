extern crate gtk;
extern crate exif;
extern crate chrono;
extern crate regex;
extern crate walkdir;

use gtk::gdk;
use gtk::glib;

// use gtk::gdk_pixbuf::{Pixbuf};

use std::fs;
use std::path::{Path, PathBuf};
use std::io::{Write, BufRead, BufReader};
use std::fs::File;
use std::process::Command;
use walkdir::WalkDir;

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
    Notebook,
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
    
    let progress_progressbar = ProgressBar::new();
    progress_progressbar.set_show_text(true);
    gtk::WidgetExt::set_widget_name(&progress_progressbar, "bar1");


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

    let vbox1 = Grid::new();
    vbox1.set_column_spacing(5);
    vbox1.set_row_spacing(5);
//    item, column, row, column length, row length
//    vbox.attach(&cdir1box_check, 0, 1 , 1, 1);
    vbox1.attach(&cdirectory1_button, 1, 1 , 2, 1);
    vbox1.attach(&cdirectory1_combobox, 3, 1 , 3, 1);
    vbox1.attach(&cdirectory_o_button, 6, 1 , 2, 1);
    vbox1.attach(&cdirectory_o_combobox, 8, 1 , 2, 1);
//    vbox.attach(&cdatenamebox_check, 1, 2 , 1, 1);
//    vbox.attach(&csourcedirbox_check, 3, 2 , 1, 1);
//    vbox.attach(&csourcedirval_label, 4, 2 , 1, 1);
//    vbox.attach(&ctargetdirbox_check, 8, 2 , 1, 1);
//    vbox.attach(&ctargetdirval_label, 9, 2 , 1, 1);
    vbox1.attach(&cnumrows_label, 2, 3 , 1, 1);
//    vbox.attach(&crenamedatebox_check, 1, 3, 1, 1);
    vbox1.attach(&cgetrows_button, 1, 4 , 1, 1);    
    vbox1.attach(&cnumrows_entry, 2, 4 , 1, 1);
//    vbox.attach(&coffsetbox_check, 3, 5 , 1, 1);
    vbox1.attach(&ctarget_label, 6, 3 , 1, 1);
    vbox1.attach(&ctarget_entry, 8, 3 , 2, 1);
//    vbox.attach(&cscroll_window1, 0, 6 , 10, 4); 
    vbox1.attach(&cexconv_button, 9, 10 , 1, 1); 
//    vbox.attach(&cupallbox_check, 4, 10, 1, 1);
//    vbox.attach(&cupall_button, 5, 10 , 1, 1); 
//    vbox.attach(&cresetbox_check, 8, 10, 1, 1);
//    vbox.attach(&creset_button, 9, 10 , 1, 1);    
    let vnotebook = Notebook::new();
    let tab1_label = Label::new(Some("XML convert"));
    vnotebook.append_page(&vbox1, Some(&tab1_label));

    let hdirectory1_button = Button::with_label("Hard Drive directory");
    let hdirectory1_combobox = ComboBoxText::new();
    hdirectory1_combobox.set_hexpand(true);
    
    let hdirectory_o_button = Button::with_label("Target Directory");
    let hdirectory_o_combobox = ComboBoxText::new();
    hdirectory_o_combobox.set_hexpand(true);
    let htarget_label = Label::new(Some("Target file name:"));
    let htarget_entry = Entry::new();
    htarget_entry.set_text("target.hdlist");
    let href_label = Label::new(Some("   Reference name:"));
    let href_entry = Entry::new();
    href_entry.set_text("HDname");


    let hexget_button = Button::with_label("Execute get directory list");
    
    let vbox2 = Grid::new();
    vbox2.set_column_spacing(5);
    vbox2.set_row_spacing(5);
//    item, column, row, column length, row length
//    vbox.attach(&cdir1box_check, 0, 1 , 1, 1);
    vbox2.attach(&hdirectory1_button, 1, 1 , 2, 1);
    vbox2.attach(&hdirectory1_combobox, 3, 1 , 3, 1);
    vbox2.attach(&hdirectory_o_button, 6, 1 , 2, 1);
    vbox2.attach(&hdirectory_o_combobox, 8, 1 , 2, 1);
    vbox2.attach(&htarget_label, 6, 3 , 1, 1);
    vbox2.attach(&htarget_entry, 8, 3 , 2, 1);
    vbox2.attach(&href_label, 1, 4 , 1, 1);    
    vbox2.attach(&href_entry, 2, 4 , 2, 1);


    vbox2.attach(&hexget_button, 9, 10 , 1, 1); 

    let tab2_label = Label::new(Some("HD directory"));
    vnotebook.append_page(&vbox2, Some(&tab2_label));
    
    let edirectory1_button = Button::with_label("CD list file");
    let edirectory1_combobox = ComboBoxText::new();
    edirectory1_combobox.set_hexpand(true);
    let edirectory2_button = Button::with_label("HD list file");
    let edirectory2_combobox = ComboBoxText::new();
    edirectory2_combobox.set_hexpand(true);
    
    let edirectory_o_button = Button::with_label("Target Directory");
    let edirectory_o_combobox = ComboBoxText::new();
    edirectory_o_combobox.set_hexpand(true);
    let esame_label = Label::new(Some("same file name:"));
    let esame_entry = Entry::new();
    esame_entry.set_text("same.slist");
    let ediff_label = Label::new(Some("different size file name:"));
    let ediff_entry = Entry::new();
    ediff_entry.set_text("diff.dlist");
    let enf_label = Label::new(Some("not found file name:"));
    let enf_entry = Entry::new();
    enf_entry.set_text("notfound.nlist");
    let eexeval_button = Button::with_label("Execute evaluate HD with CD");
    let enumrows_label = Label::new(Some("number of hd rows:"));
    let enumrows_entry = Entry::new();
    let egetrows_button = Button::with_label("get total rows");
     
    let vbox3 = Grid::new();
    vbox3.set_column_spacing(5);
    vbox3.set_row_spacing(5);
//    item, column, row, column length, row length
//    vbox.attach(&cdir1box_check, 0, 1 , 1, 1);
    vbox3.attach(&edirectory1_button, 1, 1, 2, 1);
    vbox3.attach(&edirectory1_combobox, 3, 1, 3, 1);
    vbox3.attach(&edirectory2_button, 1, 2, 2, 1);
    vbox3.attach(&edirectory2_combobox, 3, 2, 3, 1);
    vbox3.attach(&enumrows_label, 2, 3 , 1, 1);
//    vbox.attach(&crenamedatebox_check, 1, 3, 1, 1);
    vbox3.attach(&egetrows_button, 1, 4 , 1, 1);    
    vbox3.attach(&enumrows_entry, 2, 4 , 1, 1);
    vbox3.attach(&edirectory_o_button, 6, 1, 2, 1);
    vbox3.attach(&edirectory_o_combobox, 8, 1, 2, 1);
    vbox3.attach(&esame_label, 6, 3, 1, 1);
    vbox3.attach(&esame_entry, 8, 3, 2, 1);
    vbox3.attach(&ediff_label, 6, 4, 1, 1);
    vbox3.attach(&ediff_entry, 8, 4, 2, 1);
    vbox3.attach(&enf_label, 6, 5, 1, 1);
    vbox3.attach(&enf_entry, 8, 5, 2, 1);

    vbox3.attach(&eexeval_button, 1, 10, 1, 1); 
  
    let tab3_label = Label::new(Some("Eval HD with CD"));
    vnotebook.append_page(&vbox3, Some(&tab3_label));   

    let vbox = Grid::new();
    vbox.set_column_spacing(5);
    vbox.set_row_spacing(5);
    
    vbox.attach(&messagetitle_label, 1, 0 , 1, 1);
    vbox.attach(&messageval_label, 2, 0 , 8, 1);
    vbox.attach(&vnotebook, 0, 2, 10, 10);
    vbox.attach(&progress_progressbar, 0, 13 , 10, 1);

    window.set_child(Some(&vbox));
    window.set_destroy_with_parent(true);
    window.show(); 

//----------------- source file  button start -----------------------------------
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
//----------------- source file  button end -----------------------------------

//----------------- target directory button start -----------------------------------
    cdirectory_o_button.connect_clicked(glib::clone!(@weak window, @weak cdirectory_o_combobox, @weak messageval_label => move|_| {
    
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
                              let stroutput = format!("{}{} {:02}{}{:03}{}{:03}{}{:02}{}",
                                                      shexval,
                                                      sSize,
                                                      sCd.len(),
                                                      sCd,
                                                      sDir.len(),
                                                      sDir,
                                                      sFile.len(),
                                                      sFile,
                                                      sDate.len(),
                                                      sDate);
                              writeln!(&mut targetfile, "{}", stroutput).unwrap();
                              let progressfr: f64 = linenum as f64 / numrows as f64;
                              progress_progressbar.set_fraction(progressfr);
                              while glib::MainContext::pending(&glib::MainContext::default()) {
                                     glib::MainContext::iteration(&glib::MainContext::default(),true);
                              }
                              sFile = format!("");
                              sDate = format!("");
                              sSize = format!("");
            	              shexval = format!("");
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
//----------------- harddrive directory button start -----------------------------------
    hdirectory1_button.connect_clicked(glib::clone!(@weak window, @weak hdirectory1_combobox, @weak messageval_label => move|_| {
    
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
                     hdirectory1_combobox.prepend_text(&folderpath.display().to_string());
                     hdirectory1_combobox.set_active(Some(0));
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
//----------------- harddrive directory button end -----------------------------------

//----------------- target directory button start -----------------------------------
    hdirectory_o_button.connect_clicked(glib::clone!(@weak window, @weak hdirectory_o_combobox, @weak messageval_label => move|_| {
    
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
                     hdirectory_o_combobox.prepend_text(&folderpath.display().to_string());
                     hdirectory_o_combobox.set_active(Some(0));
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
//----------------- get directory list button start -----------------------------------
    hexget_button.connect_clicked(glib::clone!(@weak hdirectory1_combobox, @weak hdirectory_o_combobox, @weak href_entry, @weak htarget_entry, @weak progress_progressbar, @weak messageval_label  => move|_| {
// files must be in utf-8 format
// linux command file -i filename will show format
// linux command iconv -f format -t UTF-8 filename -o outputfile    will convert file to UTF-8   
        let mut bolok = true;
        let mut strref = String::new();
        let mut numrows: i64 = 1;
        let mut targetfullname = format!("");
        progress_progressbar.set_fraction(0.0);
        while glib::MainContext::pending(&glib::MainContext::default()) {
               glib::MainContext::iteration(&glib::MainContext::default(),true);
        }
        if let Some(dirname) = hdirectory_o_combobox.get_active_text() {
            let str_dirname = dirname.to_string();
            if Path::new(&str_dirname).exists() {
                let strtarget = htarget_entry.get_text();
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
            strref = href_entry.get_text().to_string();
            if strref.len() < 3 {
                messageval_label.set_markup("<span color=\"#FF000000\">********* reference name less than 3 characters **********</span>");
                bolok = false;
            } else {
                if strref.len() > 15 { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* reference name greater than 15 characters **********</span>");
                    bolok = false;
                } 
            }
        }
        if bolok {
          if let Some(dirname) = hdirectory1_combobox.get_active_text() {
            let str_dirname = dirname.to_string();
            if Path::new(&str_dirname).exists() {
                let mut targetfile = File::create(targetfullname).unwrap();
                for entry in WalkDir::new(&str_dirname).into_iter().filter_map(|e| e.ok()) {
                     if let Ok(metadata) = entry.metadata() {
                         if metadata.is_file() {
                             let fullpath = format!("{}",entry.path().display());
                             let mut lrperpos = fullpath.rfind("/").unwrap();
         					 let file_name = fullpath.get((lrperpos+1)..).unwrap();
         					 let file_dir = fullpath.get(0..(lrperpos)).unwrap();
                             let datetime: DateTime<Local> = metadata.modified().unwrap().into();
                             let file_date = format!("{}", datetime.format("%Y-%m-%d %T")); 
                             let file_len: u64 = metadata.len();
//                             println!("{}", entry.path().display());
//                             println!("dir:{}; name:{}; length:{}; date:{}", file_dir, file_name, file_len, file_date);
            				 let mut shexvalx = format!("");
            			     for b in file_name.bytes() {
            			        shexvalx = format!("{}{:02X}", shexvalx, b);
                             }
                             if file_name.len() > 255 {
                                 shexvalx = shexvalx.get(0..511).unwrap().to_string();
                             } else {
                                 for c in 0..(256 - file_name.len()) {
            				        shexvalx = format!("{}00", shexvalx);
            		             }
            	     	     }
         			         let sSizex = format!("{:016}", file_len);
                             let stroutput = format!("{}{} {:02}{}{:03}{}{:03}{}{:02}{}",
                                                  shexvalx,
                                                  sSizex,
                                                  strref.len(),
                                                  strref,
                                                  file_dir.len(),
                                                  file_dir,
                                                  file_name.len(),
                                                  file_name,
                                                  file_date.len(),
                                                  file_date);
                             writeln!(&mut targetfile, "{}", stroutput).unwrap();
                         }
                     }
                }
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* HD directory does not exist **********</span>");
                bolok = false;
            }
                
        } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* ERROR GETTING HD DIRECTORY IN COMBOBOX **********</span>");
            bolok = false;
        }
      }
    }));
//----------------- get directory list button start -----------------------------------

//----------------- cd  file  button start -----------------------------------
    edirectory1_button.connect_clicked(glib::clone!(@weak window, @weak edirectory1_combobox, @weak messageval_label => move|_| {
        
            messageval_label.set_text("getting directory");

            let dialog = FileChooserDialog::new(
                Some("Choose a cd file"),
                Some(&window),
                FileChooserAction::Open,
                &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
            );

            dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
               if response == gtk::ResponseType::Ok {
                 let mut baddate1 = 0;
                 if let Some(filename) = d.get_file() {
                   if let Some(filepath) = filename.get_path() {
                     edirectory1_combobox.prepend_text(&filepath.display().to_string());
                     edirectory1_combobox.set_active(Some(0));
                     messageval_label.set_text("cd file selected");
                   } else {
                     messageval_label.set_markup("<span color=\"#FF000000\">********* Directory : ERROR GETTING cd file path **********</span>");
                   }
                 } else { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* Directory : ERROR GETTING cd file **********</span>");
                 }
               }
               if messageval_label.get_text() == "getting directory" {
                   messageval_label.set_markup("<span color=\"#FF000000\">********* Directory: ERROR  OPEN  button not selected for cd file **********</span>");
               }
               d.close();
            });
            dialog.show();

    }));
//----------------- cd file  button end -----------------------------------

//----------------- hd  file  button start -----------------------------------
    edirectory2_button.connect_clicked(glib::clone!(@weak window, @weak edirectory2_combobox, @weak messageval_label => move|_| {
        
            messageval_label.set_text("getting directory");

            let dialog = FileChooserDialog::new(
                Some("Choose hd file"),
                Some(&window),
                FileChooserAction::Open,
                &[("Open", gtk::ResponseType::Ok), ("Cancel", gtk::ResponseType::Cancel)],
            );

            dialog.connect_response(move |d: &FileChooserDialog, response: gtk::ResponseType| {
               if response == gtk::ResponseType::Ok {
                 let mut baddate1 = 0;
                 if let Some(filename) = d.get_file() {
                   if let Some(filepath) = filename.get_path() {
                     edirectory2_combobox.prepend_text(&filepath.display().to_string());
                     edirectory2_combobox.set_active(Some(0));
                     messageval_label.set_text("hd file selected");
                   } else {
                     messageval_label.set_markup("<span color=\"#FF000000\">********* Directory : ERROR GETTING hd file path **********</span>");
                   }
                 } else { 
                    messageval_label.set_markup("<span color=\"#FF000000\">********* Directory : ERROR GETTING hd file **********</span>");
                 }
               }
               if messageval_label.get_text() == "getting directory" {
                   messageval_label.set_markup("<span color=\"#FF000000\">********* Directory: ERROR  OPEN  button not selected for hd file **********</span>");
               }
               d.close();
            });
            dialog.show();

    }));
//----------------- cd file  button end -----------------------------------

//----------------- target directory button start -----------------------------------
    edirectory_o_button.connect_clicked(glib::clone!(@weak window, @weak edirectory_o_combobox, @weak messageval_label => move|_| {
    
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
                     edirectory_o_combobox.prepend_text(&folderpath.display().to_string());
                     edirectory_o_combobox.set_active(Some(0));
                     messageval_label.set_text("Target folder selected");
              } else {
                     messageval_label.set_markup("<span color=\"#FF000000\">********* Directory : ERROR GETTING folder path for target **********</span>");
              }
            } else { 
                messageval_label.set_markup("<span color=\"#FF000000\">********* Directory: ERROR GETTING folder for target **********</span>");
            }
          }
          if messageval_label.get_text() == "getting directory" {
              messageval_label.set_markup("<span color=\"#FF000000\">********* Directory: ERROR  OPEN  button not selected for target **********</span>");
          }
          d.close();
        });
        dialog.show();
    }));
//----------------- target directory button end -----------------------------------
//----------------- get rows button start -----------------------------------
    egetrows_button.connect_clicked(glib::clone!(@weak edirectory2_combobox, @weak enumrows_entry, @weak progress_progressbar, @weak messageval_label  => move|_| {
        if let Some(filename) = edirectory2_combobox.get_active_text() {
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
                enumrows_entry.set_text(&numrowtext);
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
    
//----------------- get directory list button start -----------------------------------
    eexeval_button.connect_clicked(glib::clone!(@weak edirectory1_combobox, @weak edirectory2_combobox, @weak edirectory_o_combobox, @weak esame_entry, @weak ediff_entry, @weak enf_entry, @weak progress_progressbar, @weak messageval_label  => move|_| {
// files must be in utf-8 format
// linux command file -i filename will show format
// linux command iconv -f format -t UTF-8 filename -o outputfile    will convert file to UTF-8   
        let mut bolok = true;
        let mut samefullname = format!("");
        let mut difffullname = format!("");
        let mut nffullname = format!("");
        let mut cdfullname = format!("");
        let mut hdfullname = format!("");
        progress_progressbar.set_fraction(0.0);
        while glib::MainContext::pending(&glib::MainContext::default()) {
               glib::MainContext::iteration(&glib::MainContext::default(),true);
        }
        if let Some(dirname) = edirectory_o_combobox.get_active_text() {
            let str_dirname = dirname.to_string();
            if Path::new(&str_dirname).exists() {
                let strsame = esame_entry.get_text();
                if strsame.len() < 4 {
                    messageval_label.set_markup("<span color=\"#FF000000\">********* same name less than 4 characters **********</span>");
                    bolok = false;
                } else {
                    if !strsame.contains(".") { 
                        messageval_label.set_markup("<span color=\"#FF000000\">********* same name does not have a file type (ie xx.xxx) **********</span>");
                        bolok = false;
                    } else {
                        let mut lrperpos = strsame.rfind(".").unwrap();
                        if (strsame.len() - lrperpos) < 4 {
                            messageval_label.set_markup("<span color=\"#FF000000\">********* same name does not have a valid type (must be at least 3 characters **********</span>");
                            bolok = false;
                        } else {
                            let mut lfperpos = strsame.find(".").unwrap();
                            if lfperpos < 3 {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* same name is least than 3 characters **********</span>");
                                bolok = false;
                            } else {
                                samefullname = format!("{}/{}", str_dirname, strsame);
                                if Path::new(&samefullname).exists() {
                                    messageval_label.set_markup("<span color=\"#FF000000\">********* same name already exists **********</span>");
                                    bolok = false;
                                }
                            }
                        }
                    }
                }
                if bolok {
                    let strdiff = ediff_entry.get_text();
                    if strdiff.len() < 4 {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* different name less than 4 characters **********</span>");
                        bolok = false;
                    } else {
                        if !strdiff.contains(".") { 
                            messageval_label.set_markup("<span color=\"#FF000000\">********* different name does not have a file type (ie xx.xxx) **********</span>");
                            bolok = false;
                        } else {
                            let mut lrperpos = strdiff.rfind(".").unwrap();
                            if (strdiff.len() - lrperpos) < 4 {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* different name does not have a valid type (must be at least 3 characters **********</span>");
                                bolok = false;
                            } else {
                                let mut lfperpos = strdiff.find(".").unwrap();
                                if lfperpos < 3 {
                                    messageval_label.set_markup("<span color=\"#FF000000\">********* different name is least than 3 characters **********</span>");
                                    bolok = false;
                                } else {
                                    difffullname = format!("{}/{}", str_dirname, strdiff);
                                    if Path::new(&difffullname).exists() {
                                        messageval_label.set_markup("<span color=\"#FF000000\">********* different name already exists **********</span>");
                                        bolok = false;
                                    }
                                }
                            }
                        }
                    }
                }
                if bolok {
                    let strnf = enf_entry.get_text();
                    if strnf.len() < 4 {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* not found name less than 4 characters **********</span>");
                        bolok = false;
                    } else {
                        if !strnf.contains(".") { 
                            messageval_label.set_markup("<span color=\"#FF000000\">********* not found name does not have a file type (ie xx.xxx) **********</span>");
                            bolok = false;
                        } else {
                            let mut lrperpos = strnf.rfind(".").unwrap();
                            if (strnf.len() - lrperpos) < 4 {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* not found name does not have a valid type (must be at least 3 characters **********</span>");
                                bolok = false;
                            } else {
                                let mut lfperpos = strnf.find(".").unwrap();
                                if lfperpos < 3 {
                                    messageval_label.set_markup("<span color=\"#FF000000\">********* not found name is least than 3 characters **********</span>");
                                    bolok = false;
                                } else {
                                    nffullname = format!("{}/{}", str_dirname, strnf);
                                    if Path::new(&nffullname).exists() {
                                        messageval_label.set_markup("<span color=\"#FF000000\">********* not found name already exists **********</span>");
                                        bolok = false;
                                    }
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
          if let Some(filename) = edirectory1_combobox.get_active_text() {
            let str_filename = filename.to_string();
            let str_filenamex = str_filename.clone();
            if Path::new(&str_filename).exists() {
                cdfullname = str_filename;
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* cd file does not exist **********</span>");
                bolok = false;
            }
                
          } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* ERROR GETTING cd FILE IN COMBOBOX **********</span>");
            bolok = false;
          }
        }
        if bolok {         
          if let Some(filename) = edirectory2_combobox.get_active_text() {
            let str_filename = filename.to_string();
            let str_filenamex = str_filename.clone();
            if Path::new(&str_filename).exists() {
                hdfullname = str_filename;
            } else {
                messageval_label.set_markup("<span color=\"#FF000000\">********* hd file does not exist **********</span>");
                bolok = false;
            }
                
          } else {
            messageval_label.set_markup("<span color=\"#FF000000\">********* ERROR GETTING HD FILE IN COMBOBOX **********</span>");
            bolok = false;
          }
        }
        if bolok {
            let hdfile = File::open(hdfullname).unwrap(); 
            let mut hdreader = BufReader::new(hdfile);
            let mut hdline = String::new();
            let mut hdkey = String::new();
            let mut hdlen: i64 = 0;
            let mut hddir = String::new();
            let mut hdname = String::new();
            let mut hddate = String::new();
            match hdreader.read_line(&mut hdline) {
                Ok(bytes_read) => {
                          // EOF: save last file address to restart from this address for next run
                    if bytes_read == 0 {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* error no records in hd file **********</span>");
                        bolok = false;   
                    } else if bytes_read < 540 {
                        messageval_label.set_markup("<span color=\"#FF000000\">********* error record too small in hd file **********</span>");
                        bolok = false;   
                    } else {
                        hdkey = hdline.get(0..512).unwrap().to_string();
         				let len = hdline.get(512..528).unwrap();
//         				println!("length: {}", len);
         			    hdlen = len.parse().unwrap_or(-99);
         			    if hdlen < 0 {
                            messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size in first record of hd file **********</span>");
                            bolok = false;
                        } else {
                            let mut spt = 529;
                            let mut ept = spt + 2;
         				    let rlens = hdline.get(spt..ept).unwrap();
//         				    println!("reference length: {}", rlens);
         			        let rlen: i32 = rlens.parse().unwrap_or(-99);
          			        if rlen < 0 {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of reference name in first record of hd file **********</span>");
                                bolok = false;
                            } else {
                                spt = ept;
                                ept = ept + rlen as usize;
                                let refname = hdline.get(spt..ept).unwrap().to_string();
//                                println!("refname: {}", refname);
                                spt = ept;
                                ept = spt + 3;
          				        let dlens = hdline.get(spt..ept).unwrap();
//            				    println!("directory length: {}", dlens);
         			            let dlen: i32 = dlens.parse().unwrap_or(-99);
          			            if dlen < 0 {
                                    messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of directory name in first record of hd file **********</span>");
                                    bolok = false;
                                } else {
                                    spt = ept;
                                    ept = spt + dlen as usize;
                                    hddir = hdline.get(spt..ept).unwrap().to_string();
//         				            println!("hddir length: {}", hddir);
                                    spt = ept;
                                    ept = spt + 3;
                                    let nlens = hdline.get(spt..ept).unwrap();
         				            let nlen: i32 = nlens.parse().unwrap_or(-99);
          			                if nlen < 0 {
                                        messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of file name in first record of hd file **********</span>");
                                        bolok = false;
                                    } else {
                                        spt = ept;
                                        ept = ept + nlen as usize;
                                        hdname = hdline.get(spt..ept).unwrap().to_string();
//                                        println!("hdname: {}", hdname);
                                        spt = ept;
                                        ept = spt + 2;
         				                let tlens = hdline.get(spt..ept).unwrap();
//         				                println!("date length: {}", tlens);
         			                    let tlen: i32 = tlens.parse().unwrap_or(-99);
          			                    if tlen < 0 {
                                            messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of date in first record of hd file **********</span>");
                                            bolok = false;
                                        } else {
                                            spt = ept;
                                            ept = ept + tlen as usize;
                                            hddate = hdline.get(spt..ept).unwrap().to_string();
//                                            println!("hddate: {}", hddate);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                  messageval_label.set_markup("<span color=\"#FF000000\">********* error hd file first record read **********</span>");
                  bolok = false;   
                }
            };
            let cdfile = File::open(cdfullname).unwrap(); 
            let mut cdreader = BufReader::new(cdfile);
            let mut cdline = String::new();
            let mut cdkey = String::new();
            let mut cdlen: i64 = 0;
            if bolok {
                match cdreader.read_line(&mut cdline) {
                    Ok(bytes_read) => {
                          // EOF: save last file address to restart from this address for next run
                        if bytes_read == 0 {
                            messageval_label.set_markup("<span color=\"#FF000000\">********* error no records in cd file **********</span>");
                            bolok = false;   
                        } else if bytes_read < 540 {
                            messageval_label.set_markup("<span color=\"#FF000000\">********* error record too small in cd file **********</span>");
                            bolok = false;   
                        } else {
                            cdkey = cdline.get(0..512).unwrap().to_string();
         				    let len = cdline.get(512..528).unwrap();
//         				    println!("cd length: /{}/", len);
         			        cdlen = len.parse().unwrap_or(-99);
         			        if cdlen < 0 {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size in first record of cd file **********</span>");
                                bolok = false;
                            }
                        }
                    }
                    Err(err) => {
                      messageval_label.set_markup("<span color=\"#FF000000\">********* error cd file first record read **********</span>");
                      bolok = false;   
                    }
                };
            }
            if bolok {
                let mut samefile = File::create(samefullname).unwrap();
                let mut difffile = File::create(difffullname).unwrap();
                let mut nffile = File::create(nffullname).unwrap();
                let mut bloop = true;
                let mut bnmmast = false;
                let mut bmatch = false;
                while bloop && bolok {
                   if (hdkey < cdkey) || bnmmast {
                       let stroutput = format!("{:03}{}{:03}{}16{:016}{:02}{}",
                                                  hddir.len(),
                                                  hddir,
                                                  hdname.len(),
                                                  hdname,
                                                  hdlen,
                                                  hddate.len(),
                                                  hddate);
                       if bmatch {
                           writeln!(&mut difffile, "{}", stroutput).unwrap();
                           bmatch = false;
                       } else {
                           writeln!(&mut nffile, "{}", stroutput).unwrap();
                       }
                       let mut hdline1 = String::new();
                       match hdreader.read_line(&mut hdline1) {
                          Ok(bytes_read) => {
                          // EOF: save last file address to restart from this address for next run
                             if bytes_read == 0 {
                                 bloop = false;   
                             } else if bytes_read < 540 {
                                 messageval_label.set_markup("<span color=\"#FF000000\">********* error record too small in hd file **********</span>");
                                 bolok = false; 
                          
                             } else {
                                 let hdkey1 = hdline1.get(0..512).unwrap().to_string();
         				         let len1 = hdline1.get(512..528).unwrap();
//         				           println!("length: {}", len);
         			             let hdlen1: i64 = len1.parse().unwrap_or(-99);
         			             if hdlen1 < 0 {
                                     messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size in hd file **********</span>");
                                     bolok = false;
                                 } else {
                                     if hdkey1 < hdkey {
                                         messageval_label.set_markup("<span color=\"#FF000000\">********* hd file is not sorted **********</span>");
                                         bolok = false;
                                     } else {
                                         if (hdkey1 == hdkey) && (hdlen1 < hdlen) {
                                             messageval_label.set_markup("<span color=\"#FF000000\">********* hd file is not sorted **********</span>");
                                             bolok = false;
                                         } else {
                                             hdkey = hdkey1;
                                             hdlen = hdlen1;
                                             let mut spt = 529;
                                             let mut ept = spt + 2;
         				                     let rlens = hdline1.get(spt..ept).unwrap();
//                           				     println!("reference length: {}", rlens);
         			                         let rlen: i32 = rlens.parse().unwrap_or(-99);
          			                         if rlen < 0 {
                                                messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of reference name in hd file **********</span>");
                                                bolok = false;
                                             } else {
                                                spt = ept;
                                                ept = ept + rlen as usize;
                                                let refname = hdline1.get(spt..ept).unwrap().to_string();
//                                                println!("refname: {}", refname);
                                                spt = ept;
                                                ept = spt + 3;
          				                        let dlens = hdline1.get(spt..ept).unwrap();
//            				                    println!("directory length: {}", dlens);
         			                            let dlen: i32 = dlens.parse().unwrap_or(-99);
          			                            if dlen < 0 {
                                                    messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of directory name in hd file **********</span>");
                                                    bolok = false;
                                                } else {
                                                    spt = ept;
                                                    ept = spt + dlen as usize;
                                                    hddir = hdline1.get(spt..ept).unwrap().to_string();
//         				                            println!("hddir length: {}", hddir);
                                                    spt = ept;
                                                    ept = spt + 3;
                                                    let nlens = hdline1.get(spt..ept).unwrap();
         				                            let nlen: i32 = nlens.parse().unwrap_or(-99);
          			                                if nlen < 0 {
                                                        messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of file name in hd file **********</span>");
                                                        bolok = false;
                                                    } else {
                                                        spt = ept;
                                                        ept = ept + nlen as usize;
                                                        hdname = hdline1.get(spt..ept).unwrap().to_string();
                                                        println!("key < hdname: {}", hdname);
                                                        spt = ept;
                                                        ept = spt + 2;
         				                                let tlens = hdline1.get(spt..ept).unwrap();
//         				                                println!("date length: {}", tlens);
         			                                    let tlen: i32 = tlens.parse().unwrap_or(-99);
          			                                    if tlen < 0 {
                                                            messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of date in hd file **********</span>");
                                                            bolok = false;
                                                        } else {
                                                            spt = ept;
                                                            ept = ept + tlen as usize;
                                                            hddate = hdline1.get(spt..ept).unwrap().to_string();
//                                                            println!("hddate: {}", hddate);
                                                        }
                                                    }
                                                }
                                             }
                                         }
                                     }
                                 }
                             }
                          }
                          Err(err) => {
                            messageval_label.set_markup("<span color=\"#FF000000\">********* error hd file read **********</span>");
                            bolok = false;   
                          }
                       };
                   } else if hdkey > cdkey {
                       let mut cdline1 = String::new();
                       match cdreader.read_line(&mut cdline1) {
                          Ok(bytes_read) => {
                          // EOF: save last file address to restart from this address for next run
                             if bytes_read == 0 {
                                 bnmmast = true;   
                             } else if bytes_read < 540 {
                                 messageval_label.set_markup("<span color=\"#FF000000\">********* error record too small in cd file **********</span>");
                                 bolok = false;   
                             } else {
                                 let cdkey1 = cdline1.get(0..512).unwrap().to_string();
         				         let len = cdline1.get(512..528).unwrap();
//         				           println!("cd length: /{}/", len);
         			             let cdlen1: i64 = len.parse().unwrap_or(-99);
         			             if cdlen1 < 0 {
                                     messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size in cd file **********</span>");
                                     bolok = false;
                                 } else {
                                     if cdkey1 < cdkey {
                                         messageval_label.set_markup("<span color=\"#FF000000\">********* cd file is not sorted **********</span>");
                                         bolok = false;
                                     } else {
                                         if (cdkey1 == cdkey) && (cdlen1 < cdlen) {
                                             messageval_label.set_markup("<span color=\"#FF000000\">********* hd file is not sorted **********</span>");
                                             bolok = false;
                                         } else {
                                             cdkey = cdkey1;
                                             cdlen = cdlen1;
                                         }
                                     }
                                 }
                             }
                          }
                          Err(err) => {
                             messageval_label.set_markup("<span color=\"#FF000000\">********* error cd file  **********</span>");
                             bolok = false;   
                          }
                       };
                   } else {
                       if hdlen < cdlen {
                           bmatch = false;
                           let stroutput = format!("{:03}{}{:03}{}16{:016}{:02}{}",
                                                  hddir.len(),
                                                  hddir,
                                                  hdname.len(),
                                                  hdname,
                                                  hdlen,
                                                  hddate.len(),
                                                  hddate);
                           writeln!(&mut difffile, "{}", stroutput).unwrap();
                           let mut hdline2 = String::new();
                           match hdreader.read_line(&mut hdline2) {
                              Ok(bytes_read) => {
                              // EOF: save last file address to restart from this address for next run
                                 if bytes_read == 0 {
                                     bloop = false;   
                                 } else if bytes_read < 540 {
                                     messageval_label.set_markup("<span color=\"#FF000000\">********* error record too small in hd file **********</span>");
                                     bolok = false; 
                          
                                 } else {
                                     let hdkey1 = hdline2.get(0..512).unwrap().to_string();
         				             let len1 = hdline2.get(512..528).unwrap();
//         				               println!("length: {}", len);
         			                 let hdlen1: i64 = len1.parse().unwrap_or(-99);
         			                 if hdlen1 < 0 {
                                         messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size in hd file **********</span>");
                                         bolok = false;
                                     } else {
                                         if hdkey1 < hdkey {
                                             messageval_label.set_markup("<span color=\"#FF000000\">********* hd file is not sorted **********</span>");
                                             bolok = false;
                                         } else {
                                             if (hdkey1 == hdkey) && (hdlen1 < hdlen) {
                                                 messageval_label.set_markup("<span color=\"#FF000000\">********* hd file is not sorted **********</span>");
                                                 bolok = false;
                                             } else {
                                                 hdkey = hdkey1;
                                                 hdlen = hdlen1;
                                                 let mut spt = 529;
                                                 let mut ept = spt + 2;
         				                         let rlens = hdline2.get(spt..ept).unwrap();
//                           		    		     println!("reference length: {}", rlens);
         			                             let rlen: i32 = rlens.parse().unwrap_or(-99);
          			                             if rlen < 0 {
                                                     messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of reference name in hd file **********</span>");
                                                     bolok = false;
                                                 } else {
                                                     spt = ept;
                                                     ept = ept + rlen as usize;
                                                     let refname = hdline2.get(spt..ept).unwrap().to_string();
//                                                    println!("refname: {}", refname);
                                                     spt = ept;
                                                     ept = spt + 3;
          				                             let dlens = hdline2.get(spt..ept).unwrap();
//            				                           println!("directory length: {}", dlens);
         			                                 let dlen: i32 = dlens.parse().unwrap_or(-99);
          			                                 if dlen < 0 {
                                                         messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of directory name in hd file **********</span>");
                                                         bolok = false;
                                                     } else {
                                                         spt = ept;
                                                         ept = spt + dlen as usize;
                                                         hddir = hdline2.get(spt..ept).unwrap().to_string();
//         				                                 println!("hddir length: {}", hddir);
                                                         spt = ept;
                                                         ept = spt + 3;
                                                         let nlens = hdline.get(spt..ept).unwrap();
         				                                 let nlen: i32 = nlens.parse().unwrap_or(-99);
          			                                     if nlen < 0 {
                                                             messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of file name in hd file **********</span>");
                                                             bolok = false;
                                                         } else {
                                                             spt = ept;
                                                             ept = ept + nlen as usize;
                                                             hdname = hdline2.get(spt..ept).unwrap().to_string();
//                                                             println!("hdlen < hdname: {}", hdname);
                                                             spt = ept;
                                                             ept = spt + 2;
         				                                     let tlens = hdline2.get(spt..ept).unwrap();
//         				                                       println!("date length: {}", tlens);
         			                                         let tlen: i32 = tlens.parse().unwrap_or(-99);
          			                                         if tlen < 0 {
                                                                 messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of date in hd file **********</span>");
                                                                 bolok = false;
                                                             } else {
                                                                 spt = ept;
                                                                 ept = ept + tlen as usize;
                                                                 hddate = hdline2.get(spt..ept).unwrap().to_string();
//                                                                 println!("hddate: {}", hddate);
                                                             }
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                              }
                              Err(err) => {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* error hd file read **********</span>");
                                bolok = false;   
                              }
                           };
                       } else if hdlen > cdlen {
                           bmatch = true;
                           let mut cdline2 = String::new();
                           match cdreader.read_line(&mut cdline2) {
                              Ok(bytes_read) => {
                              // EOF: save last file address to restart from this address for next run
                                 if bytes_read == 0 {
                                     bnmmast = true;   
                                 } else if bytes_read < 540 {
                                     messageval_label.set_markup("<span color=\"#FF000000\">********* error record too small in cd file **********</span>");
                                     bolok = false;   
                                 } else {
                                     let cdkey1 = cdline2.get(0..512).unwrap().to_string();
         				             let len = cdline2.get(512..528).unwrap();
//         				               println!("cd length: /{}/", len);
         			                 let cdlen1: i64 = len.parse().unwrap_or(-99);
         			                 if cdlen1 < 0 {
                                         messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size in cd file **********</span>");
                                         bolok = false;
                                     } else {
                                         if cdkey1 < cdkey {
                                             messageval_label.set_markup("<span color=\"#FF000000\">********* cd file is not sorted **********</span>");
                                             bolok = false;
                                         } else {
                                             if (cdkey1 == cdkey) && (cdlen1 < cdlen) {
                                                 messageval_label.set_markup("<span color=\"#FF000000\">********* hd file is not sorted **********</span>");
                                                 bolok = false;
                                             } else {
                                                 cdkey = cdkey1;
                                                 cdlen = cdlen1;
                                             }
                                         }
                                     }
                                 }
                              }
                              Err(err) => {
                                 messageval_label.set_markup("<span color=\"#FF000000\">********* error cd file  **********</span>");
                                 bolok = false;   
                              }
                           };
                       } else {
                           bmatch = false;
                           let stroutput = format!("{:03}{}{:03}{}16{:016}{:02}{}",
                                                  hddir.len(),
                                                  hddir,
                                                  hdname.len(),
                                                  hdname,
                                                  hdlen,
                                                  hddate.len(),
                                                  hddate);
                           writeln!(&mut samefile, "{}", stroutput).unwrap();
                           let mut hdline3 = String::new();
                           match hdreader.read_line(&mut hdline3) {
                              Ok(bytes_read) => {
                              // EOF: save last file address to restart from this address for next run
                                 if bytes_read == 0 {
                                     bloop = false;   
                                 } else if bytes_read < 540 {
                                     messageval_label.set_markup("<span color=\"#FF000000\">********* error record too small in hd file **********</span>");
                                     bolok = false; 
                          
                                 } else {
                                     let hdkey1 = hdline3.get(0..512).unwrap().to_string();
         				             let len1 = hdline3.get(512..528).unwrap();
//         				               println!("length: {}", len);
         			                 let hdlen1: i64 = len1.parse().unwrap_or(-99);
         			                 if hdlen1 < 0 {
                                         messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size in hd file **********</span>");
                                         bolok = false;
                                     } else {
                                         if hdkey1 < hdkey {
                                             messageval_label.set_markup("<span color=\"#FF000000\">********* hd file is not sorted **********</span>");
                                             bolok = false;
                                         } else {
                                             if (hdkey1 == hdkey) && (hdlen1 < hdlen) {
                                                 messageval_label.set_markup("<span color=\"#FF000000\">********* hd file is not sorted **********</span>");
                                                 bolok = false;
                                             } else {
                                                 hdkey = hdkey1;
                                                 hdlen = hdlen1;
                                                 let mut spt = 529;
                                                 let mut ept = spt + 2;
         				                         let rlens = hdline3.get(spt..ept).unwrap();
//                           		    		     println!("reference length: {}", rlens);
         			                             let rlen: i32 = rlens.parse().unwrap_or(-99);
          			                             if rlen < 0 {
                                                     messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of reference name in hd file **********</span>");
                                                     bolok = false;
                                                 } else {
                                                     spt = ept;
                                                     ept = ept + rlen as usize;
                                                     let refname = hdline3.get(spt..ept).unwrap().to_string();
//                                                    println!("refname: {}", refname);
                                                     spt = ept;
                                                     ept = spt + 3;
          				                             let dlens = hdline3.get(spt..ept).unwrap();
//            				                           println!("directory length: {}", dlens);
         			                                 let dlen: i32 = dlens.parse().unwrap_or(-99);
          			                                 if dlen < 0 {
                                                         messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of directory name in hd file **********</span>");
                                                         bolok = false;
                                                     } else {
                                                         spt = ept;
                                                         ept = spt + dlen as usize;
                                                         hddir = hdline3.get(spt..ept).unwrap().to_string();
//         				                                 println!("hddir length: {}", hddir);
                                                         spt = ept;
                                                         ept = spt + 3;
                                                         let nlens = hdline3.get(spt..ept).unwrap();
         				                                 let nlen: i32 = nlens.parse().unwrap_or(-99);
          			                                     if nlen < 0 {
                                                             messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of file name in hd file **********</span>");
                                                             bolok = false;
                                                         } else {
                                                             spt = ept;
                                                             ept = ept + nlen as usize;
                                                             hdname = hdline3.get(spt..ept).unwrap().to_string();
//                                                             println!("hdlen = hdname: {}", hdname);
                                                             spt = ept;
                                                             ept = spt + 2;
         				                                     let tlens = hdline3.get(spt..ept).unwrap();
//         				                                       println!("date length: {}", tlens);
         			                                         let tlen: i32 = tlens.parse().unwrap_or(-99);
          			                                         if tlen < 0 {
                                                                 messageval_label.set_markup("<span color=\"#FF000000\">********* invalid size of date in hd file **********</span>");
                                                                 bolok = false;
                                                             } else {
                                                                 spt = ept;
                                                                 ept = ept + tlen as usize;
                                                                 hddate = hdline3.get(spt..ept).unwrap().to_string();
//                                                                 println!("hddate: {}", hddate);
                                                             }
                                                         }
                                                     }
                                                 }
                                             }
                                         }
                                     }
                                 }
                              }
                              Err(err) => {
                                messageval_label.set_markup("<span color=\"#FF000000\">********* error hd file read **********</span>");
                                bolok = false;   
                              }
                           };
                       }
                   }
                } // end while
                if bolok {
                    messageval_label.set_text("evaluation completed ok");
                }
 
            }
        }

    }));
//----------------- get directory list button start -----------------------------------


//-------------------- connects end
}
