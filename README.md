# `axum` on Cloudflare Workers

I figured out a way to serve static files, albeit with a few limitations. One of the biggest is that it takes up binary space since it's compiled along with the rest of the code. This is a problem for large files, but it's a good solution for small files. I switched to Tera for templating, and it's been a great experience so far. I'm used to Jinja-like templating, so it's not that bad.
