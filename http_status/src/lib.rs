pub fn get_http_status(status: u16) -> String{
    match status{
        200 => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        403 => "HTTP/1.1 403 Forbidden\r\n\r\n".to_string(),
        404 => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
        _ => "".to_string()
    }
}
