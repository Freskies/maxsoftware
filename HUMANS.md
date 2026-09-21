# Context for your slaves

Please, if you happen to use slaves to make changes, reference this file for more information on the delicate parts.

## Pages

In `templates/pages` there are three files per page. One for the title, one for the page and one for the content. All
because you can either navigate with URLs or HTMX calls. See `render_page()` for more information.

## render_page

In `routes/pages.rs` there is the function: `render_page()`
I need that function because of HTMX. You can either switch page using urls or clicking a link in the main nav of
the page. That nav uses HTMX. In `render_page()` I check the header of the request to know if I have to return a
fragment or the entire page. 