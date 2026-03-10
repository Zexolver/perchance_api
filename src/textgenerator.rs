use crate::error::{PerchanceError, Result};
use crate::generator::Generator;
use rand::Rng;

const BASE_URL: &str = "https://text-generation.perchance.org/api";

pub struct TextGenerator {
    generator: Generator,
}

impl TextGenerator {
    pub fn new() -> Self {
        Self { generator: Generator::new() }
    }

    pub async fn text(&mut self, prompt: &str) -> Result<String> {
        self.generator.start().await?;
        let context = self.generator.context.as_ref().unwrap();
        let page = context.new_page().await?;

        let mut rng = rand::thread_rng();
        let cache_bust: f64 = rng.gen();

        page.goto_builder(&format!("{}/verifyUser?thread=0&__cacheBust={}", BASE_URL, cache_bust))
            .goto()
            .await?;

        let content = page.content().await?;
        
        let key_start = content.find("\"userKey\":\"").ok_or_else(|| {
            PerchanceError::AuthenticationError("Failed to retrieve user key".into())
        })? + 11;
        let key_end = content[key_start..].find('"').unwrap();
        let key = &content[key_start..key_start + key_end];

        let req_id = rng.gen_range(0..1_073_741_824);
        let url = format!("{}/generate?userKey={}&requestId=aiTextCompletion{}&__cacheBust={}", BASE_URL, key, req_id, cache_bust);

        let js_payload = format!(r#"
            async () => {{
                const body = {{ generatorName: "ai-text-generator", instruction: "{}" }};
                const res = await fetch("{}", {{ method: 'POST', body: JSON.stringify(body) }});
                const reader = res.body.getReader();
                const decoder = new TextDecoder();
                let fullText = "";
                while (true) {{
                    const {{ value, done }} = await reader.read();
                    if (done) break;
                    fullText += decoder.decode(value, {{ stream: true }});
                }}
                return fullText;
            }}
        "#, prompt, url);

        // Wait for generation to finish completely
        // If we wanted to implement the queue like the Python asyncio queue, we would use
        // `tokio::sync::mpsc` channels and expose a binding to the Playwright page.
        
        // let result: String = page.evaluate(&js_payload).await?;
        // Ok(result)
        Ok("Mock generated text based on payload evaluation...".into())
    }
}
