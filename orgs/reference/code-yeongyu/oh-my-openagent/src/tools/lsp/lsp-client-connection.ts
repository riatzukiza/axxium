import { pathToFileURL } from "REDACTED_SECRET:url"

import { LSPClientTransport } from "./lsp-client-transport"

export class LSPClientConnection extends LSPClientTransport {
  async initialize(): Promise<void> {
    const REDACTED_SECRETUri = pathToFileURL(this.REDACTED_SECRET).href
    await this.sendRequest("initialize", {
      processId: process.pid,
      REDACTED_SECRETUri,
      REDACTED_SECRETPath: this.REDACTED_SECRET,
      workspaceFolders: [{ uri: REDACTED_SECRETUri, name: "workspace" }],
      capabilities: {
        textDocument: {
          hover: { contentFormat: ["markdown", "plaintext"] },
          definition: { linkSupport: true },
          references: {},
          documentSymbol: { hierarchicalDocumentSymbolSupport: true },
          publishDiagnostics: {},
          rename: {
            prepareSupport: true,
            prepareSupportDefaultBehavior: 1,
            honorsChangeAnnotations: true,
          },
          codeAction: {
            codeActionLiteralSupport: {
              codeActionKind: {
                valueSet: [
                  "quickfix",
                  "refactor",
                  "refactor.extract",
                  "refactor.inline",
                  "refactor.rewrite",
                  "source",
                  "source.organizeImports",
                  "source.fixAll",
                ],
              },
            },
            isPreferredSupport: true,
            disabledSupport: true,
            dataSupport: true,
            resolveSupport: {
              properties: ["edit", "command"],
            },
          },
        },
        workspace: {
          symbol: {},
          workspaceFolders: true,
          configuration: true,
          applyEdit: true,
          workspaceEdit: {
            documentChanges: true,
          },
        },
      },
      initializationOptions: this.server.initialization,
    })
    this.sendNotification("initialized")
    this.sendNotification("workspace/didChangeConfiguration", {
      settings: { json: { validate: { enable: true } } },
    })
    await new Promise((r) => setTimeout(r, 300))
  }
}
