mod protocol;

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

struct NullclawServer {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for NullclawServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: protocol::server_capabilities(),
            ..Default::default()
        })
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "Nullclaw scanning for safety risks...")
            .await;
        
        let uri = params.text_document.uri;
        
        // Dummy logic to produce a PublishDiagnosticsParams
        let diagnostics = vec![Diagnostic {
            range: Range {
                start: Position { line: 0, character: 0 },
                end: Position { line: 0, character: 5 },
            },
            severity: Some(DiagnosticSeverity::WARNING),
            code: None,
            code_description: None,
            source: Some("nullclaw".to_string()),
            message: "Consider adding type annotations for clarity.".to_string(),
            related_information: None,
            tags: None,
            data: None,
        }];

        self.client.publish_diagnostics(uri, diagnostics, None).await;
    }

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let completions = vec![
            CompletionItem::new_simple("pub".to_string(), "Keyword".to_string()),
            CompletionItem::new_simple("fn".to_string(), "Keyword".to_string()),
            CompletionItem::new_simple("let".to_string(), "Keyword".to_string()),
            CompletionItem::new_simple("mut".to_string(), "Keyword".to_string()),
            CompletionItem::new_simple("println!".to_string(), "Macro".to_string()),
        ];
        
        Ok(Some(CompletionResponse::Array(completions)))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| NullclawServer { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
