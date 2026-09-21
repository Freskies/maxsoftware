use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate;

#[derive(Template)]
#[template(path = "pages/home_content.html")]
pub struct HomeContentTemplate;

#[derive(Template)]
#[template(path = "pages/speedyweb.html")]
pub struct SpeedyWebTemplate;

#[derive(Template)]
#[template(path = "pages/speedyweb_content.html")]
pub struct SpeedyWebContentTemplate;

#[derive(Template)]
#[template(path = "pages/speedyweb.html")]
pub struct CustomizedTemplate;

#[derive(Template)]
#[template(path = "pages/speedyweb_content.html")]
pub struct CustomizeContentTemplate;

#[derive(Template)]
#[template(path = "pages/speedyweb.html")]
pub struct AITemplate;

#[derive(Template)]
#[template(path = "pages/speedyweb_content.html")]
pub struct AIContentTemplate;

#[derive(Template)]
#[template(path = "pages/not_found.html")]
pub struct NotFoundTemplate;