use askama::Template;

// HOME

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate;

#[derive(Template)]
#[template(path = "pages/home_content.html")]
pub struct HomeContentTemplate;

// SPEEDY

#[derive(Template)]
#[template(path = "pages/speedyweb.html")]
pub struct SpeedyWebTemplate;

#[derive(Template)]
#[template(path = "pages/speedyweb_content.html")]
pub struct SpeedyWebContentTemplate;

// AI SOLUTIONS

#[derive(Template)]
#[template(path = "pages/ai.html")]
pub struct AITemplate;

#[derive(Template)]
#[template(path = "pages/ai_content.html")]
pub struct AIContentTemplate;

// PRIVACY POLICY

#[derive(Template)]
#[template(path = "pages/privacy_policy.html")]
pub struct PrivacyPolicyTemplate;

// NOT FOUND

#[derive(Template)]
#[template(path = "pages/not_found.html")]
pub struct NotFoundTemplate;