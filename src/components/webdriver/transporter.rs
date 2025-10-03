use colorful::core::color_string::CString;

pub async fn transporter<'a>(message: &'a CString) {
    println!("{message}");
}
