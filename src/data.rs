use crate::course_object::CourseObject;

use std::path::Path;
use calamine::{Xlsx, open_workbook, Reader, RangeDeserializerBuilder, Data, Error};
use gtk4::{gio, prelude::Cast};

pub fn parse_xlsx(path: &Path) -> Result<gio::ListStore, Error> {
    let course_data = gio::ListStore::new::<CourseObject>();
    
    let mut workbook: Xlsx<_> = open_workbook(path)
	.expect("Err: cannot open file");
    let range = workbook.worksheet_range("Sheet1")
	.expect("Err: unable to get range");	
    let mut iter = range.deserialize::<Vec<String>>()
	.expect("Err: unable to create iterator");
    
    while let Some(Ok(row)) = iter.next() {	   
	
	let crn = row.get(0)
	    .unwrap()
		.clone();
	
	let name = row.get(7)
	    .unwrap()
	    .clone();
	
	course_data.append(&CourseObject::new(crn, name));
    }
	
    Ok(course_data)
}
