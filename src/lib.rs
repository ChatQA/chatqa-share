use serde::{Deserialize, Serialize};
use worker::*;

#[event(fetch, respond_with_errors)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/chat/:id", |_req, ctx| async move {
            println!("{}", markdown::to_html("## Hello, *world*!"));
            if let Some(id) = ctx.param("id") {
                let share = ctx.kv("CHATQA_SHARE")?;
                return match share.get(id).json::<Vec<Message>>().await? {
                    Some(messages) => Response::from_json(&messages),
                    None => Response::error("Not found", 404),
                };
            }
            Response::error("Bad Request", 400)
        })
        .post_async("/chat/:id", |mut req, ctx| async move {
            if let Some(id) = ctx.param("id") {
                let messages = req.json::<Vec<Message>>().await?;
                let share = ctx.kv("CHATQA_SHARE")?;
                share.put(id, messages)?.execute().await?;
                return Response::empty();
            }
            Response::error("Bad Request", 400)
        })
        .get_async("/:id", |_req, ctx| async move {
            if let Some(id) = ctx.param("id") {
                let share = ctx.kv("CHATQA_SHARE")?;
                return match share.get(id).json::<Vec<Message>>().await? {
                    Some(messages) => Response::from_html(replace_template(messages)),
                    None => Response::error("Not found", 404),
                };
            }
            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}

#[derive(Deserialize, Serialize)]
struct Message {
    role: String,
    content: String,
}

fn replace_template(messages: Vec<Message>) -> String {
    let contents: Vec<String> = messages
        .into_iter()
        .map(|message| markdown::to_html(&message.content))
        .collect();

    let all_contents = contents.join("\n");
    page_template(&all_contents)
}

fn page_template(content: &String) -> String {
    format!(
        r##"
    <!DOCTYPE html>
    <html>
      <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <script src="https://cdn.tailwindcss.com?plugins=forms,typography,aspect-ratio,line-clamp"></script>
      </head>
      <body>
        <div class="min-h-screen bg-gray-50 py-8 flex flex-col justify-center relative overflow-hidden lg:py-12">
          <img src="/img/beams.jpg" alt="" class="fixed top-48 left-1/2 -translate-x-2/3 -translate-y-1/2 max-w-none" width="1308" />
          <div class="absolute inset-0 bg-[url(/img/grid.svg)] bg-top [mask-image:linear-gradient(180deg,white,rgba(255,255,255,0))]"></div>
          <div class="relative w-full px-6 py-12 bg-white shadow-xl shadow-slate-700/10 ring-1 ring-gray-900/5 md:max-w-3xl md:mx-auto lg:max-w-4xl lg:pt-16 lg:pb-28">
            <div class="max-w-prose mx-auto lg:text-lg">
            <svg aria-hidden="true" viewBox="0 0 542.05 123.91" class="h-6">
            <path d="M63.55,52.19L1.71,114.01c-2.28,2.28-2.28,5.95,0,8.21,2.28,2.25,5.95,2.28,8.21,0l13.8-13.8h16.46c12.03,0,23.7-3.49,33.65-9.92,2.69-1.74,1.33-5.57-1.89-5.57-1.23,0-2.23-.99-2.23-2.23,0-.99,.65-1.84,1.57-2.13l19.61-5.88c.61-.19,1.16-.51,1.62-.97l5.42-5.42c2.44-2.44,.7-6.61-2.74-6.61h-7.79c-1.23,0-2.23-.99-2.23-2.23,0-.99,.65-1.84,1.57-2.13l27.11-8.13c.97-.29,1.79-.94,2.25-1.86,2.61-5.08,3.97-10.77,3.97-16.61,0-9.92-3.95-19.44-10.97-26.46l-1.33-1.33C100.78,3.95,91.27,0,81.34,0s-19.44,3.95-26.46,10.97l-25.1,25.1c-11.62,11.62-18.15,27.38-18.15,43.81v13.39L57.53,47.39c1.5-1.5,3.97-1.5,5.47,0,1.31,1.31,1.48,3.29,.53,4.79h.02Z" fill="#2563eb" />
            <g>
              <path d="M194.19,91.03c-3.96,1.95-9.13,2.92-15.5,2.92-8.2,0-14.71-2.46-19.53-7.37-4.82-4.91-7.22-11.46-7.22-19.63,0-8.6,2.69-15.64,8.07-21.11,5.38-5.47,12.32-8.21,20.83-8.21,5.32,0,9.77,.69,13.36,2.07v11.21c-3.68-2.16-7.85-3.23-12.52-3.23-5.32,0-9.57,1.7-12.76,5.1-3.19,3.4-4.78,7.84-4.78,13.32s1.51,9.65,4.54,12.9c3.02,3.26,7.11,4.89,12.27,4.89,4.85,0,9.27-1.17,13.25-3.52v10.65Z" />
              <path d="M239.44,93h-11.32v-22.15c0-5.98-2.18-8.96-6.54-8.96-2.18,0-3.98,.83-5.4,2.5-1.42,1.66-2.13,3.82-2.13,6.47v22.15h-11.36V35.45h11.36v24.5h.14c2.98-4.52,6.98-6.79,12.02-6.79,8.81,0,13.22,5.32,13.22,15.96v23.87Z" />
              <path d="M250.12,56.47c1.76-.98,4.11-1.78,7.05-2.39,2.94-.61,5.46-.91,7.54-.91,10.88,0,16.31,5.45,16.31,16.35v23.48h-10.79v-5.66h-.14c-2.65,4.41-6.53,6.61-11.64,6.61-3.68,0-6.63-1.05-8.84-3.15-2.21-2.1-3.32-4.95-3.32-8.56,0-7.45,4.44-11.79,13.32-13.01l10.69-1.44c0-4.52-2.36-6.79-7.07-6.79s-9.01,1.41-13.11,4.22v-8.75Zm12.83,18.91c-4.1,.54-6.15,2.39-6.15,5.55,0,1.45,.5,2.62,1.49,3.52,1,.89,2.34,1.34,4.03,1.34,2.32,0,4.23-.82,5.73-2.46,1.5-1.64,2.25-3.68,2.25-6.12v-2.78l-7.35,.95Z" />
              <path d="M313.65,92.54c-1.71,.94-4.3,1.41-7.77,1.41-8.25,0-12.38-4.34-12.38-13.01v-18.35h-6.4v-8.47h6.4v-8.54l11.29-3.23v11.78h8.86v8.47h-8.86v16.38c0,4.15,1.64,6.22,4.92,6.22,1.27,0,2.58-.37,3.94-1.12v8.47Z" />
              <path d="M318.29,66.49c0-8.53,2.49-15.47,7.47-20.83,4.98-5.36,11.56-8.03,19.74-8.03s14.04,2.59,18.79,7.77c4.75,5.18,7.12,11.84,7.12,19.97s-2.47,15.38-7.42,20.67c-.3,.33-.61,.65-.91,.95l13.85,13.36h-17.19l-7.24-7.38c-2.44,.66-5.06,.98-7.88,.98-7.8,0-14.14-2.56-19.02-7.68-4.88-5.12-7.31-11.71-7.31-19.78Zm12.2-.63c0,5.3,1.28,9.63,3.83,12.99,2.55,3.36,6.06,5.04,10.51,5.04s8.09-1.61,10.62-4.83c2.53-3.22,3.8-7.55,3.8-12.99s-1.23-10.15-3.69-13.45c-2.46-3.29-5.93-4.94-10.41-4.94s-8.13,1.69-10.74,5.06-3.92,7.75-3.92,13.11Z" fill="#2563eb" />
              <path d="M427.03,93h-12.66l-4.08-12.48h-19.79l-4.01,12.48h-12.59l19.93-54.46h13.64l19.55,54.46Zm-19.48-21.45l-6.19-18.88c-.4-1.2-.68-2.78-.84-4.75h-.32c-.14,1.59-.46,3.13-.95,4.61l-6.22,19.02h14.52Z" fill="#2563eb" />
            </g>
            </svg>
            </div>
            <div class="mt-8 prose prose-slate mx-auto lg:prose-lg">{}<p>Generated by <strong>ChatGPT</strong>.</p></div>
          </div>
        </div>
      </body>
    </html>    
    "##,
        content
    )
}
