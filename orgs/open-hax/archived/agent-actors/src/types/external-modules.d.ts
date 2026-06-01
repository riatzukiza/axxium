declare module '@opencode-ai/plugin' {
  export interface PluginInput {
    readonly client?: unknown;
    readonly [key: string]: unknown;
  }

  export type Plugin = (
    pluginContext: PluginInput,
  ) => Promise<{ readonly tool?: Record<string, unknown> } | unknown>;
}

declare module '@opencode-ai/plugin/tool' {
  export const tool: ((config: {
    readonly description: string;
    readonly args: unknown;
    readonly execute: (args: any) => unknown;
  }) => unknown) & {
    readonly schema: any;
  };
}

declare module '@opencode-ai/sdk' {
  export interface OpencodeClient {
    readonly session: {
      readonly create: (args: any) => Promise<{ readonly data?: any; readonly error?: unknown }>;
      readonly prompt: (args: any) => Promise<{ readonly data?: any; readonly error?: unknown }>;
      readonly messages: (args: any) => Promise<{ readonly data?: readonly any[] }>;
      readonly message: (args: any) => Promise<{ readonly data?: any }>;
    };
  }

  export function createOpencodeClient(options: { readonly baseUrl: string }): OpencodeClient;
}

declare module '@promethean-os/persistence' {
  export interface GenericEntry {
    readonly id: string;
    readonly text: string;
    readonly timestamp?: string | number | Date;
    readonly metadata?: Record<string, unknown>;
  }

  export class DualStoreManager<
    TTextKey extends string = string,
    TTimestampKey extends string = string,
  > {
    static create<TTextKey extends string = string, TTimestampKey extends string = string>(
      name: string,
      textKey: TTextKey,
      timestampKey: TTimestampKey,
    ): Promise<DualStoreManager<TTextKey, TTimestampKey>>;

    insert(entry: any): Promise<unknown>;
    get(id: string): Promise<GenericEntry | null>;
    getMostRecent(limit: number): Promise<readonly GenericEntry[]>;
    getMostRelevant(queries: readonly string[], limit: number): Promise<readonly GenericEntry[]>;
  }
}
